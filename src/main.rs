use testcaseFTP::{configuration, state::startup};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those
                // events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let configuration = configuration::get_configuration().expect("Failed to load configuration");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", configuration.ftp.port))
        .await
        .unwrap();
    let app = startup().await;
    axum::serve(listener, app).await.unwrap();
}
