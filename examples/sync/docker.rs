use shiplift::{ContainerOptions, Docker, PullOptions, RmContainerOptions};
use std::net::TcpListener;
use tokio::prelude::{Future, Stream};

// don't worry too much about the detail here. All this is doing is creating a temporary
// CouchDB instance in a docker container for testing purposes.
// see https://github.com/softprops/shiplift for details.

pub fn get_unused_port() -> u32 {
    let listener = TcpListener::bind("localhost:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    port.into()
}

// create and run a new couchdb container exposed at the given port,
// and return the docker ID of that container.
pub fn create_couchdb_container(port: u32) -> String {
    let id = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(
            Docker::default()
                // pull latest couchdb image
                .images()
                .pull(&PullOptions::builder().image("couchdb:latest").build())
                .for_each(|output| {
                    println!("{:?}", output);
                    Ok(())
                })
                .map(|_| {
                    println!("pulled image: couchdb:latest");
                    ()
                })
                // then create the couchdb container
                .and_then(move |_| {
                    Docker::default().containers().create(
                        &ContainerOptions::builder("couchdb:latest")
                            .expose(5984, "tcp", port)
                            .build(),
                    )
                })
                .map(|info| {
                    let id = info.id;
                    println!("created container: {}", &id);
                    id
                }),
        )
        .unwrap();

    tokio::run(
        Docker::default()
            .containers()
            .get(&id)
            .start()
            .map(|_| println!("started container"))
            // handle any errors
            .map_err(|e| {
                println!("error: {}", e);
            }),
    );

    id
}

// kill the container. kill it with fire.
pub fn remove_couchdb_container(id: &str) {
    tokio::run(
        Docker::default()
            .containers()
            .get(&id)
            .remove(RmContainerOptions::builder().force(true).build())
            .map(|_| println!("removed container"))
            // handle any errors
            .map_err(|e| {
                println!("error: {}", e);
            }),
    );
}
