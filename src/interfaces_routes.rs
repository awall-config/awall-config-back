use gotham::handler::IntoResponse;
use gotham::helpers::http::response::create_response;
use gotham::state::State;
use pnet::datalink::{self, NetworkInterface};

use hyper::{Body, Response, StatusCode};

use super::models;

impl IntoResponse for models::InterfaceVec {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self).expect("serialized product"),
        )
    }
}

/// Create a `Handler` that is invoked for requests to the path "/"
pub fn get_interfaces(state: State) -> (State, models::InterfaceVec) {
  let interf_vec = datalink::interfaces();

  let result_intf = interf_vec.into_iter().map(|i:NetworkInterface| {
      let iface_name:String = i.name;

      models::Interface { iface: iface_name}

  }).collect();

  (state, result_intf)
}
