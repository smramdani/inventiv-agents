use axum::{
    body::Body,
    http::{header::HeaderName, Request},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

const TRACE_ID_HEADER: &str = "X-Trace-ID";

pub async fn trace_id_middleware(mut request: Request<Body>, next: Next) -> Response {
    // 1. Extract or Generate TraceID
    let trace_id = request
        .headers()
        .get(TRACE_ID_HEADER)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
        .unwrap_or_else(Uuid::new_v4);

    // 2. Insert into extensions for access in handlers
    request.extensions_mut().insert(TraceID(trace_id));

    // 3. Process the request
    let mut response = next.run(request).await;

    // 4. Set the header in the response for frontend correlation
    if let Ok(value) = trace_id.to_string().parse() {
        response
            .headers_mut()
            .insert(HeaderName::from_static("x-trace-id"), value);
    }

    response
}

#[derive(Clone, Copy, Debug)]
pub struct TraceID(pub Uuid);
