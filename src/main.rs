mod client;
mod tools;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, StreamableHttpService,
};
use rmcp::model::{Implementation, ProtocolVersion, ServerCapabilities, ServerInfo};
use rmcp::handler::server::ServerHandler;
use rmcp::tool_handler;
use tracing::warn;

use crate::client::BackendClient;
use crate::tools::ToolsServer;

#[tool_handler]
impl ServerHandler for ToolsServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "tools-mcp".to_string(),
                version: "0.1.0".to_string(),
                title: None,
                description: None,
                website_url: None,
                icons: None,
            },
            instructions: Some(
                "Manage short links, landing pages, and message topics on tools.syncapp.tech"
                    .to_string(),
            ),
        }
    }
}

async fn ping() -> &'static str {
    "pong"
}

async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let token = match std::env::var("MCP_AUTH_TOKEN") {
        Ok(t) => t,
        Err(_) => return Ok(next.run(req).await),
    };
    let provided = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if provided != token {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(next.run(req).await)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let backend_client = BackendClient::from_env();

    if std::env::var("MCP_AUTH_TOKEN").is_err() {
        warn!("MCP_AUTH_TOKEN not set - MCP server is unprotected");
    }

    let service = StreamableHttpService::new(
        move || Ok(ToolsServer::new(backend_client.clone())),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let mcp_router = Router::new()
        .nest_service("/mcp", service)
        .layer(middleware::from_fn(auth_middleware));

    let router = Router::new()
        .route("/ping", get(ping))
        .merge(mcp_router);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    tracing::info!("MCP server listening on {}", bind_address);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
