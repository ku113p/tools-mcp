use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Content};
use rmcp::schemars;
use rmcp::{tool, tool_router, ErrorData};
use serde::Deserialize;

use crate::client::BackendClient;

#[derive(Clone)]
pub struct ToolsServer {
    client: BackendClient,
    pub tool_router: ToolRouter<Self>,
}

impl ToolsServer {
    pub fn new(client: BackendClient) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }
}

#[derive(Deserialize, schemars::JsonSchema, Default)]
pub struct CreateShortLinkArgs {
    #[schemars(description = "The URL to shorten")]
    url: String,
    #[schemars(description = "Optional human-readable name for this link")]
    name: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema, Default)]
pub struct CreateLandingPageArgs {
    #[schemars(description = "URL path for the page (e.g. 'about')")]
    path: String,
    #[schemars(description = "HTML content of the page")]
    html: String,
    #[schemars(description = "Optional human-readable name for this page")]
    name: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema, Default)]
pub struct GetLandingPageArgs {
    #[schemars(description = "URL path of the page to retrieve")]
    path: String,
}

#[derive(Deserialize, schemars::JsonSchema, Default)]
pub struct CreateTopicArgs {
    #[schemars(description = "Topic name")]
    name: String,
    #[schemars(description = "Telegram bot API key for notifications")]
    tg_api_key: Option<String>,
    #[schemars(description = "Telegram chat ID for notifications")]
    tg_chat_id: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema, Default)]
pub struct ListMessagesArgs {
    #[schemars(description = "UUID of the topic")]
    topic_id: String,
}

fn result_to_call_tool(result: anyhow::Result<serde_json::Value>) -> Result<CallToolResult, ErrorData> {
    match result {
        Ok(val) => Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&val).unwrap_or_default(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[tool_router]
impl ToolsServer {
    #[tool(description = "Create a short link that redirects to the given URL")]
    async fn create_short_link(
        &self,
        Parameters(args): Parameters<CreateShortLinkArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let url = format!("{}/generate", self.client.short_links_url);
        let mut body = serde_json::json!({ "url": args.url });
        if let Some(name) = args.name {
            body["name"] = serde_json::Value::String(name);
        }
        let result = self.client.post_json(&url, &self.client.short_links_token, &body).await;
        result_to_call_tool(result)
    }

    #[tool(description = "List all short links with their URLs, names, and click counts")]
    async fn list_short_links(&self) -> Result<CallToolResult, ErrorData> {
        let url = format!("{}/links", self.client.short_links_url);
        let result = self.client.get(&url, &self.client.short_links_token).await;
        result_to_call_tool(result)
    }

    #[tool(description = "Create a landing page with custom HTML at the given path")]
    async fn create_landing_page(
        &self,
        Parameters(args): Parameters<CreateLandingPageArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut create_url = format!(
            "{}/create_page/{}",
            self.client.landing_pages_url,
            urlencoding::encode(&args.path)
        );
        if let Some(name) = &args.name {
            create_url = format!("{}?name={}", create_url, urlencoding::encode(name));
        }
        let result = self
            .client
            .post_body(&create_url, &self.client.landing_pages_token, args.html, "text/html")
            .await;
        result_to_call_tool(result)
    }

    #[tool(description = "List all landing pages with their paths and names")]
    async fn list_landing_pages(&self) -> Result<CallToolResult, ErrorData> {
        let url = format!("{}/pages", self.client.landing_pages_url);
        let result = self.client.get(&url, &self.client.landing_pages_token).await;
        result_to_call_tool(result)
    }

    #[tool(description = "Get the HTML content of a landing page")]
    async fn get_landing_page(
        &self,
        Parameters(args): Parameters<GetLandingPageArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let url = format!(
            "{}/p/{}",
            self.client.landing_pages_url,
            urlencoding::encode(&args.path)
        );
        let result = self.client.get(&url, &self.client.landing_pages_token).await;
        result_to_call_tool(result)
    }

    #[tool(description = "Create a new message topic (contact form endpoint)")]
    async fn create_topic(
        &self,
        Parameters(args): Parameters<CreateTopicArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let url = format!("{}/topics", self.client.message_url);
        let mut body = serde_json::json!({ "name": args.name });
        if let (Some(api_key), Some(chat_id)) = (args.tg_api_key, args.tg_chat_id) {
            body["tg_api"] = serde_json::json!({
                "api_key": api_key,
                "chat_id": chat_id,
            });
        }
        let result = self.client.post_json(&url, &self.client.message_token, &body).await;
        result_to_call_tool(result)
    }

    #[tool(description = "List all message topics")]
    async fn list_topics(&self) -> Result<CallToolResult, ErrorData> {
        let url = format!("{}/topics", self.client.message_url);
        let result = self.client.get(&url, &self.client.message_token).await;
        result_to_call_tool(result)
    }

    #[tool(description = "List messages for a specific topic")]
    async fn list_messages(
        &self,
        Parameters(args): Parameters<ListMessagesArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let url = format!(
            "{}/topics/{}/messages",
            self.client.message_url,
            urlencoding::encode(&args.topic_id)
        );
        let result = self.client.get(&url, &self.client.message_token).await;
        result_to_call_tool(result)
    }
}
