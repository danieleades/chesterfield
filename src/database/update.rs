use serde::{Deserialize, Serialize};

use crate::inner_client::InnerClient;
use crate::Error;
use futures::compat::Future01CompatExt;

/// A request to update an existing document.
pub struct UpdateRequest<'a, T>
where
    T: Serialize,
{
    client: InnerClient,
    _id: String,
    payload: UpdatePayload<'a, T>,
}

impl<'a, T> UpdateRequest<'a, T>
where
    T: Serialize,
{
    pub(crate) fn new(
        client: &InnerClient,
        document: &'a T,
        id: impl Into<String>,
        rev: impl Into<String>,
    ) -> Self {
        UpdateRequest {
            client: client.duplicate(),
            _id: id.into(),
            payload: UpdatePayload {
                _rev: rev.into(),
                payload: document,
            },
        }
    }

    pub async fn send(self) -> Result<UpdateResponse, Error> {
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
