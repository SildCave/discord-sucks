mod prometheus;

use axum::{
    routing::get,
    Router
};
use futures::future::ready;
use prometheus::setup_metrics_recorder;
pub use prometheus::track_metrics;
use tower_http::{
    classify::{
        ServerErrorsAsFailures,
        SharedClassifier
    },
    trace::TraceLayer
};

pub fn metrics_app(
    trace_layer: TraceLayer<SharedClassifier<ServerErrorsAsFailures>>
) -> Router {
    let recorder_handle = setup_metrics_recorder();
    Router::new()
        .route("/metrics", get(move || ready(recorder_handle.render())))
        .route("/health", get(|| async { "OK" }))
        .layer(trace_layer)
}