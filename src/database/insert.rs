use serde::{Deserialize, Serialize};

pub mod sync {

    use super::{InsertPayload, InsertRequestQuery, InsertResponse};
    use crate::inner_client::sync::InnerClient;
    use crate::Error;
    use serde::Serialize;

    pub struct InsertRequest<'a, T>
    where
        T: Serialize,
    {
        client: &'a InnerClient,
        payload: InsertPayload<T>,
        query: InsertRequestQuery,
    }

    impl<'a, T> InsertRequest<'a, T>
    where
        T: Serialize,
    {
        pub(crate) fn new(
            client: &'a InnerClient,
            document: T,
            id: impl Into<Option<String>>,
        ) -> Self {
            InsertRequest {
                client,
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
                .get()
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

    pub struct InsertRequest<T>
    where
        T: Serialize,
    {
        client: InnerClient,
        payload: InsertPayload<T>,
        query: InsertRequestQuery,
    }

    impl<T> InsertRequest<T>
    where
        T: Serialize,
    {
        pub(crate) fn new(
            client: &InnerClient,
            document: T,
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
pub struct InsertPayload<T> {
    _id: Option<String>,
    payload: T,
}

#[derive(Serialize, Clone, Default)]
pub struct InsertRequestQuery {
    batch: Option<String>,
}

#[derive(Deserialize)]
pub struct InsertResponse {
    pub id: String,
    pub ok: bool,
    pub rev: String,
}
