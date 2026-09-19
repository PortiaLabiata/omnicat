pub mod error;

use crate::error::StdioError;
use transport::*;
use tokio::io::{
    self,
    AsyncWriteExt,
    AsyncReadExt,
};

pub struct TransportStdio {
    stdin: io::Stdin,
    stdout: io::Stdout,
}

pub struct StdioHandle {}

impl CreationHandle for StdioHandle {}

impl Transport<StdioError, StdioHandle> for TransportStdio {
    async fn create(_handle: StdioHandle) -> Result<Self, StdioError> {
        let stdin = io::stdin();
        let stdout = io::stdout();

        return Ok(TransportStdio {
            stdin,
            stdout,
        });
    }

    async fn send(self, data: &[u8], _timeout: u32) 
        -> (Self, Result<usize, StdioError>)
    {
        let mut stdout = self.stdout;
        let stdin = self.stdin;

        let res = match stdout.write(data).await {
            Ok(r) => Ok(r),
            Err(e) => Err(StdioError::new(
                "Failed to read from stdin, somehow",
                e,
            )),
        };

        return (Self {
            stdin, stdout
        }, res);
    }
    
    async fn receive(self, data: &mut [u8], _timeout: u32) 
        -> (Self, Result<usize, StdioError>) 
    {
        let stdout = self.stdout;
        let mut stdin = self.stdin;

        let res = match stdin.read(data).await {
            Ok(r) => Ok(r),
            Err(e) => Err(StdioError::new(
                "Failed to read from stdin, somehow",
                e,
            )),
        };

        return (Self {
            stdin, stdout
        }, res);
    }
}
