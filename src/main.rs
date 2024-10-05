extern crate rbatis;
extern crate rbdc;
mod config;
mod controller;
mod res;
mod pojo;
mod mapper;
mod services;

use config::nacos::init_nacos_service;
use config::init::init_filesystem;

#[tokio::main]
async fn main() {
    init_nacos_service().await;
    init_filesystem().await;
}
