use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::create_response;
use gotham::state::{State, FromState};
use pnet::datalink::{self, NetworkInterface};
use futures::{future, Future, Stream};
use hyper::{Body, Response, StatusCode};
use std::process::Command;

use super::models;
use super::config;

use std::fs;

fn error(state: &State, e: &std::error::Error) -> Response<Body> {
  let msg = models::Message {
    message: e.to_string()
  };

  let r = create_response(state,
                  StatusCode::INTERNAL_SERVER_ERROR,
                  mime::APPLICATION_JSON, 
                  serde_json::to_string(&msg).expect("Error"));
  return r;
}

fn result_to_response<T,E> (state: &State, res: Result<T,E>) -> Response<Body> 
  where T: std::convert::Into<Body>,
        E: std::error::Error {
  match res {
    Ok(o) => {
      let r = create_response(state,
                      StatusCode::OK,
                      mime::APPLICATION_JSON, 
                      o);

      return r;
    },
    Err(e) => {
      return error(state, &e)
    }

  }
}

fn get_config_filename(state: &State) -> &str {
  let config = config::Config::borrow_from(state);
  let filename = config.get_config_file();

  filename
}
/// Create a `Handler` that is invoked for requests to the path "/"
pub fn get_config(state: State) -> (State, Response<Body>) {

  let filename = get_config_filename(&state);
  let config_content = fs::read_to_string(filename);
  
  let resp = result_to_response(&state, config_content);
  
  (state, resp)

}

/// Create a `Handler` that is invoked for requests to the path "/"
pub fn put_config(mut state: State) -> Box<HandlerFuture> {
  
  let body = Body::take_from(&mut state);

  let f = body.concat2().then ( |c| match c {
    Ok(valid_body) => { 
      let conf_str = valid_body.into_bytes();
      
      let conf_json: serde_json::Result<models::Config> = serde_json::from_slice(&conf_str);

      match conf_json {
        Ok(json) => {
          // we have a valid json. Can write it to fs and apply config
          let filename = get_config_filename(&state);

          // convert back the cleaned json structure (because of parsing) to a string
          let result_config_str = serde_json::to_string(&json).unwrap();

          // and write it
          let write_res = fs::write(filename, &result_config_str);

          let activ_res = write_res.map( |_| Command::new("awall").args(&["activate"]).output());         

          let final_res = activ_res.map(|_| result_config_str.clone());

          let response = result_to_response(&state, final_res);

          return future::ok((state, response))
        }
        Err(e) => {
          let response = error(&state,&e);
          return future::ok((state, response))
        }
      }

    }
    Err(e) => {
      let response = error(&state,&e);

      return future::ok((state, response))
    }
  });

  Box::new(f)
}
