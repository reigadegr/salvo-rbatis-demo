mod config;
mod research;

use config::router::init_router;

#[macro_use]
extern crate rbatis;
extern crate rbdc;

use once_cell::sync::Lazy;
use rbatis::RBatis;
use salvo::prelude::*;
use serde::{Serialize, Deserialize};
use rbdc_mysql::driver::MysqlDriver;

pub static RB: Lazy<RBatis> = Lazy::new(|| RBatis::new());

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Users {
    pub id: i64,
    pub username: String,
    pub password: String,
}

#[tokio::main]
async fn main() {
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
