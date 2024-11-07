use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::info;

struct AppState {
    directory: PathBuf,
}

pub async fn http_serve(directory: PathBuf, port: u16) -> anyhow::Result<()> {
    let socket = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(socket).await?;
    info!("http serve {:?} on {}", directory, socket);
    let serve_dir = ServeDir::new(directory.clone());
    let state = AppState { directory };
    // build our application with a single route
    let app = Router::new()
        .nest_service("/tower", serve_dir)
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    axum::serve(listener, app).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let path = state.directory.join(path);
    info!("{}", format!("current path : {:?}", path));

    if !path.exists() {
        return (StatusCode::NOT_FOUND, "File not found".into());
    }

    if path.is_file() {
        match tokio::fs::read_to_string(path).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)),
        }
    } else {
        let mut response = String::new();
        let mut dir = tokio::fs::read_dir(&path).await.unwrap();
        while let Ok(Some(entry)) = dir.next_entry().await {
            response.push_str(&format!(
                r#"
                <li><a href="{}">{}</a></li>
                "#,
                entry.path().display(),
                entry.path().display()
            ));
        }
        (StatusCode::OK, format!("<ul>{}</ul>", response))
    }
}
