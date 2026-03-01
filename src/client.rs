use reqwest::Client;
use serde_json::Value;

#[derive(Clone)]
pub struct BackendClient {
    http: Client,
    pub short_links_url: String,
    pub short_links_token: Option<String>,
    pub landing_pages_url: String,
    pub landing_pages_token: Option<String>,
    pub message_url: String,
    pub message_token: Option<String>,
}

impl BackendClient {
    pub fn from_env() -> Self {
        Self {
            http: Client::new(),
            short_links_url: std::env::var("SHORT_LINKS_URL")
                .unwrap_or_else(|_| "http://short-links-app:3000".to_string()),
            short_links_token: std::env::var("SHORT_LINKS_TOKEN").ok(),
            landing_pages_url: std::env::var("LANDING_PAGES_URL")
                .unwrap_or_else(|_| "http://landing-pages-app:3000".to_string()),
            landing_pages_token: std::env::var("LANDING_PAGES_TOKEN").ok(),
            message_url: std::env::var("MESSAGE_URL")
                .unwrap_or_else(|_| "http://message-app:3000".to_string()),
            message_token: std::env::var("MESSAGE_TOKEN").ok(),
        }
    }

    fn build_request(
        &self,
        method: reqwest::Method,
        url: &str,
        token: &Option<String>,
    ) -> reqwest::RequestBuilder {
        let mut req = self.http.request(method, url);
        if let Some(t) = token {
            req = req.header("Authorization", format!("Bearer {}", t));
        }
        req
    }

    pub async fn get(&self, url: &str, token: &Option<String>) -> anyhow::Result<Value> {
        let resp = self
            .build_request(reqwest::Method::GET, url, token)
            .send()
            .await?;
        let status = resp.status();
        let text = resp.text().await?;
        if !status.is_success() {
            anyhow::bail!("HTTP {}: {}", status, text);
        }
        Ok(serde_json::from_str(&text).unwrap_or(Value::String(text)))
    }

    pub async fn post_json(
        &self,
        url: &str,
        token: &Option<String>,
        body: &Value,
    ) -> anyhow::Result<Value> {
        let resp = self
            .build_request(reqwest::Method::POST, url, token)
            .json(body)
            .send()
            .await?;
        let status = resp.status();
        let text = resp.text().await?;
        if !status.is_success() {
            anyhow::bail!("HTTP {}: {}", status, text);
        }
        Ok(serde_json::from_str(&text).unwrap_or(Value::String(text)))
    }

    pub async fn post_body(
        &self,
        url: &str,
        token: &Option<String>,
        body: String,
        content_type: &str,
    ) -> anyhow::Result<Value> {
        let resp = self
            .build_request(reqwest::Method::POST, url, token)
            .header("Content-Type", content_type)
            .body(body)
            .send()
            .await?;
        let status = resp.status();
        let text = resp.text().await?;
        if !status.is_success() {
            anyhow::bail!("HTTP {}: {}", status, text);
        }
        Ok(serde_json::from_str(&text).unwrap_or(Value::String(text)))
    }
}
