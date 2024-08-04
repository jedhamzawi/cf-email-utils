//! Handler for deleting e-mail routes

use reqwest::header::{HeaderMap, HeaderValue};

use crate::{
    args::{Args, DeleteCmd},
    error::Error,
    models::list::ListResponse,
};

pub(crate) async fn handle(args: &Args, cmd: &DeleteCmd) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let route_ids = if cmd.opts.delete_all {
        list(args, &client)
            .await
            .map_err(|err| Error::new(format!("Error listing routing rules: {err}")))?
    } else {
        cmd.opts.route_ids.clone()
    };
    println!("{route_ids:#?}");
    Ok(())
}

async fn list(args: &Args, client: &reqwest::Client) -> Result<Vec<String>, Error> {
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
    let mut page: usize = 1;
    let mut query = vec![("page", page.to_string()), ("per_page", String::from("50"))];
    let resp = client
        .get(url)
        .headers(headers)
        .query(&query)
        .send()
        .await?;
    let list_resp = resp.json::<ListResponse>().await?;
    if !list_resp.success {
        return Err(Error::new(
            "An unknown error occurred listing e-mail routes",
        ));
    }
    let routes = list_resp
        .result
        .into_iter()
        .flat_map(|rule| rule.id)
        .collect::<Vec<String>>();
    Ok(routes)
}

async fn delete() -> Result<(), Error> {
    Ok(())
}

async fn delete_all() -> Result<(), Error> {
    Ok(())
}

async fn delete_targets() -> Result<(), Error> {
    Ok(())
}
