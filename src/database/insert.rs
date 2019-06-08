use serde::{Deserialize, Serialize};

pub mod sync {

    use super::{InsertPayload, InsertRequestQuery, InsertResponse};
    use crate::inner_client::sync::InnerClient;
    use crate::Error;
    use serde::Serialize;

    /// A request to insert a document into a CouchDB database.
    ///
    /// The request is lazy- it won't do anything until you call its 'send' method.
    pub struct InsertRequest<'a, T>
    where
        T: Serialize,
    {
        client: InnerClient,
        payload: InsertPayload<'a, T>,
        query: InsertRequestQuery,
    }

    impl<'a, T> InsertRequest<'a, T>
    where
        T: Serialize,
    {
        pub(crate) fn new(
            client: &InnerClient,
            document: &'a T,
            id: impl Into<Option<String>>,
        ) -> Self {
            InsertRequest {
                client: client.duplicate(),
                payload: InsertPayload {
                    _id: id.into(),
                    payload: document,
                },
                query: InsertRequestQuery::default(),
            }
        }

        pub fn send(self) -> Result<InsertResponse, Error> {
            let response = self
                .client
                .post()
                .json(&self.payload)
                .query(&self.query)
                .send()?
                .json()?;
            Ok(response)
        }
    }

}

pub mod r#async {

    use super::{InsertPayload, InsertRequestQuery, InsertResponse};
    use crate::inner_client::r#async::InnerClient;
    use crate::Error;
    use serde::Serialize;
    use tokio::prelude::Future;

    /// A Request to insert a document into the database
    pub struct InsertRequest<'a, T>
    where
        T: Serialize,
    {
        client: InnerClient,
        payload: InsertPayload<'a, T>,
        query: InsertRequestQuery,
    }

    impl<'a, T> InsertRequest<'a, T>
    where
        T: Serialize,
    {
        pub(crate) fn new(
            client: &InnerClient,
            document: &'a T,
            id: impl Into<Option<String>>,
        ) -> Self {
            InsertRequest {
                client: client.duplicate(),
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
        pub fn send(self) -> impl Future<Item = InsertResponse, Error = Error> {
            self.client
                .post()
                .json(&self.payload)
                .query(&self.query)
                .send()
                .and_then(|mut response| response.json())
                .map_err(Error::from)
        }
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
