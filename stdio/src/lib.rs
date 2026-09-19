use transport::*;
use transport::error::*;
use tokio::io::{
    self,
    AsyncWriteExt,
    AsyncReadExt,
};

pub struct TransportStdio {
    stdin: io::Stdin,
    stdout: io::Stdout,
}

impl Transport for TransportStdio {
    async fn create(_handle: CreationHandle) 
        -> Result<Self, Error> 
    {
        let stdin = io::stdin();
        let stdout = io::stdout();

        return Ok(TransportStdio {
            stdin,
            stdout,
        });
    }

    async fn send(self, data: &[u8]) 
        -> (Self, Result<usize, Error>)
    {
        let mut stdout = self.stdout;
        let stdin = self.stdin;

        let res = match stdout.write(data).await {
            Ok(r) => Ok(r),
            Err(e) => Err(Error::new(
                "Failed to read from stdin, somehow",
                e,
            )),
        };

        return (Self {
            stdin, stdout
        }, res);
    }
    
    async fn receive(self, data: &mut [u8]) 
        -> (Self, Result<usize, Error>) 
    {
        let stdout = self.stdout;
        let mut stdin = self.stdin;

        let res = match stdin.read(data).await {
            Ok(r) => Ok(r),
            Err(e) => Err(Error::new(
                "Failed to read from stdin, somehow",
                e,
            )),
        };

        return (Self {
            stdin, stdout
        }, res);
    }
}
