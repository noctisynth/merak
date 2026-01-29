use std::net::Ipv4Addr;
use std::{env, sync::Arc};

use axum::http::StatusCode;
use axum::routing::get;
use serde::Serialize;
use surrealdb::engine::any;
use surrealdb::opt::auth::Root;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_redoc::{Redoc, Servable};

use merak::auth::service::AuthService;
use merak::routes::auth;

#[derive(ToSchema, Serialize)]
struct HelloResponse {
    message: String,
}

#[utoipa::path(method(get, head), path = "/hello", operation_id = "hello", responses(
    (status = 200, description = "Successful response", body = HelloResponse),
))]
async fn hello() -> axum::Json<HelloResponse> {
    axum::Json(HelloResponse {
        message: "Hello, World!".to_string(),
    })
}

async fn not_found() -> (StatusCode, axum::Json<HelloResponse>) {
    (
        StatusCode::NOT_FOUND,
        axum::Json(HelloResponse {
            message: "Not Found".to_string(),
        }),
    )
}

#[derive(OpenApi)]
#[openapi(
    paths(hello),
    tags(
        (name = "Authentication", description = "Authentication endpoints"),
    ),
    info(
        title = "Merak API",
        version = "0.1.0",
        description = "Merak API Documentation",
        license(name = "AGPL-3.0", url = "https://www.gnu.org/licenses/agpl-3.0.html"),
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env if present and read SurrealDB connection info
    let _ = dotenv::dotenv();

    // url 一定要携带协议名称
    let url = env::var("SURREAL_URL").unwrap_or("ws://127.0.0.1:5070".to_string());
    let ns = env::var("SURREAL_NS").unwrap_or("test".to_string());
    let db_name = env::var("SURREAL_DB").unwrap_or("test".to_string());

    // Prepare credentials as Root
    let creds = Root {
        username: &env::var("SURREAL_USER").unwrap_or("root".to_string()),
        password: &env::var("SURREAL_PASS").unwrap_or("root".to_string()),
    };

    // Connect to SurrealDB (Any)
    // 我认为作为一个框架，可以封装一下db backend 的连接，这样可以更方便的切换不同的数据库？或是作为一个feature 实现不同的DB-manager-engine?
    let db = any::connect(&url).await?;
    db.use_ns(&ns).use_db(&db_name).await?;
    db.signin(creds).await?;
    let state = Arc::new(db);

    // Create auth state
    let auth_state = auth::AuthState {
        db: state.clone(),
        auth_service: Arc::new(AuthService::from_env()),
    };

    // Build openapi + base router
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(hello))
        .with_state(state)
        .nest("/auth", auth::routes().with_state(auth_state))
        .fallback(not_found)
        .split_for_parts();

    // Redoc UI
    let router = router
        .merge(Redoc::with_url("/redoc", api.clone()))
        .route("/apidoc/openapi.json", get(async move || axum::Json(api)));

    // Start server
    let listener = tokio::net::TcpListener::bind((Ipv4Addr::UNSPECIFIED, 8080)).await?;
    println!("Serving on http://127.0.0.1:8080...");
    println!("OpenAPI JSON available at http://127.0.0.1:8080/apidoc/openapi.json");
    println!("Redoc UI available at http://127.0.0.1:8080/redoc");
    axum::serve(listener, router).await?;
    Ok(())
}
