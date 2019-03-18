extern crate serde;

use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::IntoIterator;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Interface {
    pub iface: String,
}

#[derive(Serialize, Deserialize)]
pub struct InterfaceVec(Vec<Interface>);

impl FromIterator<Interface> for InterfaceVec {

  fn from_iter<I: IntoIterator<Item=Interface>>(iter: I) -> Self {
      let mut c = InterfaceVec(Vec::new());

      for i in iter {
          c.0.push(i);
      }

      c
  }
}

#[derive(Serialize, Deserialize)]
pub struct Policy {
  #[serde(rename = "in")]
  #[serde(skip_serializing_if = "Option::is_none")] 
  in_f: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")] 
  out: Option<String>,
  action: String,
}

#[derive(Serialize, Deserialize)]
pub struct Snat {
  out: String
}

#[derive(Serialize, Deserialize)]
pub struct Service {
  proto: String,
  port: i16
}

#[derive(Serialize, Deserialize)]
pub struct ServiceMap {
  service: HashMap<String,Service>,
}

#[derive(Serialize, Deserialize)]
pub struct Dnat {
  #[serde(rename = "in")]
  in_f: String,
  dest: String,
  service: String,
  #[serde(rename = "to-port")]
  to_port: String
}

#[derive(Serialize, Deserialize)]
pub struct ClampMss {
  out: String
}

#[derive(Serialize, Deserialize)]
pub struct ConnLimit {
  count: i32,
  interval: i32
}

#[derive(Serialize, Deserialize)]
pub struct FlowLimit {
  count: i32,
  interval: i32
}

#[derive(Serialize, Deserialize)]
pub struct Filter {
  #[serde(rename = "in")]
  in_f: String,
  out: String,
  service: String,
  action: String,
  #[serde(rename = "conn-limit")]
  conn_limit: ConnLimit
}

#[derive(Serialize, Deserialize)]
pub struct Config {
  description: String,
  #[serde(skip_serializing_if = "Option::is_none")] 
  variable: Option<HashMap<String,String>>,
  zone: HashMap<String,Interface>,
  policy: Vec<Policy>,
  #[serde(skip_serializing_if = "Option::is_none")] 
  #[serde(rename = "clamp-mss")]
  clamp_mss: Option<Vec<ClampMss>>,
  #[serde(skip_serializing_if = "Option::is_none")] 
  filter: Option<Vec<Filter>>,
  #[serde(skip_serializing_if = "Option::is_none")] 
  dnat: Option<Vec<Dnat>>,
  #[serde(skip_serializing_if = "Option::is_none")] 
  snat: Option<Vec<Snat>>,
  #[serde(skip_serializing_if = "Option::is_none")] 
  service: Option<ServiceMap>
}
