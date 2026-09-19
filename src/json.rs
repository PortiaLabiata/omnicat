use serde::Deserialize;
use crate::transport_wrapper::*;
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
    return TransportStdio::create(CreationHandle::default())
        .await
        .map_err(|_| ());
} 

pub async fn create_transport(cfg: &JsonConfig) -> Result<TransportStruct, ()> {
    match cfg.kind {
        TransportKind::Stdio => {
            return create_stdio(cfg)
                .await
                .map(|v| { 
                    TransportStruct {
                        name: cfg.name.clone(),
                        rxbuf: Vec::with_capacity(cfg.rxbuf_size),
                        txbuf: Vec::with_capacity(cfg.txbuf_size),
                        to: cfg.to.clone(),
                        transport: TransportEnum::Stdio(v),
                    }
                });
        }
    }
}
