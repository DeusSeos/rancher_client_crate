#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate serde_repr;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate reqwest;

pub mod apis;
pub mod models;


pub fn default_api_version() -> Option<String> {
    Some("management.cattle.io/v3".to_string())
}