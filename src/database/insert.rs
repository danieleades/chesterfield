use crate::client::Client;
use crate::Result;
use futures::compat::Future01CompatExt;
use serde::{Deserialize, Serialize};

/// A Request to insert a document into the database
pub struct InsertRequest<'a, T>
where
    T: Serialize,
{
    client: Client,
    payload: InsertPayload<'a, T>,
    query: InsertRequestQuery,
}

impl<'a, T> InsertRequest<'a, T>
where
    T: Serialize,
{
    pub(crate) fn new(client: &Client, document: &'a T, id: impl Into<Option<String>>) -> Self {
        InsertRequest {
            client: client.into(),
            payload: InsertPayload {
                _id: id.into(),
                payload: document,
            },
            query: InsertRequestQuery::default(),
        }
    }

    /// Consume the request and send it to the database.
    ///
    /// Returns a future that resolves to an InsertResponse.
    pub async fn send(self) -> Result<InsertResponse> {
        let response = self
            .client
            .post()
            .json(&self.payload)
            .query(&self.query)
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
pub struct InsertPayload<'a, T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<String>,

    #[serde(flatten)]
    payload: &'a T,
}

#[derive(Serialize, Clone, Default)]
pub struct InsertRequestQuery {
    batch: Option<String>,
}

/// Reponse from the CouchDB database after inserting a document
#[derive(Debug, Deserialize)]
pub struct InsertResponse {
    /// The _id of the inserted document
    pub id: String,

    /// Insert operation status
    pub ok: bool,

    /// The current revision of the inserted document
    pub rev: String,
}
