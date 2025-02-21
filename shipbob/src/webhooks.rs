use anyhow::Result;

use crate::Client;

pub struct Webhooks {
    pub client: Client,
}

impl Webhooks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Webhooks { client }
    }

    /**
    * Get Webhooks.
    *
    * This function performs a `GET` to the `/webhook` endpoint.
    *
    * All parameters are AND filters
    *
    * **Parameters:**
    *
    * * `topic: crate::types::WebhooksTopics` -- Topic of the webhooks requested.
    * * `page: u64` -- Unique id of the channel.
    * * `limit: i64` -- Amount of Webhooks per page to request.
    */
    pub async fn get_page(
        &self,
        topic: crate::types::WebhooksTopics,
        page: u64,
        limit: i64,
    ) -> Result<Vec<crate::types::Webhook>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("Limit".to_string(), limit.to_string()));
        }
        if !page.to_string().is_empty() {
            query_args.push(("Page".to_string(), page.to_string()));
        }
        if !topic.to_string().is_empty() {
            query_args.push(("Topic".to_string(), topic.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/webhook?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Get Webhooks.
    *
    * This function performs a `GET` to the `/webhook` endpoint.
    *
    * As opposed to `get`, this function returns all the pages of the request at once.
    *
    * All parameters are AND filters
    */
    pub async fn get_all(
        &self,
        topic: crate::types::WebhooksTopics,
    ) -> Result<Vec<crate::types::Webhook>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !topic.to_string().is_empty() {
            query_args.push(("Topic".to_string(), topic.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/webhook?{}", query_);

        self.client.get_all_pages(&url, None).await
    }

    /**
    * Create a new webhook subscription.
    *
    * This function performs a `POST` to the `/webhook` endpoint.
    *
    * **Parameters:**
    *
    * * `channel_id: i64` -- Unique id of the channel.
    */
    pub async fn post(
        &self,
        body: &crate::types::WebhooksCreateWebhookSubscriptionModel,
    ) -> Result<crate::types::Webhook> {
        let url = "/webhook".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Delete an existing webhook subscription.
    *
    * This function performs a `DELETE` to the `/webhook/{id}` endpoint.
    *
    * **Parameters:**
    *
    * * `id: i64` -- Unique id of the channel.
    */
    pub async fn delete(&self, id: i64) -> Result<()> {
        let url = format!(
            "/webhook/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
