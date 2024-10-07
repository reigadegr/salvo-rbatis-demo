use crate::config::router::init_router;
use once_cell::sync::Lazy;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use salvo::conn::rustls::Keycert;
use salvo::conn::rustls::RustlsConfig;
use salvo::conn::QuinnListener;
use salvo::conn::TcpListener;
use salvo::{Listener, Server};
// use salvo_core::prelude::*;
pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

/**
async fn main() {
    tracing_subscriber::fmt().init();
    let cert = include_bytes!("../certs/cert.pem").to_vec();
    let key = include_bytes!("../certs/key.pem").to_vec();

    let router = Router::new().get(hello);
    let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));
    let listener = TcpListener::new(("127.0.0.1", 5800)).rustls(config.clone());

    let acceptor = QuinnListener::new(config, ("127.0.0.1", 5800))
        .join(listener)
        .bind()
        .await;

    Server::new(acceptor).serve(router).await;
}

**/
pub async fn init_salvo_framework() {
    tracing_subscriber::fmt().init();
    let cert = include_bytes!("../../cert/cert.pem").to_vec();
    let key = include_bytes!("../../cert/key.pem").to_vec();
    let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));
    // mysql connect info
    let mysql_uri = "mysql://root:1234@127.0.0.1:3306/bs_desktop?characterEncoding=utf-8&serverTimezone=UTC&useSSL=false&allowPublicKeyRetrieval=true";
    RB.link(MysqlDriver {}, mysql_uri).await.unwrap();

    // router
    let router = init_router();

    tracing::info!("Listening on http://127.0.0.1:5800");
    let listener = TcpListener::new(("0.0.0.0", 5800)).rustls(config.clone());
    let acceptor = QuinnListener::new(config, "0.0.0.0:5800")
        .join(listener)
        .bind()
        .await;
    let server = Server::new(acceptor);
    // 优雅地关闭服务器
    // let handle = server.handle();
    // tokio::spawn(async move {
    //     tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    //     handle.stop_graceful(None);
    // });

    server.serve(router).await;
}
