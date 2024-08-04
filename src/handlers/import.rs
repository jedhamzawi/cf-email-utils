//! Handler for importing e-mail aliases as Cloudflare routes

use std::{collections::HashMap, sync::Arc};

use csv::StringRecord;
use futures::stream::{self, StreamExt, TryStreamExt};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;

use crate::{
    args::{ExportFormat, ExportSourceCommand, ImportCmd},
    Args, Error,
};

const BW_USERNAME_CSV_HEADER: &str = "login_username";
const BW_FIELDS_CSV_HEADER: &str = "fields";
const SL_ALIAS_CSV_HEADER: &str = "alias";
const SL_ENABLED_CSV_HEADER: &str = "enabled";
const SL_NOTE_CSV_HEADER: &str = "note";

pub(crate) async fn handle(args: &Args, cmd: &ImportCmd) -> Result<(), Error> {
    let aliases = match &cmd.export_cmd {
        crate::args::ExportSourceCommand::SimpleLogin => get_aliases_sl(cmd)
            .map_err(|err| Error::new(format!("Faild to parse aliases from Bitwarden: {err}")))?,
        crate::args::ExportSourceCommand::Bitwarden(bw_args) => {
            get_aliases_bw(cmd, &bw_args.format).map_err(|err| {
                Error::new(format!("Faild to parse aliases from Bitwarden: {err}"))
            })?
        }
    };
    import_aliases(aliases, &args, cmd)
        .await
        .map_err(|err| Error::new(format!("Failed to create aliases in Cloudflare! {err}")))?;
    Ok(())
}

fn get_header_index(headers: &StringRecord, header_name: &str) -> Result<usize, Error> {
    headers
        .iter()
        .position(|header| header == header_name)
        .ok_or(Error::new("Export CSV is missing '{header_name}' header!"))
}

fn get_aliases_sl(cmd: &ImportCmd) -> Result<HashMap<String, String>, Error> {
    let export_file = std::fs::File::open(&cmd.export_path)?;
    let mut csv_reader = csv::Reader::from_reader(&export_file);
    let headers = csv_reader
        .headers()
        .map_err(|_| Error::new("Export CSV has no headers!"))?;
    let alias_index = get_header_index(&headers, SL_ALIAS_CSV_HEADER)?;
    let enabled_index = get_header_index(&headers, SL_ENABLED_CSV_HEADER)?;
    let note_index = get_header_index(&headers, SL_NOTE_CSV_HEADER)?;
    let aliases = csv_reader
        .into_records()
        .collect::<Result<Vec<csv::StringRecord>, csv::Error>>()
        .map_err(|err| Error::new(format!("Malformed export CSV: {err}")))?
        .into_iter()
        .filter_map(|record| {
            record
                .get(enabled_index)
                .is_some_and(|enabled| enabled.to_lowercase() == "true")
                .then_some(record.get(alias_index).map(|address| {
                    (
                        remove_domain(address).to_string(),
                        record.get(note_index).unwrap_or_default().to_string(),
                    )
                }))
                .flatten()
        })
        .collect();
    Ok(aliases)
}

fn remove_domain(address: &str) -> String {
    match address.find('@') {
        Some(index) => address[..index].to_string(),
        None => address.to_string(),
    }
}

fn get_aliases_bw(
    cmd: &ImportCmd,
    format: &ExportFormat,
) -> Result<HashMap<String, String>, Error> {
    let export_file = std::fs::File::open(&cmd.export_path)?;
    let mut aliases = HashMap::new();
    let email_domain = if cmd.domain.starts_with('@') {
        cmd.domain.to_string()
    } else {
        format!("@{}", cmd.domain)
    };
    match format {
        ExportFormat::Csv => {
            let mut csv_reader = csv::Reader::from_reader(&export_file);
            let headers = csv_reader
                .headers()
                .map_err(|_| Error::new("Export CSV has no headers!"))?;
            let login_index = get_header_index(&headers, BW_USERNAME_CSV_HEADER)?;
            let fields_index = get_header_index(&headers, BW_FIELDS_CSV_HEADER)?;
            for record in csv_reader
                .into_records()
                .collect::<Result<Vec<csv::StringRecord>, csv::Error>>()
                .map_err(|err| Error::new(format!("Malformed export CSV: {err}")))?
            {
                if let Some(username) = record.get(login_index) {
                    let username = username.trim();
                    if username.ends_with(&email_domain) {
                        let alias = username.trim_end_matches(&email_domain);
                        aliases.insert(alias.to_string(), format!("Used for {alias}"));
                    }
                }
                if let Some(fields) = record.get(fields_index) {
                    aliases.extend(fields.split(' ').filter_map(|field| {
                        let field = field.trim();
                        field.ends_with(&email_domain).then_some({
                            let alias = field.trim_end_matches(&email_domain);
                            (alias.to_string(), format!("Used for {alias}"))
                        })
                    }));
                }
            }
        }
        ExportFormat::Json => return Err(Error::new("JSON is not current supported!")),
    }
    Ok(aliases)
}

async fn import_aliases(
    aliases: HashMap<String, String>,
    args: &Args,
    cmd: &ImportCmd,
) -> Result<(), Error> {
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/email/routing/rules",
        args.zone_identifier
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("application/json")
            .expect("application/json is not an allowed header value"),
    );
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", args.cf_api_key))
            .expect("Invalid Cloudflare API key"),
    );
    let domain = if cmd.domain.starts_with('@') {
        cmd.domain.chars().skip(1).into_iter().collect()
    } else {
        cmd.domain.clone()
    };
    let imported_from = match &cmd.export_cmd {
        ExportSourceCommand::SimpleLogin => "Imported from SimpleLogin",
        ExportSourceCommand::Bitwarden(_) => "Imported from Bitwarden",
    };
    let client = Arc::new(reqwest::Client::new());
    stream::iter(aliases.into_iter())
        .map(Ok)
        .try_for_each_concurrent(25, |(alias, note)| {
            let client = client.clone();
            let headers = headers.clone();
            let url = &url;
            let destination_address = &cmd.destination_address;
            let alias_address = format!("{alias}@{domain}");
            let imported_from = &imported_from;
            let body = json!({
                "actions": [
                    {
                      "type": "forward",
                      "value": [
                        destination_address
                      ]
                    }
                  ],
              "enabled": true,
              "matchers": [
                {
                  "field": "to",
                  "type": "literal",
                  "value": &alias_address
                }
              ],
              "name": format!("{note}\n\n{imported_from}"),
              "priority": 0
            });
            async move {
                let res = client.post(url).headers(headers).json(&body).send().await;
                match res {
                    Ok(resp) => println!("{}: {}", alias_address, resp.status()),
                    Err(err) => {
                        println!("{}: {}", alias_address, err);
                        // ignore if it's a conflict error
                        if !err.status().is_some_and(|code| code == 409) {
                            return Err(err.into());
                        }
                    }
                }
                Ok(())
            }
        })
        .await
}
