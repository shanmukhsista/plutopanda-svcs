use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


pub fn init_tracing_subscriber(){
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

}