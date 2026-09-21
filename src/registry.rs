use crate::wrapper::{
    TransportEnum,
    TransportHandle,
};
use transport::*;

struct RegistryEntry {
    handle: TransportHandle,
    transport: Option<TransportEnum>,
}

type TransportRegistry = Vec<RegistryEntry>;

pub fn init(n: usize) -> TransportRegistry {
    return Vec::with_capacity(n);
}

pub fn set_handle(id: u32, h: TransportHandle, r: &mut TransportRegistry) {
    r.get_mut(id as usize).unwrap().handle = h;
}

pub fn put(t: TransportEnum, r: &mut TransportRegistry) {
    let id = t.id();
    r
        .get_mut(id as usize)
        .unwrap()
        .transport = Some(t);
}

pub fn take(id: u32, r: &mut TransportRegistry) -> Option<TransportEnum> {
    return r
        .get_mut(id as usize)
        .unwrap()
        .transport
        .take();
}
