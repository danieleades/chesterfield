use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod sync {

    use super::{GetRequestQuery, GetResponse};
    use crate::inner_client::sync::InnerClient;
    use crate::Error;
    use serde::de::DeserializeOwned;

    /// A request to retrieve a document from a CouchDB database.
    ///
    /// The request is lazy- it doesn't do a thing until you call its '[send](GetRequest::send)'
    /// method.
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

        /// Send the request.
        ///
        /// This will consume the 'get' request and return a [GetResponse](GetResponse).
        /// The response is generic, so occasionally you might need type annotations.
        pub fn send<T: DeserializeOwned>(self) -> Result<GetResponse<T>, Error> {
            println!("get request base url: {}", self.client.url());
            println!("joined url: {}", self.client.url().join(&self.id).unwrap());
            let request = self.client.join(&self.id)?.get().query(&self.query);

            println!("sending request: {:#?}", request);

            let response = request.send()?.json()?;

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

    /// A request to retrieve a document from a CouchDB database.
    ///
    /// The request is lazy- it doesn't do a thing until you call its '[send](GetRequest::send)'
    /// method.
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

        /// Send the request.
        ///
        /// This will consume the 'get' request and return a [GetResponse](GetResponse).
        /// The response is generic, so occasionally you might need type annotations.
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

#[derive(Debug, Deserialize)]
pub struct GetResponseMeta {
    pub _id: String,
    pub _rev: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _deleted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _attachments: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _conflicts: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _deleted_conflicts: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _local_seq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _revs_info: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _revisions: Option<Value>,
}

/// A response from a GetRequest.
///
/// The response is generic over a type parameter T. You can use this
/// to strongly type the response. Alternatively you can use the
/// default generic parameter of serde_json::Value, which can represent
/// any valid response from the database.
///
/// The GetResponse implements Deref with respect to the returned document.
/// You can also consume the response and retrieve the document with the [into_inner](GetResponse::into_inner)
/// method.
#[derive(Debug, Deserialize)]
pub struct GetResponse<T = Value> {
    #[serde(flatten)]
    document: Option<T>,

    #[serde(flatten)]
    meta_data: GetResponseMeta,
}

impl<T> GetResponse<T> {
    /// Return metadata about the response.
    ///
    /// Which metadata is available will depend on the options set
    /// in the request.
    pub fn meta_data(&self) -> &GetResponseMeta {
        &self.meta_data
    }

    /// Consume the response and return the contained document
    pub fn into_inner(self) -> Option<T> {
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
