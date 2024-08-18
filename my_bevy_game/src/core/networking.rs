use async_net::{AsyncToSocketAddrs, TcpListener, TcpStream};
use async_tungstenite::tungstenite::Message;
use async_tungstenite::WebSocketStream;
use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task};
use crossbeam_channel::{Receiver, Sender};
use futures::{pin_mut, select, FutureExt, SinkExt, StreamExt};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        let (ws_tx, ws_rx) = crossbeam_channel::unbounded();
        app.insert_resource(WsListener::new(ws_tx));
        app.insert_resource(WsAcceptQueue { ws_rx });
        app.add_systems(Startup, startup);
        app.add_systems(Update, accept_ws_from_queue);
        app.add_systems(Update, receive_message);
    }
}

fn startup(listener: Res<WsListener>) {
    listener.listen("127.0.0.1:8080");
}

fn receive_message(mut commands: Commands, connections: Query<(Entity, &WsConnection)>) {
    for (entity, conn) in connections.iter() {
        loop {
            match conn.receive() {
                Ok(message) => {
                    conn.send(message);
                }
                Err(ReceiveError::Empty) => break,
                Err(ReceiveError::Closed) => {
                    commands.entity(entity).despawn();
                    break;
                }
            }
        }
    }
}

#[derive(Resource)]
pub struct WsListener {
    ws_tx: Sender<WebSocketStream<TcpStream>>,
}

#[derive(Resource)]
pub struct WsAcceptQueue {
    ws_rx: Receiver<WebSocketStream<TcpStream>>,
}

impl WsListener {
    pub fn new(ws_tx: Sender<WebSocketStream<TcpStream>>) -> Self {
        Self { ws_tx }
    }

    pub fn listen(&self, bind_to: impl AsyncToSocketAddrs) {
        let listener = futures::executor::block_on(TcpListener::bind(bind_to))
            .expect("cannot bind to the address");

        let task_pool = IoTaskPool::get();
        let ws_tx = self.ws_tx.clone();
        let task = task_pool.spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        println!("new connection from {}", addr);
                        let ws_tx = ws_tx.clone();
                        let accept = async move {
                            match async_tungstenite::accept_async(stream).await {
                                Ok(websocket) => {
                                    // Ignoring error is ok because then WsResource is not present,
                                    // thus there is no need for accepting a new websocket.
                                    let _ = ws_tx.send(websocket);
                                }
                                Err(e) => {
                                    println!("error handshaking a new websocket: {}", e);
                                }
                            }
                        };
                        task_pool.spawn(accept).detach();
                    }
                    Err(e) => {
                        println!("error accepting a new connection: {}", e);
                    }
                }
            }
        });

        task.detach();
    }
}

#[derive(Component)]
pub struct WsConnection {
    _io: Task<()>,
    sender: async_channel::Sender<Message>,
    receiver: async_channel::Receiver<Message>,
}

pub use async_channel::TryRecvError as ReceiveError;
impl WsConnection {
    pub fn send(&self, message: Message) -> bool {
        self.sender.try_send(message).is_ok()
    }

    pub fn receive(&self) -> Result<Message, ReceiveError> {
        self.receiver.try_recv()
    }
}

pub fn accept_ws_from_queue(mut commands: Commands, queue: ResMut<WsAcceptQueue>) {
    for mut websocket in queue.ws_rx.try_iter() {
        let (message_tx, io_message_rx) = async_channel::unbounded::<Message>();
        let (io_message_tx, message_rx) = async_channel::unbounded::<Message>();

        let io = IoTaskPool::get().spawn(async move {
            loop {
                let from_channel = io_message_rx.recv().fuse();
                let from_ws = websocket.next().fuse();

                pin_mut!(from_channel, from_ws);

                select! {
                    message = from_channel => if let Ok(message) = message {
                        let _ =  websocket.send(message).await;
                    } else {
                        break;
                    },
                    message = from_ws => if let Some(Ok(message)) = message {
                        let _ = io_message_tx.send(message).await;
                    } else {
                        break;
                    },
                    complete => break,
                }
            }
        });
        commands.spawn(WsConnection {
            _io: io,
            sender: message_tx,
            receiver: message_rx,
        });
    }
}
