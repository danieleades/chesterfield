use chesterfield::{sync::Client, GetResponse};
use serde_json::{json, Value};
use std::{thread, time};

mod docker;
use self::docker::{create_couchdb_container, get_unused_port, remove_couchdb_container};

// This example assumes you have docker installed!

fn main() {

    // find a random port
    let couchdb_port = get_unused_port();
    println!("using host port {}", couchdb_port);

    // create a couchdb docker container
    let container_id = create_couchdb_container(couchdb_port);

    let couchdb_url = format!("http://localhost:{}", couchdb_port);

    // create a new couchdb client
    let client = Client::new(couchdb_url).unwrap();

    // create a new interface to the database "examples"
    let database = client.database("examples").unwrap();

    // define a serde_json::Value document to store in the database
    let document = json!(
        {
            "field1": 5,
            "field2": "some string",
            "object" : {
                "values": [
                    "value1",
                    "value2",
                    "value3",
                ]
            }
        }
    );

    // choose a random id (you can also insert without an id, couchdb will generate one for you)
    let document_id = "abcd1234".to_string();

    // wait for database to start up
    println!("waiting 5 seconds for couchdb node to be up");
    thread::sleep(time::Duration::from_secs(5));

    // create the database
    println!("creating database");
    database.create().unwrap();

    // insert the document
    println!("inserting document");
    println!("{:#?}", database.insert(document, document_id.clone()).send().unwrap());

    // retrieve the document, using the ID.
    let get_response: GetResponse = database.get(document_id).send().unwrap();
    println!("{:#?}", get_response);

    // finally, delete the couchdb container
    remove_couchdb_container(&container_id);
}
