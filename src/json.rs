use serde::Deserialize;
use crate::transport_wrapper::*;
use stdio::*;
use transport::*;

#[derive(Deserialize)]
pub struct JsonConfig {
    pub name: String,
    pub kind: TransportKind,
    pub to: Vec<String>,
}

#[derive(Deserialize)]
pub struct JsonConfigs {
    pub configs: Vec<JsonConfig>
}

async fn create_stdio(_cfg: &JsonConfig) -> Result<TransportStdio, ()> {
    return TransportStdio::create(StdioHandle {})
        .await
        .map_err(|_| ());
} 

pub async fn create_transport(cfg: &JsonConfig) -> Result<TransportStruct, ()> {
    match cfg.kind {
        TransportKind::Stdio => {
            return create_stdio(cfg)
                .await
                .map(|v| { 
                    let w = TransportWrapper::Stdio(v);
                    TransportStruct {
                        name: cfg.name.clone(),
                        to: cfg.to.clone(),
                        wrapper: w,
                    }
                });
        }
    }
}
