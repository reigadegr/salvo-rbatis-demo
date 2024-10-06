use crate::config::router::init_router;
use rbdc_mysql::MysqlDriver;
use salvo::conn::TcpListener;
use salvo::{Listener, Server};

use once_cell::sync::Lazy;
use rbatis::RBatis;
// use salvo_core::prelude::*;
pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

pub async fn init_salvo_framework() {
    tracing_subscriber::fmt().init();

    // mysql connect info
    let mysql_uri = "mysql://root:1234@127.0.0.1:3306/bs_desktop?characterEncoding=utf-8&serverTimezone=UTC&useSSL=false&allowPublicKeyRetrieval=true";
    RB.link(MysqlDriver {}, mysql_uri).await.unwrap();

    // router
    let router = init_router();

    tracing::info!("Listening on http://127.0.0.1:5800");
    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    let server = Server::new(acceptor);
    // 优雅地关闭服务器
    // let handle = server.handle();
    // tokio::spawn(async move {
    //     tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    //     handle.stop_graceful(None);
    // });
    
    server.serve(router).await;
}
