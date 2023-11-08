use std::net::SocketAddr;
use std::time::Duration;

use axum::{error_handling::*, routing::*};
use axum::{Router, Server};
use setup::{Args, Cfg, State};
use tower_http::ServiceBuilderExt;
use tower_http::{cors::*, request_id::*, trace::*};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod endpoint;
mod services;
mod setup;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let cfg = Cfg::parse();

    // Tracing.
    let env = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "gateway=trace,tower_http=trace".into());
    let fmt = tracing_subscriber::fmt::layer().pretty().with_target(true);
    tracing_subscriber::registry().with(fmt).with(env).init();

    // Services.
    let state = State::new(&args, &cfg).await?;

    // Middlewares.
    let error_layer = HandleErrorLayer::new(endpoint::handle_box_error);
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_response(DefaultOnResponse::new().include_headers(true));
    let cors_layer = CorsLayer::permissive();

    let middlewares = tower::ServiceBuilder::default()
        .layer(cors_layer)
        .layer(error_layer)
        .timeout(Duration::from_secs(300))
        .compression()
        // Strict order: Set > Trace > Propagate.
        .set_x_request_id(MakeRequestUuid)
        .layer(tracing_layer)
        .propagate_x_request_id();

    // Endpoints.
    let app = Router::default()
        .route(
            "/",
            get(|| async { "You have hit the backend congratulations" }),
        )
        .route("/scrape_url", post(endpoint::scrape_url))
        .layer(middlewares)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    println!("🚀 Tony Spread them cheeks @: http://{}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

// let num = 69;
// let link = vec!["BOOTYCHEECKS1".to_string(), "BOOTYCHEEKS2".to_string()];
// let discovery_boi = Discover::new(num,link);
// discovery_boi.crawl();

// let cors = CorsLayer::new()
//     .allow_origin(Any)
//     .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
//     .allow_credentials(true)
//     .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
