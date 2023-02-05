use std::time::Duration;
use acunetix_sdk::apis::configuration::{ApiKey, Configuration};
use acunetix_sdk::apis::default_api as acunetix_api;
use reqwest::redirect::Policy;
use acunetix_cli::cli::TopLevel;

#[tokio::main]
async fn main() {
    let toplevel: TopLevel = argh::from_env();
    println!("{:?}",toplevel);
    let mut api_config = Configuration::default();
    let api_key = ApiKey {
        prefix: None,
        key: String::from("1986ad8c0a5b3df4d7028d5f3c06e936cafd94cb0191d430f8a57416536ddd416"),
    };
    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(0)
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .redirect(Policy::none())
        .timeout(Duration::new(30, 0)).build().unwrap();
    api_config.api_key = Some(api_key);
    api_config.client = client;
    api_config.base_path = String::from("https://10.168.1.201:13443/api/v1");
    let me = acunetix_api::get_info(&api_config).await;
    println!("{:?}", me);
    println!("Hello, world!");
}
