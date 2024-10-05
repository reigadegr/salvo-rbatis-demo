mod config;
mod research;
mod res;
mod pojo;
mod mapper;

use config::nacos::init_nacos_service;
use config::init::init_filesystem;


#[tokio::main]
async fn main() {
    init_nacos_service().await;
    init_filesystem().await;
}
