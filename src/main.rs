use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use axum::extract::{FromRef, State};
use axum::http::Uri;
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let cache: Cache = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new().route("/test", get(test))
        .with_state(AppState {
            cache
        });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

type Cache = Arc<Mutex<HashMap<String, AtomicU64>>>;

#[derive(Clone)]
struct AppState {
    cache: Cache,
}

impl FromRef<AppState> for Cache {
    fn from_ref(v: &AppState) -> Self {
        v.cache.clone()
    }
}

async fn test(State(cache): State<Cache>, uri: Uri) -> &'static str {
    count(cache, uri.path()).await;
    "OK"
}

async fn count(cache: Cache, key: &str) {
    let mut cache = cache.lock().unwrap();
    match cache.get_mut(key) {
        None => {
            // TODO 加锁之后，不能使用 async 函数
            // let x = xxx().await;
            cache.insert(key.to_string(), AtomicU64::new(1));
        }
        Some(v) => {
            v.fetch_add(1, Ordering::Relaxed);
        }
    }
}

async fn xxx() -> u32 {
    return 1;
}