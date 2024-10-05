extern crate rbatis;
// extern crate rbdc;
mod config;
mod controller;
mod mapper;
mod pojo;
mod res;
mod services;

use config::init::init_filesystem;
use config::nacos::init_nacos_service;

#[tokio::main]
async fn main() {
    init_nacos_service().await;
    init_filesystem().await;
}
