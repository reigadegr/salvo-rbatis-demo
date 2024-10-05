mod config;
mod research;
mod res;
use config::nacos::init_nacos_service;
use config::init::init_filesystem;
#[macro_use]
extern crate rbatis;
extern crate rbdc;

use once_cell::sync::Lazy;
use rbatis::RBatis;
use serde::{Serialize, Deserialize};
pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Users {
    pub id: i64,
    pub username: String,
    pub password: String,
}


#[tokio::main]
async fn main() {
    init_nacos_service().await;
    init_filesystem().await;
}
