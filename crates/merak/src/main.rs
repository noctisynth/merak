use std::net::Ipv4Addr;
use std::{env, sync::Arc};

use axum::http::StatusCode;
use axum::routing::get;
use serde::Serialize;
use surrealdb::opt::auth::Root;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_redoc::{Redoc, Servable};

use merak::BusinessCodes;
use merak::common::code::{CommonCode, register_all_codes};
use merak::common::response::{ApiResponse, ErrorResponse};
use merak::routes::auth;
use merak::services::auth::AuthService;

#[derive(ToSchema, Serialize)]
struct HelloResponse {
    message: String,
}

#[utoipa::path(get, path = "/hello", operation_id = "hello", responses(
    (status = 200, description = "Successful response", body = ApiResponse<HelloResponse>),
))]
async fn hello() -> axum::Json<ApiResponse<HelloResponse>> {
    axum::Json(ApiResponse::ok(HelloResponse {
        message: "Hello, World!".to_string(),
    }))
}

async fn not_found() -> (StatusCode, axum::Json<ErrorResponse>) {
    (
        StatusCode::NOT_FOUND,
        axum::Json(ErrorResponse::new(CommonCode::NotFound, "Not Found")),
    )
}

#[derive(OpenApi)]
#[openapi(
    paths(hello),
    components(schemas(BusinessCodes)),
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

    // Prepare connection info
    let url = env::var("SURREAL_URL").unwrap_or("ws://127.0.0.1:5070".to_string());
    let ns = env::var("SURREAL_NS").unwrap_or("test".to_string());
    let db_name = env::var("SURREAL_DB").unwrap_or("test".to_string());

    // Prepare credentials as Root
    let creds = Root {
        username: &env::var("SURREAL_USER").unwrap_or("root".to_string()),
        password: &env::var("SURREAL_PASS").unwrap_or("root".to_string()),
    };

    // Connect to SurrealDB (Any)
    let db = surrealdb::engine::any::connect(&url).await?;
    db.use_ns(&ns).use_db(&db_name).await?;
    db.signin(creds).await?;
    let state = Arc::new(db);

    // Create auth state
    let auth_state = auth::AuthState {
        db: state.clone(),
        auth_service: Arc::new(AuthService::from_env()),
    };

    // Build openapi + base router
    let (router, mut api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(hello))
        .with_state(state)
        .nest("/auth", auth::routes().with_state(auth_state))
        .fallback(not_found)
        .split_for_parts();

    register_all_codes(&mut api);

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
