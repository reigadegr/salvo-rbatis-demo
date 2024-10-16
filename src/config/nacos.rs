use nacos_sdk::api::constants;
use nacos_sdk::api::naming::{
    NamingChangeEvent, NamingEventListener, NamingService, NamingServiceBuilder, ServiceInstance,
};
use nacos_sdk::api::props::ClientProps;
use std::sync::{Arc, LazyLock};

const SERVER_NAME: &str = "users-service";
const USERNAME: &str = "admin";
const PASSWORD: &str = "admin";

const SERVER_ADDR: &str = "127.0.0.1";
const IP_ADDR: &str = "127.0.0.1";
const PORT: i32 = 5800;

pub struct MyNamingEventListener;

impl NamingEventListener for MyNamingEventListener {
    fn event(&self, event: Arc<NamingChangeEvent>) {
        tracing::info!("subscriber notify: {:?}", event);
    }
}

static CLIENT_PROPS: LazyLock<ClientProps> = LazyLock::new(|| {
    ClientProps::new()
        .server_addr(SERVER_ADDR)
        .namespace("")
        .app_name(SERVER_NAME)
        .auth_username(USERNAME)
        .auth_password(PASSWORD)
});
static NAMING_SERVICE: LazyLock<Box<dyn NamingService>> = LazyLock::new(|| {
    let naming_service = NamingServiceBuilder::new(CLIENT_PROPS.clone())
        .enable_auth_plugin_http()
        .build()
        .unwrap();
    Box::new(naming_service)
});

pub async fn init_nacos_service() {
    let listener = Arc::new(MyNamingEventListener);
    let _subscribe_ret = NAMING_SERVICE
        .subscribe(
            SERVER_NAME.to_string(),
            Some(constants::DEFAULT_GROUP.to_string()),
            Vec::default(),
            listener,
        )
        .await;

    let service_instance1 = ServiceInstance {
        ip: IP_ADDR.to_string(),
        port: PORT,
        ..Default::default()
    };

    let _register_instance_ret = NAMING_SERVICE
        .batch_register_instance(
            SERVER_NAME.to_string(),
            Some(constants::DEFAULT_GROUP.to_string()),
            vec![service_instance1],
        )
        .await;
}
