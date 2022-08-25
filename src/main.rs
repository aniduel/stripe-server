mod config;
mod models;
mod routes;

use std::{fs, net::SocketAddr};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Extension, Router, Server,
};
use redis::AsyncCommands;
use tokio::sync::broadcast;
use tracing::{info, debug, warn};

use config::Config;
use routes::webhook_post;

pub struct Error(anyhow::Error);
impl<E: Into<anyhow::Error>> From<E> for Error {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub type StripeResult<T, E = Error> = Result<T, E>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let description = self.0.to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, description).into_response()
    }
}

#[derive(Clone)]
pub struct State(broadcast::Sender<String>);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let conf: Config = toml::from_str(&fs::read_to_string("config.toml")?)?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let redis_uri = format!(
        "redis://{}:{}@{}:{}",
        conf.redis.username, conf.redis.password, conf.redis.host, conf.redis.port
    );
    let redis = redis::Client::open(redis_uri)?;
    let mut conn = redis.get_tokio_connection().await?;

    let (tx, mut rx) = broadcast::channel(100);

    let state = State(tx);

    tokio::spawn(async move {
        loop {
            let message = rx.recv().await;

            if let Ok(message) = message {
                debug!("Sending message: {message}");
                let rslt: Result<redis::Value, redis::RedisError> =
                    conn.publish("stripe:1", message).await;
                if let Err(rslt) = rslt {
                    warn!("Error sending ipc message {}", rslt);
                }
            }
        }
    });

    let router = Router::new()
        .route("/stripe", post(webhook_post))
        .layer(Extension(state));

    info!("Serving...");
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
