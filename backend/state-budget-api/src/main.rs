use axum::{
    Json,
    extract::{Extension, Path},
    routing::{get, post},
    Router
}; 
use std::sync::Arc;
use serde::Deserialize;

struct AppState {
    // Add fields here
}

let shared_state = Arc::new(AppState {})

let app = Router::new()
    .route(
        "/users",
        post({
            let shared_state = Arc::clone(&shared_state);
            move |body| create_user(body, shared_state)
        }),
    )
    .route(
        "/users/:id",
        get({
            let shared_state = Arc::clone(&shared_state);
            move |path| get_user(path, shared_state)
        }),
    );

async fn get_user(Path(user_id): Path<String>, state: Arc<AppState>) {
    // get user here
}

async fn create_user(Json(payload): Json<CreateUserPayload>, state: Arc<AppState>){
    // create user here
}

#[derive(Deserialize)]
struct CreateUserPayload {
    // Payload here
}

fn main() {
    println!("Hello, world!");
}
