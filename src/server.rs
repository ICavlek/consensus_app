use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::thread;

use tendermint_abci::{Application, Error};
use tracing::{error, info};

use crate::application::RequestDispatcher;
use crate::codec::ServerCodec;

pub const DEFAULT_SERVER_READ_BUF_SIZE: usize = 1024 * 1024;

pub struct ServerBuilder {
    read_buf_size: usize,
}

impl ServerBuilder {
    pub fn new(read_buf_size: usize) -> Self {
        Self { read_buf_size }
    }

    pub fn bind<Addr, App>(self, addr: Addr, app: App) -> Result<Server<App>, Error>
    where
        Addr: ToSocketAddrs,
        App: Application,
    {
        let listener = TcpListener::bind(addr).map_err(Error::io)?;
        let local_addr = listener.local_addr().map_err(Error::io)?.to_string();
        info!("ABCI server running at {}", local_addr);
        Ok(Server {
            app,
            listener,
            local_addr,
            read_buf_size: self.read_buf_size,
        })
    }
}

pub struct Server<App> {
    app: App,
    listener: TcpListener,
    local_addr: String,
    read_buf_size: usize,
}

impl<App: Application> Server<App> {
    pub fn listen(self) -> Result<(), Error> {
        loop {
            let (stream, addr) = self.listener.accept().map_err(Error::io)?;
            let addr = addr.to_string();
            info!("Incoming connection from: {}", addr);
            self.spawn_client_handler(stream, addr);
        }
    }

    pub fn local_addr(&self) -> String {
        self.local_addr.clone()
    }

    fn spawn_client_handler(&self, stream: TcpStream, addr: String) {
        let app = self.app.clone();
        let read_buf_size = self.read_buf_size;
        let _ = thread::spawn(move || Self::handle_client(stream, addr, app, read_buf_size));
    }

    fn handle_client(stream: TcpStream, addr: String, app: App, read_buf_size: usize) {
        let mut codec = ServerCodec::new(stream, read_buf_size);
        loop {
            let request = match codec.next() {
                Some(result) => match result {
                    Ok(r) => r,
                    Err(e) => {
                        error!(
                            "Failed to read incoming request from client {}: {:?}",
                            addr, e
                        );
                        return;
                    }
                },
                None => {
                    info!("Client {} terminated stream", addr);
                    return;
                }
            };
            let response = app.handle(request);
            if let Err(e) = codec.send(response) {
                error!("Failed sending response to client {}: {:?}", addr, e);
                return;
            }
        }
    }
}
