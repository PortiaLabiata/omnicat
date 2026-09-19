pub mod error;

use std::future::Future;
use error::*;

#[derive(Default)]
pub struct CreationHandle {
    path: String,
}

pub trait Creatable: Sized {
    fn create(handle: CreationHandle)
        -> impl Future<Output = Result<Self, Error>>;
}

#[ambassador::delegatable_trait]
pub trait Transport: Sized {
    fn send(self, data: &[u8], timeout: u32)
        -> impl Future<Output = (Self, Result<usize, Error>)>;

    fn receive(self, receive: &mut [u8], timeout: u32)
        -> impl Future<Output = (Self, Result<usize, Error>)>;
}

