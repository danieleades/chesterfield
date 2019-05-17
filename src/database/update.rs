use serde::{Deserialize, Serialize};

pub mod sync {

    use super::{UpdatePayload, UpdateResponse};
    use crate::inner_client::sync::InnerClient;
    use crate::Error;
    use serde::Serialize;

    pub struct UpdateRequest<'a, T>
    where
        T: Serialize,
    {
        client: &'a InnerClient,
        _id: String,
        payload: UpdatePayload<T>,
    }

    impl<'a, T> UpdateRequest<'a, T>
    where
        T: Serialize,
    {
        pub(crate) fn new(
            client: &'a InnerClient,
            document: T,
            id: impl Into<String>,
            rev: impl Into<String>,
        ) -> Self {
            UpdateRequest {
                client,
                _id: id.into(),
                payload: UpdatePayload {
                    _rev: rev.into(),
                    payload: document,
                },
            }
        }

        pub fn send(self) -> Result<UpdateResponse, Error> {
            let response = self
                .client
                .join(self._id)?
                .put()
                .json(&self.payload)
                .send()?
                .json()?;
            Ok(response)
        }
    }

}

pub mod r#async {

    use super::{UpdatePayload, UpdateResponse};
    use crate::inner_client::r#async::InnerClient;
    use crate::Error;
    use serde::Serialize;
    use tokio::prelude::{future::result, Future};

    pub struct UpdateRequest<T>
    where
        T: Serialize,
    {
        client: InnerClient,
        _id: String,
        payload: UpdatePayload<T>,
    }

    impl<T> UpdateRequest<T>
    where
        T: Serialize,
    {
        pub(crate) fn new(
            client: &InnerClient,
            document: T,
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

        pub fn send(self) -> impl Future<Item = UpdateResponse, Error = Error> {
            result(self.client.join(&self._id).map_err(Error::from))
                .and_then(move |client| {
                    client.put().json(&self.payload).send().map_err(Error::from)
                })
                .and_then(|mut response| response.json().map_err(Error::from))
        }
    }

}

#[derive(Serialize)]
pub struct UpdatePayload<T> {
    _rev: String,

    #[serde(flatten)]
    payload: T,
}

#[derive(Deserialize)]
pub struct UpdateResponse {
    pub id: String,
    pub ok: bool,
    pub rev: String,
}
