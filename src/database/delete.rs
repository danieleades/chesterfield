use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::Error;

pub struct DeleteRequest {
    id: String,
    client: Client,
    query: DeleteRequestQuery,
}

impl DeleteRequest {
    pub(crate) fn new(client: &Client, id: impl Into<String>, rev: impl Into<String>) -> Self {
        DeleteRequest {
            id: id.into(),
            client: client.into(),
            query: DeleteRequestQuery::new(rev),
        }
    }

    pub async fn send(self) -> Result<DeleteResponse, Error> {
        let response = self
            .client
            // create a new client pointing at "<database>/documentId"
            .join(&self.id)?
            // construct the delete request, and send it
            .delete()
            .query(&self.query)
            .send()
            .await?
            // extract the JSON blob
            .json()
            .await?;
        Ok(response)
    }
}

#[derive(Serialize, Clone)]
pub struct DeleteRequestQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch: Option<String>,

    rev: String,
}

impl DeleteRequestQuery {
    fn new(rev: impl Into<String>) -> Self {
        DeleteRequestQuery {
            batch: None,
            rev: rev.into(),
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteResponse {
    pub id: String,
    pub ok: bool,
    pub rev: String,
}
