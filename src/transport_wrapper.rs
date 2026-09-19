use serde::Deserialize;

use stdio::*;
use transport::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TransportKind {
    Stdio,
}

pub struct TransportStruct {
    pub name: String,
    pub to: Vec<String>,
    pub transport: Box<dyn Transport>,
}

impl std::cmp::PartialEq for TransportStruct {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name; 
    }
}

impl std::cmp::Eq for TransportStruct {} 

impl std::hash::Hash for TransportStruct {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
