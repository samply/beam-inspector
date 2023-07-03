use std::{net::SocketAddr, str::FromStr};

use clap::Parser;
use hyper::Uri;

mod serve;

#[derive(Debug, Parser)]
#[clap(name("🌈 Samply.Beam.Inspector"))]
struct Config {

    /// Local bind address
    #[clap(long, env, value_parser, default_value_t = SocketAddr::from_str("0.0.0.0:8081").unwrap())]
    pub bind_addr: SocketAddr,

    /// Beam proxy base url 
    #[clap(long, env, value_parser)]
    pub beam_proxy_url: Uri,

    /// Beam proxy monitoring api key
    #[clap(long, env, value_parser)]
    pub api_key: String,
}



#[tokio::main]
async fn main() {
    let config = Config::parse();
    axum::Server::bind(&config.bind_addr)
        .serve(serve::router(config.beam_proxy_url, config.api_key).into_make_service())
        .await
        .expect("Error starting server");
}
