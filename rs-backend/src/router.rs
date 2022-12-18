use axum:: {
    routing::{self, Router},
    response, extract,
    http::StatusCode,
};
use std::{sync::Arc, collections::HashMap};
use tokio::sync::Mutex;
use std::net::IpAddr;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
type UserData = Arc<Mutex<HashMap<String, Device>>>;

#[derive(Serialize, Deserialize)]
struct Device{
    name: String,
    ipaddr: IpAddr,
    password: String,
    online: bool,
}
#[derive(Serialize, Debug)]
struct DeviceRespon{
    uuid: String,
    name: String,
    ipaddr: String,
    online: bool,
}

#[derive(Deserialize, Debug)]
struct DeviceRequest {
    name: String,
    ipaddr: String,
    password: String,
}

pub fn init_resource_router() -> Router {
    let user_data:UserData = Arc::new(Mutex::new(HashMap::new()));
    Router::new()
        .route("/", routing::get(get_devices))
        .route("/", routing::post(set_devices))
        .with_state(user_data)
}

async fn set_devices(extract::State(user_data): extract::State<UserData>,extract::Json(payload): extract::Json<DeviceRequest>) -> StatusCode {
    tracing::info!("set_device: {:?}", payload);
    let mut devices = user_data.lock().await;
    devices.insert(Uuid::new_v4().to_string(), Device { name: payload.name, ipaddr: payload.ipaddr.parse().unwrap(), password: payload.password, online: false });
    StatusCode::OK
}

async fn get_devices(extract::State(user_data): extract::State<UserData>,) -> response::Json<Vec<DeviceRespon>> {
    let data = user_data.lock().await;
    let data = data.iter().map(|(uuid, device)| {
        DeviceRespon{
            uuid: uuid.clone(),
            name: device.name.clone(),
            ipaddr: device.ipaddr.to_string(),
            online: device.online,
        }
    }).collect::<Vec<_>>();
    tracing::info!("get_device: {:?}", data);
    response::Json(data)
}