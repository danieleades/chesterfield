use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::Result;
use futures::compat::Future01CompatExt;

/// A request to update an existing document.
pub struct UpdateRequest<'a, T>
where
    T: Serialize,
{
    client: Client,
    _id: String,
    payload: UpdatePayload<'a, T>,
}

impl<'a, T> UpdateRequest<'a, T>
where
    T: Serialize,
{
    pub(crate) fn new(
        client: &Client,
        document: &'a T,
        id: impl Into<String>,
        rev: impl Into<String>,
    ) -> Self {
        UpdateRequest {
            client: client.into(),
            _id: id.into(),
            payload: UpdatePayload {
                _rev: rev.into(),
                payload: document,
            },
        }
    }

    /// Consume the update request and send it to the remote
    pub async fn send(self) -> Result<UpdateResponse> {
        let response = self
            .client
            .join(&self._id)?
            .put()
            .json(&self.payload)
            .send()
            .compat()
            .await?
            .json()
            .compat()
            .await?;
        Ok(response)
    }
}

#[derive(Serialize)]
pub struct UpdatePayload<'a, T> {
    _rev: String,

    #[serde(flatten)]
    payload: &'a T,
}

#[derive(Deserialize)]
pub struct UpdateResponse {
    pub id: String,
    pub ok: bool,
    pub rev: String,
}
