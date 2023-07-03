mod serve;

#[tokio::main]
async fn main() {
    axum::Server::bind(&"127.0.0.1:1337".parse().unwrap()).serve(serve::router("http://localhost:8081", "Maintenance".into()).into_make_service()).await.unwrap();
}
