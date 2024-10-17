use testcaseFTP::{configuration, state::startup};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let configuration = configuration::get_configuration().expect("Failed to load configuration");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", configuration.ftp.port))
        .await
        .unwrap();
    let app = startup().await;
    axum::serve(listener, app).await.unwrap();
}
