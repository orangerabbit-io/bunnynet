use anyhow::{bail, Context, Result};
use reqwest::blocking::{Client as HttpClient, Response};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::models::pagination::PaginatedList;

pub struct Client {
    http: HttpClient,
    base_url: String,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String, base_url: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));

        let http = HttpClient::builder()
            .default_headers(headers)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Client {
            http,
            base_url,
            api_key,
        })
    }

    pub fn get(&self, path: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .get(&url)
            .header("AccessKey", &self.api_key)
            .send()
            .with_context(|| format!("Request failed: GET {}", url))?;
        Self::check_status(resp)
    }

    pub fn get_with_params(&self, path: &str, params: &[(&str, &str)]) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .get(&url)
            .header("AccessKey", &self.api_key)
            .query(params)
            .send()
            .with_context(|| format!("Request failed: GET {}", url))?;
        Self::check_status(resp)
    }

    #[allow(dead_code)]
    pub fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self.get(path)?;
        resp.json::<T>().context("Failed to parse JSON response")
    }

    #[allow(dead_code)]
    pub fn get_json_with_params<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        let resp = self.get_with_params(path, params)?;
        resp.json::<T>().context("Failed to parse JSON response")
    }

    pub fn get_bytes(&self, path: &str) -> Result<Vec<u8>> {
        let resp = self.get(path)?;
        let bytes = resp.bytes().context("Failed to read response bytes")?;
        Ok(bytes.to_vec())
    }

    pub fn post(&self, path: &str, body: &HashMap<String, serde_json::Value>) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .header("AccessKey", &self.api_key)
            .json(body)
            .send()
            .with_context(|| format!("Request failed: POST {}", url))?;
        Self::check_status(resp)
    }

    pub fn post_no_body(&self, path: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .header("AccessKey", &self.api_key)
            .send()
            .with_context(|| format!("Request failed: POST {}", url))?;
        Self::check_status(resp)
    }

    pub fn post_text(&self, path: &str, body: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .header("AccessKey", &self.api_key)
            .header("Content-Type", "text/plain")
            .body(body.to_string())
            .send()
            .with_context(|| format!("Request failed: POST {}", url))?;
        Self::check_status(resp)
    }

    pub fn post_with_params(&self, path: &str, params: &[(&str, &str)]) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .header("AccessKey", &self.api_key)
            .query(params)
            .send()
            .with_context(|| format!("Request failed: POST {}", url))?;
        Self::check_status(resp)
    }

    pub fn put(&self, path: &str, body: &HashMap<String, serde_json::Value>) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .put(&url)
            .header("AccessKey", &self.api_key)
            .json(body)
            .send()
            .with_context(|| format!("Request failed: PUT {}", url))?;
        Self::check_status(resp)
    }

    pub fn put_file(&self, path: &str, data: Vec<u8>, content_type: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .put(&url)
            .header("AccessKey", &self.api_key)
            .header("Content-Type", content_type)
            .body(data)
            .send()
            .with_context(|| format!("Request failed: PUT {}", url))?;
        Self::check_status(resp)
    }

    pub fn delete(&self, path: &str) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .delete(&url)
            .header("AccessKey", &self.api_key)
            .send()
            .with_context(|| format!("Request failed: DELETE {}", url))?;
        Self::check_status(resp)
    }

    pub fn delete_with_body(
        &self,
        path: &str,
        body: &HashMap<String, serde_json::Value>,
    ) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .delete(&url)
            .header("AccessKey", &self.api_key)
            .json(body)
            .send()
            .with_context(|| format!("Request failed: DELETE {}", url))?;
        Self::check_status(resp)
    }

    /// Fetch all pages of a paginated list endpoint, returning a combined Vec of items.
    /// `params` should contain any extra query parameters (e.g. search, includeDeleted)
    /// but NOT page or perPage — those are managed automatically.
    pub fn fetch_all_pages<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<Vec<T>> {
        let mut all_items = Vec::new();
        let mut page: i32 = 1;
        loop {
            let mut page_params: Vec<(&str, String)> = params
                .iter()
                .map(|(k, v)| (*k, v.to_string()))
                .collect();
            page_params.push(("page", page.to_string()));
            page_params.push(("perPage", "1000".to_string()));

            let params_ref: Vec<(&str, &str)> =
                page_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

            let resp = self.get_with_params(path, &params_ref)?;
            let list: PaginatedList<T> = resp
                .json()
                .context("Failed to parse paginated response")?;
            all_items.extend(list.items);

            if !list.has_more_items {
                break;
            }
            page += 1;
        }
        Ok(all_items)
    }

    fn check_status(resp: Response) -> Result<Response> {
        let status = resp.status();
        if status.is_success() {
            return Ok(resp);
        }
        let url = resp.url().to_string();
        let body = resp.text().unwrap_or_default();
        match status.as_u16() {
            400 => bail!("Bad request (HTTP {}): {}", status, body),
            401 | 403 => bail!("Authentication failed (HTTP {}): {}", status, body),
            404 => bail!("Not found (HTTP {}): {}", status, body),
            422 => bail!("Validation error (HTTP {}): {}", status, body),
            429 => bail!("Rate limited (HTTP {}): {}", status, body),
            _ => bail!("API error (HTTP {}) for {}: {}", status, url, body),
        }
    }
}
