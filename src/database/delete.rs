use serde::{Deserialize, Serialize};

pub mod sync {

    use super::{DeleteRequestQuery, DeleteResponse};
    use crate::inner_client::sync::InnerClient;
    use crate::Error;

    pub struct DeleteRequest<'a> {
        id: String,
        client: &'a InnerClient,
        query: DeleteRequestQuery,
    }

    impl<'a> DeleteRequest<'a> {
        pub(crate) fn new(
            client: &'a InnerClient,
            id: impl Into<String>,
            rev: impl Into<String>,
        ) -> Self {
            DeleteRequest {
                id: id.into(),
                client,
                query: DeleteRequestQuery::new(rev),
            }
        }

        pub fn send(self) -> Result<DeleteResponse, Error> {
            let response = self
                .client
                .join(&self.id)?
                .delete()
                .query(&self.query)
                .send()?
                .json()?;
            Ok(response)
        }
    }

}

pub mod r#async {

    use super::{DeleteRequestQuery, DeleteResponse};
    use crate::inner_client::r#async::InnerClient;
    use crate::Error;
    use tokio::prelude::{future::result, Future};

    pub struct DeleteRequest {
        id: String,
        client: InnerClient,
        query: DeleteRequestQuery,
    }

    impl DeleteRequest {
        pub(crate) fn new(
            client: &InnerClient,
            id: impl Into<String>,
            rev: impl Into<String>,
        ) -> Self {
            DeleteRequest {
                id: id.into(),
                client: client.duplicate(),
                query: DeleteRequestQuery::new(rev),
            }
        }

        pub fn send(
            self,
        ) -> impl Future<Item = DeleteResponse, Error = Error> {
            result(self.client.join(&self.id).map_err(Error::from))
                .and_then(move |client| {
                    client
                        .delete()
                        .query(&self.query)
                        .send()
                        .map_err(Error::from)
                })
                .and_then(|mut response| response.json().map_err(Error::from))
        }
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
