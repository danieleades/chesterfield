mod get;
mod insert;
mod update;
pub use self::{get::GetResponse, insert::InsertResponse, update::UpdateResponse};

pub mod sync {
    pub use super::{
        get::sync::GetRequest, insert::sync::InsertRequest, update::sync::UpdateRequest,
    };
    use crate::inner_client::sync::InnerClient;
    use serde::Serialize;

    pub struct Database {
        client: InnerClient,
    }

    impl Database {
        pub(crate) fn new(client: InnerClient) -> Self {
            Database { client }
        }

        pub fn get(&self, id: impl Into<String>) -> GetRequest {
            GetRequest::new(&self.client, id)
        }

        pub fn insert<T: Serialize>(
            &self,
            document: T,
            id: impl Into<Option<String>>,
        ) -> InsertRequest<T> {
            InsertRequest::new(&self.client, document, id)
        }

        pub fn update<T: Serialize>(
            &self,
            document: T,
            id: impl Into<String>,
            rev: impl Into<String>,
        ) -> UpdateRequest<T> {
            UpdateRequest::new(&self.client, document, id, rev)
        }
    }
}

pub mod r#async {
    pub use super::{
        get::r#async::GetRequest, insert::r#async::InsertRequest, update::r#async::UpdateRequest,
    };
    use crate::inner_client::r#async::InnerClient;
    use serde::Serialize;

    pub struct Database {
        client: InnerClient,
    }

    impl Database {
        pub(crate) fn new(client: InnerClient) -> Self {
            Database { client }
        }

        pub fn get(&self, id: impl Into<String>) -> GetRequest {
            GetRequest::new(&self.client, id)
        }

        pub fn insert<T: Serialize>(
            &self,
            document: T,
            id: impl Into<Option<String>>,
        ) -> InsertRequest<T> {
            InsertRequest::new(&self.client, document, id)
        }

        pub fn update<T: Serialize>(
            &self,
            document: T,
            id: impl Into<String>,
            rev: impl Into<String>,
        ) -> UpdateRequest<T> {
            UpdateRequest::new(&self.client, document, id, rev)
        }
    }
}
