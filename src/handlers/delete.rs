//! Handler for deleting e-mail routes

use futures::{stream, StreamExt, TryStreamExt};
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashSet;

use crate::{
    args::{Args, DeleteCmd},
    error::Error,
    models::{list::ListResponse, RouteMatcherType},
};

pub(crate) async fn handle(args: &Args, cmd: &DeleteCmd) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let route_ids = if cmd.opts.delete_all {
        list(args, &client)
            .await
            .map_err(|err| Error::new(format!("Error listing routing rules: {err}")))?
    } else {
        cmd.opts.route_ids.iter().cloned().collect()
    };
    delete(route_ids, args, &client)
        .await
        .map_err(|err| Error::new(format!("Error deleting routing rules: {err}")))?;
    Ok(())
}

async fn list(args: &Args, client: &reqwest::Client) -> Result<HashSet<String>, Error> {
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
    let mut routes = HashSet::new();
    loop {
        let resp = client
            .get(&url)
            .headers(headers.clone())
            .query(&query)
            .send()
            .await?;
        let list_resp = resp.json::<ListResponse>().await?;
        if !list_resp.success {
            return Err(Error::new(
                "An unknown error occurred listing e-mail routes",
            ));
        }
        if list_resp.result.is_empty() {
            break;
        }
        routes.extend(list_resp.result.into_iter().filter_map(|rule| {
            let catch_all = rule
                .matchers
                .first()
                .is_some_and(|matcher| matcher.r#type == RouteMatcherType::All);
            (!catch_all).then_some(rule.id).flatten()
        }));
        page += 1;
        query[0].1 = page.to_string();
    }
    Ok(routes)
}

async fn delete(
    route_ids: HashSet<String>,
    args: &Args,
    client: &reqwest::Client,
) -> Result<(), Error> {
    let base_url = format!(
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
    stream::iter(route_ids)
        .map(Ok)
        .try_for_each_concurrent(10, |id| {
            let url = format!("{base_url}/{id}");
            let headers = headers.clone();
            async move {
                let resp = client.delete(&url).headers(headers).send().await?;
                println!("{}: {}", id, resp.status());
                resp.error_for_status().map(|_| ()).map_err(Error::from)
            }
        })
        .await
}
