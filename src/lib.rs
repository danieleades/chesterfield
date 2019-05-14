mod client;
mod inner_client;
pub use client::Client;
mod database;
pub use database::Database;
mod error;
//pub use error::Error;
//mod traits;

pub use reqwest::Url;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
