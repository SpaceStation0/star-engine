use tokio::net::{TcpListener, TcpStream};
use std::net::{SocketAddr, Shutdown, IpAddr};
use std::collections::HashMap;
use bytes::{BytesMut};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use std::task::Poll;
use tokio::net::tcp::split::{ReadHalf, WriteHalf};
use tokio::prelude::*;
use futures::future;
use std::str::FromStr;

pub struct Message {
    pub bytes: BytesMut
}

/// A wrapper type that maps clients to their address and the channel
/// to communicate with them.
pub type ClientMap = HashMap<ClientID, (SocketAddr, UnboundedSender<Message>, UnboundedReceiver<Message>)>;

/// A wrapper type that puts a client map in an arc mutex pointer so
/// it can be accessed and modified by multiple threads.
pub type SharedClientMap = Arc<Mutex<ClientMap>>;

/// A client identifier number, used to represent the UID (Unique Identifier) for each client.
pub type ClientID = u32;

pub type ClientMessages = HashMap<ClientID, Vec<Message>>;

/// A client future that processes a client connection and
/// communicates with a server.
pub struct Client {
    socket: TcpStream,
    id: ClientID,
    server_tx: UnboundedSender<Message>,
    server_rx: UnboundedReceiver<Message>,
    shared_client_map: SharedClientMap,
}

pub trait ClientMessageCodec {
    type Output;
    fn process_messages(&mut self, client_messages: ClientMessages) -> Self::Output;
}

pub struct BlankCodec;

impl ClientMessageCodec for BlankCodec {
    type Output = ();
    fn process_messages(&mut self, _: ClientMessages) {}
}

pub struct Server<C, M>
where C: ClientMessageCodec<Output=M> + Send + 'static, M: 'static {
    shared_client_map: SharedClientMap,
    codec: C
}

impl<C, M> Server<C, M>
where C: ClientMessageCodec<Output=M> + Send + 'static, M: 'static {
    pub fn new(codec: C) -> Server<C, M> {
        Server {
            codec,
            shared_client_map: Arc::new(Mutex::new(HashMap::new()))
        }
    }
    pub fn start(mut self) {
        tokio::runtime::Runtime::new().unwrap().block_on(
            async move {
                let _ = self.serve().await;
            }
        );
    }
}

impl<C, M> Server<C, M>
where C: ClientMessageCodec<Output=M> + Send + 'static, M: 'static {
    async fn serve(&mut self) -> Result<(), std::io::Error> {
        let mut client_nonce: ClientID = 0;
        let mut listener = TcpListener::bind(
            SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), 4343)).await?;
        while let (stream, address) = listener.accept().await? {
            let id = client_nonce;
            let (tx, rx) = unbounded_channel();
            let (tx2, rx2) = unbounded_channel();
            self.insert_client(id, address, tx, rx2);
            let client_map = self.shared_client_map.clone();
            tokio::spawn(async move {
                Client::new(stream, id, tx2, rx, client_map)
                    .process().await;
            });

        }

        Ok(())
    }
    fn insert_client(&mut self,
                     id: ClientID,
                     address: SocketAddr,
                     tx: UnboundedSender<Message>,
                     rx: UnboundedReceiver<Message>)
    {
        self.shared_client_map.lock().expect("To get a lock on the shared client map")
            .insert(id, (address, tx, rx));
    }
}


impl Client {
    fn new(socket: TcpStream,
               id: ClientID,
               server_tx: UnboundedSender<Message>,
               server_rx: UnboundedReceiver<Message>,
               shared_client_map: SharedClientMap) -> Client
    {
        Client {
            socket,
            id,
            server_tx,
            server_rx,
            shared_client_map
        }
    }
}

impl Client {
    const BUFFER_SIZE: usize = 4096;
    async fn read(mut r_socket: ReadHalf<'_>, mut server_tx: UnboundedSender<Message>) -> Result<(), std::io::Error> {
        loop {
            let mut buffer = BytesMut::with_capacity(Client::BUFFER_SIZE);
            println!("Reading");
            r_socket.read(&mut buffer).await?;
            if buffer.iter().zip([0 as u8; 4096].iter()).all(|(a, b)| a == b) {
                break;
            }
            server_tx.send(Message { bytes: buffer }).await;
        }
        Ok(())
    }
    async fn write(mut w_socket: WriteHalf<'_>, mut server_rx: UnboundedReceiver<Message>) -> Result<(), std::io::Error> {
        while let Some(Message{ bytes }) = server_rx.recv().await {
            w_socket.write(&bytes).await?;
        }
        Ok(())
    }
    async fn process(mut self) {

        let (mut r_socket, mut w_socket) = self.socket.split();
        let _ = future::join(
            Client::read(r_socket, self.server_tx),
            Client::write(w_socket, self.server_rx)
        ).await;

        // The client has exited, so remove their information from the client map
        self.shared_client_map.lock().expect("To get a lock on the shared client map")
            .remove(&self.id);
        println!("Client with ID {} has disconnected!", self.id);
    }
}