#![allow(unused_imports)]
#![allow(unused_doc_comments)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![recursion_limit = "128"]

extern crate actix;
extern crate actix_web;
extern crate base64;
extern crate chrono;
extern crate data_encoding;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate failure;
extern crate futures;

#[macro_use]
extern crate log;
extern crate nanoid;
extern crate pretty_env_logger;
extern crate redis;
extern crate ring;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate validator_derive;
extern crate validator;
extern crate uuid;

// Internal modules
pub mod db;
pub mod utils;
