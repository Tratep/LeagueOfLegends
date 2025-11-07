use server::config::config_loader;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let _dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    info!(".ENV LOADED");
}

//todo: next slide 68: https://docs.google.com/presentation/d/1Kwg4lrePjYi_49VfdS7J-zdenaye89Yv-ILbsFv8YOE/edit?slide=id.g38eef0216c6_0_126#slide=id.g38eef0216c6_0_126
