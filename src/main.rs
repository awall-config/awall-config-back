//! An introduction to fundamental `Router` and `Router Builder` concepts to create a routing tree.

extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate pnet;
extern crate clap;
extern crate futures;

use gotham::router::builder::*;
use gotham::router::Router;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;

use clap::{Arg, App};

mod interfaces_routes;
mod config_routes;
mod models;
mod config;
mod cors_middleware;

/// Create a `Router`
///
/// Provides tree of routes with only a single top level entry that looks like:
///
/// /                     --> GET
///
/// If no match for a request is found a 404 will be returned. Both the HTTP verb and the request
/// path are considered when determining if the request matches a defined route.
fn router(config: config::Config) -> Router {

    let config_middleware = config::ConfigMiddleware::new ( config );

    let corsMiddleware = cors_middleware::CORSMiddleware::default();

    let (chain, pipelines) = single_pipeline(
        new_pipeline()
        .add(config_middleware)
        .add(corsMiddleware)
        .build()    
    );

    build_router(chain, pipelines, |route| {
        // For the path "/" invoke the handler "say_hello"
        route.get("/interfaces").to(interfaces_routes::get_interfaces);
        route.get("/config").to(config_routes::get_config);
        route.put("/config").to(config_routes::put_config);
    })
}

/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    let matches = App::new("Awall configurator")
                          .version("1.0")
                          .author("D. Ledanseur <david.ledanseur@intech.lu>")
                          .about("Web Gui to configure the Awall firewall")
                          .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .help("The awall config file")
                            .required(true)
                            .takes_value(true))
                          .get_matches();

    let conf = matches.value_of("config").unwrap_or("/etc/awall/private/base.json");
    let config_obj = config::Config::new ( conf.to_owned() );

    println!("Using config file: {}", conf);

    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);

    // All incoming requests are delegated to the router for further analysis and dispatch
    gotham::start(addr, router(config_obj))
}
