extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;


fn main() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri: Uri = "http://httpbin.org/ip".parse().unwrap();
    println!("Sending request to uri: [{}]", uri);
    println!();

    let work =
        client.get(uri).and_then(|res| {
            println!("Response: {}", res.status());

            res.body().for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(From::from)
            })
        });
    core.run(work).unwrap();
}

// fn main() {
//     println!("Hello, world!");
// }

