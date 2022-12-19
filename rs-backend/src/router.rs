use axum::{
    extract,
    http::StatusCode,
    response,
    routing::{self, Router},
};
use futures::{future::join_all, TryFutureExt};
use std::{collections::HashMap, net::Ipv4Addr, sync::Arc, time::Duration};
use tokio::sync::RwLock;

use serde::{Deserialize, Serialize};
use std::convert::From;
use std::net::IpAddr;
use uuid::Uuid;
type UserData = Arc<RwLock<HashMap<String, Device>>>;

#[derive(Serialize, Deserialize)]
struct Device {
    name: String,
    ipaddr: IpAddr,
    user: String,
    password: String,
}

impl Device {
    async fn online(&self) -> bool {
        let pinger = tokio_icmp_echo::Pinger::new()
            .and_then(
                |ping| async move { Ok(ping.ping(self.ipaddr, 0, 0, Duration::from_secs(1))) },
            )
            .and_then(|ping| async move { ping.await });
        if let Ok(timeout) = pinger.await {
            return timeout.is_some();
        } else {
            tracing::warn!("maybe PermissionDenied");
            return false;
        }
    }
}

impl From<DeviceRequest> for Device {
    fn from(req: DeviceRequest) -> Self {
        Device {
            name: req.name,
            ipaddr: req
                .ipaddr
                .parse()
                .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            user: req.user,
            password: req.password,
        }
    }
}

#[derive(Serialize, Debug)]
struct DeviceRespon {
    uuid: String,
    name: String,
    ipaddr: String,
    online: bool,
}

#[derive(Deserialize, Debug)]
struct DeviceRequest {
    name: String,
    ipaddr: String,
    user: String,
    password: String,
}

pub fn init_resource_router() -> Router {
    let user_data: UserData = Arc::new(RwLock::new(HashMap::new()));
    Router::new()
        .route("/", routing::get(get_devices))
        .route("/", routing::post(set_devices))
        .route("/", routing::delete(remove_devices))
        .with_state(user_data)
}
#[derive(Deserialize)]
struct DeviceRemove {
    uuid: String,
}
async fn remove_devices(
    extract::State(user_data): extract::State<UserData>,
    extract::Json(payload): extract::Json<DeviceRemove>,
) -> StatusCode {
    let mut devices = user_data.write().await;
    if devices.contains_key(&payload.uuid) {
        devices.remove(&payload.uuid);
    }
    StatusCode::OK
}

async fn set_devices(
    extract::State(user_data): extract::State<UserData>,
    extract::Json(payload): extract::Json<DeviceRequest>,
) -> StatusCode {
    tracing::info!("set_device: {:?}", payload);
    let mut devices = user_data.write().await;
    devices.insert(Uuid::new_v4().to_string(), payload.into());
    StatusCode::OK
}

async fn get_devices(
    extract::State(user_data): extract::State<UserData>,
) -> response::Json<Vec<DeviceRespon>> {
    let devices = user_data.read().await;
    let mut data = devices
        .iter()
        .map(|(uuid, device)| DeviceRespon {
            uuid: uuid.clone(),
            name: device.name.clone(),
            ipaddr: device.ipaddr.to_string(),
            online: false,
        })
        .collect::<Vec<_>>();

    let onlines = join_all(data.iter().map(|device| devices[&device.uuid].online())).await;
    for (index, device) in data.iter_mut().enumerate() {
        device.online = onlines[index]
    }
    tracing::info!("get_device: {:?}", data);
    response::Json(data)
}
