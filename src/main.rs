mod config;
mod research;
mod res;
use config::router::init_router;
use config::nacos::init_nacos_service;
#[macro_use]
extern crate rbatis;
extern crate rbdc;

use once_cell::sync::Lazy;
use rbatis::RBatis;
use salvo::prelude::*;
use serde::{Serialize, Deserialize};
use rbdc_mysql::driver::MysqlDriver;
pub static RB: Lazy<RBatis> = Lazy::new(|| RBatis::new());

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Users {
    pub id: i64,
    pub username: String,
    pub password: String,
}

async fn init_filesystem() {
    tracing_subscriber::fmt().init();

    // mysql connect info
    let mysql_uri = "mysql://root:1234@127.0.0.1:3306/bs_desktop?characterEncoding=utf-8&serverTimezone=UTC&useSSL=false&allowPublicKeyRetrieval=true";
    RB.link(MysqlDriver {}, mysql_uri).await.unwrap();

    // router
    let router = init_router();

    tracing::info!("Listening on http://127.0.0.1:5800");
    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
#[tokio::main]
async fn main() {
    init_nacos_service().await;
    init_filesystem().await;
}
