use axum::{
    routing::get,
    Router, response::Html
};

mod api;
mod data;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/countries", get(api::all))
        .route("/api/countries/:cca3", get(api::by_cca3))


        .route("/api/name/:name", get(api::name))
        .route("/api/capital/:name", get(api::capital))
        .route("/api/language/:language", get(api::language))
        .route("/api/currency/:currency", get(api::currency))
        .route("/api/callingcode/:calling_code", get(api::callingcode))

        .route("/", get(index));

    let env_port = std::env::var("PORT").map(|port| port.parse::<u16>().unwrap());
    let port = env_port.unwrap_or(8080);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


// basic handler that responds with a static string
async fn index() -> Html<String> {
    let html = tokio::fs::read_to_string("resource/index.html").await.unwrap();
    Html(html)
}
