use crate::{NexApiResponse, commands::keys::get_api_key};
use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Serialize};

pub struct NexApiClient {
    client: Client,
    base_url: String,
}

impl NexApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://nexhealth.info/".into(),
        }
    }

    pub async fn request<T, B, Q>(
        &self,
        path: &str,
        method: Method,
        body: Option<&B>,
        query: Option<&Q>,
        use_beta: bool,
    ) -> Result<NexApiResponse<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned + Serialize,
        B: Serialize,
        Q: Serialize,
    {
        let url = format!("{}{}", self.base_url.trim(), path.trim());
        let mut request = self.client.request(method, &url);

        let key_response = get_api_key().expect("No api key is saved for the client to use").unwrap();

        let trimmed_token: String = key_response.trim().to_string();
        request = request.header("Authorization", &trimmed_token);

        request = request.header("Content-Type", "application/json");

        let version = if use_beta { "v20240412" } else { "v2" };
        request = request.header("Nex-Api-Version", version);

        if let Some(q) = query {
            request = request.query(q);
        }

        if let Some(b) = body {
            request = request.json(b);
        }

        let built_request = request.build()?;
        println!("Query URL: {}", built_request.url());

        let response = self.client.execute(built_request).await?;

        // let text = response.text().await?;
        // println!("{}", text);
        // Err("Request cancelled on purpose.".into())

        let parsed = response.json::<NexApiResponse<T>>().await?;

        Ok(parsed)
    }
}
