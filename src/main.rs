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
use nacos_sdk::api::constants;
use nacos_sdk::api::naming::{
    NamingChangeEvent, NamingEventListener, NamingService, NamingServiceBuilder, ServiceInstance,
};
use nacos_sdk::api::props::ClientProps;
use std::sync::{Arc, LazyLock};

pub struct MyNamingEventListener;

impl NamingEventListener for MyNamingEventListener {
    fn event(&self, event: Arc<NamingChangeEvent>) {
        tracing::info!("subscriber notify: {:?}", event);
    }
}
pub static RB: Lazy<RBatis> = Lazy::new(|| RBatis::new());

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Users {
    pub id: i64,
    pub username: String,
    pub password: String,
}

async fn init_filesystem(){
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

static CLIENT_PROPS: LazyLock<ClientProps> = LazyLock::new(|| {
    ClientProps::new()
        .server_addr(constants::DEFAULT_SERVER_ADDR)
        .namespace("")
        .app_name("lazy_app")
        .auth_username("admin")
        .auth_password("admin")
});
static NAMING_SERVICE: LazyLock<Box<dyn NamingService>> = LazyLock::new(|| {
    let naming_service = NamingServiceBuilder::new(CLIENT_PROPS.clone())
        .enable_auth_plugin_http()
        .build()
        .unwrap();
    Box::new(naming_service)
});

async fn init_nacos_service() {
    let listener = std::sync::Arc::new(MyNamingEventListener);
    let _subscribe_ret = NAMING_SERVICE
        .subscribe(
            "salvo-rbatis-demo".to_string(),
            Some(constants::DEFAULT_GROUP.to_string()),
            Vec::default(),
            listener,
        ).await;

    let service_instance1 = ServiceInstance {
        ip: "127.0.0.1".to_string(),
        port: 5800,
        ..Default::default()
    };

    let _register_instance_ret = NAMING_SERVICE
        .batch_register_instance(
            "salvo-rbatis-demo".to_string(),
            Some(constants::DEFAULT_GROUP.to_string()),
            vec![service_instance1],
        ).await;
}
#[tokio::main]
async fn main() {
init_nacos_service().await;
    init_filesystem().await;
}
