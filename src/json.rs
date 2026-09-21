use serde::Deserialize;
use crate::transport_wrapper::*;
use crate::id;

use stdio::*;
use transport::*;

#[derive(Deserialize)]
pub struct JsonConfig {
    pub name: String,
    pub kind: TransportKind,

    #[serde(default = "default_rxbuf")]
    pub rxbuf_size: usize,

    #[serde(default = "default_txbuf")]
    pub txbuf_size: usize,

    pub to: Vec<String>,
}

fn default_rxbuf() -> usize {
    return 1500;
}

fn default_txbuf() -> usize {
    return 1500;
}

#[derive(Deserialize)]
pub struct JsonConfigs {
    pub configs: Vec<JsonConfig>
}

async fn create_stdio(_cfg: &JsonConfig) -> Result<TransportStdio, ()> {
    let handle = CreationHandle {
        id: id::get(),
        ..Default::default()
    };

    return TransportStdio::create(handle)
        .await
        .map_err(|_| ());
} 

pub async fn create_transport(cfg: &JsonConfig) -> Option<()> {
    match cfg.kind {
        TransportKind::Stdio => {
            let t = match create_stdio(cfg)
                .await
                .map(|v| { 
                    TransportEnum::Stdio(v)
                })
            {
                Ok(t) => t,
                Err(_) => return Some(()),
            };

            id::register_transport(t);
            None
        }
    }
}
