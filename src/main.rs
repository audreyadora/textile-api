use axum::{
    http::StatusCode, 
    routing::get, 
    Router
};

async fn index() -> String {
    String::from("hello!!!") 
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
   
}