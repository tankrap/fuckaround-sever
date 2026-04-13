use axum::middleware::from_fn_with_state;
use axum::routing::{delete, patch, put};
use axum::{
    Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use axum_client_ip::ClientIpSource;
use hyper::{Method, StatusCode};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::app::App;

pub async fn generate_routers(app: App) -> Router
{
    macro_rules! must_be_logged_in {
        ($resource:expr) => {
            $resource.layer(from_fn_with_state(app.clone(), must_have_session))
        };
    }
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);
    return Router::new()
        .with_state(app)
        .layer(cors)
        .layer(ClientIpSource::RightmostXForwardedFor.into_extension());
}
