use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod sync {

    use super::{GetRequestQuery, GetResponse};
    use crate::inner_client::sync::InnerClient;
    use crate::Error;
    use serde::de::DeserializeOwned;

    pub struct GetRequest<'a> {
        id: String,
        client: &'a InnerClient,
        query: GetRequestQuery,
    }

    impl<'a> GetRequest<'a> {
        pub(crate) fn new(client: &'a InnerClient, id: impl Into<String>) -> Self {
            GetRequest {
                id: id.into(),
                client,
                query: GetRequestQuery::default(),
            }
        }

        pub fn send<T: DeserializeOwned>(self) -> Result<GetResponse<T>, Error> {
            let response = self
                .client
                .join(&self.id)?
                .get()
                .query(&self.query)
                .send()?
                .json()?;
            Ok(response)
        }
    }

}

pub mod r#async {

    use super::{GetRequestQuery, GetResponse};
    use crate::inner_client::r#async::InnerClient;
    use crate::Error;
    use serde::de::DeserializeOwned;
    use tokio::prelude::{future::result, Future};

    pub struct GetRequest {
        id: String,
        client: InnerClient,
        query: GetRequestQuery,
    }

    impl GetRequest {
        pub(crate) fn new(client: &InnerClient, id: impl Into<String>) -> Self {
            GetRequest {
                id: id.into(),
                client: client.duplicate(),
                query: GetRequestQuery::default(),
            }
        }

        pub fn send<T: DeserializeOwned>(
            self,
        ) -> impl Future<Item = GetResponse<T>, Error = Error> {
            result(self.client.join(&self.id).map_err(Error::from))
                .and_then(move |client| client.get().query(&self.query).send().map_err(Error::from))
                .and_then(|mut response| response.json().map_err(Error::from))
        }
    }

}

#[derive(Serialize, Clone)]
pub struct GetRequestQuery {
    attachments: bool,
    att_encoding_info: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    atts_since: Option<Vec<String>>,
    conflicts: bool,
    deleted_conflicts: bool,
    latest: bool,
    local_seq: bool,
    meta: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_revs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rev: Option<String>,
    revs: bool,
    revs_info: bool,
}

impl Default for GetRequestQuery {
    fn default() -> Self {
        GetRequestQuery {
            attachments: false,
            att_encoding_info: false,
            atts_since: None,
            conflicts: false,
            deleted_conflicts: false,
            latest: false,
            local_seq: false,
            meta: false,
            open_revs: None,
            rev: None,
            revs: false,
            revs_info: false,
        }
    }
}

#[derive(Deserialize)]
pub struct GetResponseMeta {
    pub _id: String,
    pub _rev: String,
    pub _deleted: Option<bool>,
    pub _attachments: Option<Value>,
    pub _conflicts: Option<Vec<Value>>,
    pub _deleted_conflicts: Option<Vec<Value>>,
    pub _local_seq: Option<String>,
    pub _revs_info: Option<Vec<Value>>,
    pub _revisions: Option<Value>,
}

#[derive(Deserialize)]
pub struct GetResponse<T = Value> {
    #[serde(flatten)]
    pub document: T,

    #[serde(flatten)]
    pub metadata: GetResponseMeta,
}

impl<T> GetResponse<T> {
    pub fn into_inner(self) -> T {
        self.document
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
