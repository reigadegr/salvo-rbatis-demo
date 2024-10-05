use crate::config::router::init_router;
use rbdc_mysql::MysqlDriver;
use salvo::conn::TcpListener;
use salvo::{Listener, Server};

use once_cell::sync::Lazy;
use rbatis::RBatis;
pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

pub async fn init_filesystem() {
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
