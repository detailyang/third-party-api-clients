use anyhow::Result;

use crate::Client;

pub struct EnvelopeWorkflowDefinition {
    pub client: Client,
}

impl EnvelopeWorkflowDefinition {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeWorkflowDefinition { client }
    }

    /**
    * Gets an envelope's workflow definition.
    *
    * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/workflow` endpoint.
    *
    * Returns an envelope's workflow definition if the envelope specified by `envelopeId` has one.
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn get(&self, account_id: &str, envelope_id: &str) -> Result<crate::types::Workflow> {
        let url = format!(
            "/v2.1/accounts/{}/envelopes/{}/workflow",
            crate::progenitor_support::encode_path(account_id),
            crate::progenitor_support::encode_path(envelope_id),
        );

        self.client.get(&url, None).await
    }

    /**
    * Updates an envelope's workflow definition.
    *
    * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/workflow` endpoint.
    *
    * Updates the specified envelope's workflow definition if  it has one.
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn put(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::Workflow,
    ) -> Result<crate::types::Workflow> {
        let url = format!(
            "/v2.1/accounts/{}/envelopes/{}/workflow",
            crate::progenitor_support::encode_path(account_id),
            crate::progenitor_support::encode_path(envelope_id),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Deletes an envelope's workflow definition.
    *
    * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/workflow` endpoint.
    *
    * Deletes the specified envelope's workflow definition if it has one.
    *
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn delete(&self, account_id: &str, envelope_id: &str) -> Result<()> {
        let url = format!(
            "/v2.1/accounts/{}/envelopes/{}/workflow",
            crate::progenitor_support::encode_path(account_id),
            crate::progenitor_support::encode_path(envelope_id),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Gets template's workflow definition.
    *
    * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/workflow` endpoint.
    *
    * Returns template's workflow definition if the template specified by `templateId` has one.
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn get_template(
        &self,
        account_id: &str,
        template_id: &str,
    ) -> Result<crate::types::Workflow> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/workflow",
            crate::progenitor_support::encode_path(account_id),
            crate::progenitor_support::encode_path(template_id),
        );

        self.client.get(&url, None).await
    }

    /**
    * Update a template's workflow definiton.
    *
    * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/workflow` endpoint.
    *
    * Updates the specified template's workflow definition if  it has one.
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn put_template(
        &self,
        account_id: &str,
        template_id: &str,
        body: &crate::types::Workflow,
    ) -> Result<crate::types::Workflow> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/workflow",
            crate::progenitor_support::encode_path(account_id),
            crate::progenitor_support::encode_path(template_id),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Deletes a template's workflow definition.
    *
    * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/templates/{templateId}/workflow` endpoint.
    *
    * Deletes the specified template's workflow definition if it has one.
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn delete_template(&self, account_id: &str, template_id: &str) -> Result<()> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/workflow",
            crate::progenitor_support::encode_path(account_id),
            crate::progenitor_support::encode_path(template_id),
        );

        self.client.delete(&url, None).await
    }
}
