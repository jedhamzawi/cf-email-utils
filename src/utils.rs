//! Some useful functions/macros for use across the crate

const EMAIL_ROUTING_URL: &str = ;

#[macro_export]
macro_rules! build_req {
    ($args:expr, $body:expr) => {
        let url = format!(
"https://api.cloudflare.com/client/v4/zones/{}/email/routing/rules", $args.zone_identifier);

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
    
};
}

