use crate::filesharing::network::Client;
use async_std::io;
use async_std::io::{BufReader, Stdin};
use async_std::task::spawn;
use clap::Parser;
use futures::channel::mpsc::Receiver;
use futures::io::Lines;
use futures::prelude::*;
use futures::stream::Fuse;
use libp2p::core::{Multiaddr, PeerId};
use libp2p::multiaddr::Protocol;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

#[async_std::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let (mut network_client, mut network_events, mut network_event_loop) = network::new().await?;

    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    spawn(network_event_loop.run());

    network_client
        .start_listening("/ip4/0.0.0.0/tcp/0".parse()?)
        .await
        .expect("Listening not to fail.");

    spawn(run(network_events, network_client.clone()));

    spawn(network_client.run(stdin));

    // In case the user provided an address of a peer on the CLI, dial it.
    // if let Some(addr) = opt.peer {
    //     let peer_id = match addr.iter().last() {
    //         Some(Protocol::P2p(hash)) => PeerId::from_multihash(hash).expect("Valid hash."),
    //         _ => return Err("Expect peer multiaddr to contain peer ID.".into()),
    //     };
    //     network_client
    //         .dial(peer_id, addr)
    //         .await
    //         .expect("Dial to succeed");
    // }

    // let opt = Opt::parse();
    //
    // match opt.argument {
    //     // Providing a file.
    //     CliArgument::Provide { path, name } => {
    //         // Advertise oneself as a provider of the file on the DHT.
    //         network_client.start_providing(name.clone()).await;
    //
    //         loop {
    //             match network_events.next().await {
    //                 // Reply with the content of the file on incoming requests.
    //                 Some(network::Event::InboundRequest { request, channel }) => {
    //                     if request == name {
    //                         network_client
    //                             .respond_file(std::fs::read(&path)?, channel)
    //                             .await;
    //                     }
    //                 }
    //                 e => todo!("{:?}", e),
    //             }
    //         }
    //     }
    //     // Locating and getting a file.
    //     CliArgument::Get { name } => {
    //         // Locate all nodes providing the file.
    //         let providers = network_client.get_providers(name.clone()).await;
    //         if providers.is_empty() {
    //             return Err(format!("Could not find provider for file {name}.").into());
    //         }
    //
    //         // Request the content of the file from each node.
    //         let requests = providers.into_iter().map(|p| {
    //             let mut network_client = network_client.clone();
    //             let name = name.clone();
    //             async move { network_client.request_file(p, name).await }.boxed()
    //         });
    //
    //         // Await the requests, ignore the remaining once a single one succeeds.
    //         let file_content = futures::future::select_ok(requests)
    //             .await
    //             .map_err(|_| "None of the providers returned file.")?
    //             .0;
    //
    //         //TODO: change it (logic when we receive file).
    //         std::io::stdout().write_all(&file_content)?;
    //     }
    // }

    loop {}

    Ok(())
}

pub async fn run(mut network_events: Receiver<network::Event>, mut network_client: Client) {
    loop {
        match network_events.next().await {
            // Reply with the content of the file on incoming requests.
            Some(network::Event::InboundRequest { request, channel }) => {
                network_client
                    .respond_file(std::fs::read(&request).expect("panic"), channel)
                    .await;
            }
            e => todo!("{:?}", e),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(name = "libp2p file sharing example")]
struct Opt {
    #[clap(long)]
    peer: Option<Multiaddr>,

    #[clap(subcommand)]
    argument: CliArgument,
}

#[derive(Debug, Parser)]
enum CliArgument {
    Provide {
        #[clap(long)]
        path: PathBuf,
        #[clap(long)]
        name: String,
    },
    Get {
        #[clap(long)]
        name: String,
    },
}

/// The network module, encapsulating all network related logic.
mod network {
    use super::*;
    use crate::filesharing::network::Event::InboundRequest;
    use async_std::io::{BufReader, Stdin};
    use async_trait::async_trait;
    use either::Either;
    use futures::channel::mpsc::Receiver;
    use futures::channel::{mpsc, oneshot};
    use futures::io::Lines;
    use futures::stream::Fuse;
    use libp2p::core::either::EitherError;
    use libp2p::core::upgrade::{read_length_prefixed, write_length_prefixed, ProtocolName};
    use libp2p::identity::ed25519;
    use libp2p::kad::record::store::MemoryStore;
    use libp2p::kad::{GetProvidersOk, Kademlia, KademliaEvent, QueryId, QueryResult};
    use libp2p::multiaddr::Protocol;
    use libp2p::request_response::{self, ProtocolSupport, RequestId, ResponseChannel};
    use libp2p::swarm::{ConnectionHandlerUpgrErr, NetworkBehaviour, Swarm, SwarmEvent};
    use libp2p::{development_transport, identity};
    use libp2p_kad::record::Key;
    use libp2p_kad::{
        AddProviderOk, GetClosestPeersError, GetRecordOk, PeerRecord, PutRecordOk, Quorum, Record,
    };
    use std::collections::{hash_map, HashMap, HashSet};
    use std::iter;
    use std::str::FromStr;

    /// Creates the network components, namely:
    ///
    /// - The network client to interact with the network layer from anywhere
    ///   within your application.
    ///
    /// - The network event stream, e.g. for incoming requests.
    ///
    /// - The network task driving the network itself.
    pub async fn new() -> Result<(Client, Receiver<Event>, EventLoop), Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let key_copy = local_key.clone();

        let local_peer_id = PeerId::from(local_key.public());
        println!("Local peer id: {local_peer_id:?}");

        let transport = development_transport(local_key).await?;

        // Build the Swarm, connecting the lower layer transport logic with the
        // higher layer network behaviour logic.
        let mut swarm = {
            let store = MemoryStore::new(local_peer_id);
            let kademlia = Kademlia::new(local_peer_id, store);

            //TODO: normal protocol name
            let mut cfg_identify = libp2p_identify::Config::new("a".to_string(), key_copy.public());
            let identify = libp2p_identify::Behaviour::new(cfg_identify);

            let request_response = request_response::RequestResponse::new(
                FileExchangeCodec(),
                iter::once((FileExchangeProtocol(), ProtocolSupport::Full)),
                Default::default(),
            );

            let mut behaviour = MyBehaviour {
                request_response,
                kademlia,
                identify,
            };

            // TODO: take it from arrays.
            behaviour.kademlia.add_address(
                &"12D3KooWBK2e2x7f3x3zhNNbEYXCa9borYZXkYNPUPxFMFCY8Km9".parse()?,
                "/ip4/172.29.71.208/tcp/41959".parse()?,
            );

            //TODO: what executor use
            Swarm::with_async_std_executor(transport, behaviour, local_peer_id)
        };

        swarm
            .behaviour_mut()
            .kademlia
            .bootstrap()
            .expect("Can't bootstrap.");

        let (command_sender, command_receiver) = mpsc::channel(0);
        let (event_sender, event_receiver) = mpsc::channel(0);

        Ok((
            Client {
                sender: command_sender,
            },
            event_receiver,
            EventLoop::new(swarm, command_receiver, event_sender),
        ))
    }

    #[derive(Clone)]
    pub struct Client {
        sender: mpsc::Sender<Command>,
    }

    impl Client {
        /// Listen for incoming connections on the given address.
        pub async fn start_listening(
            &mut self,
            addr: Multiaddr,
        ) -> Result<(), Box<dyn Error + Send>> {
            let (sender, receiver) = oneshot::channel();
            self.sender
                .send(Command::StartListening { addr, sender })
                .await
                .expect("Command receiver not to be dropped.");
            receiver.await.expect("Sender not to be dropped.")
        }

        pub async fn run(mut self, mut stdin: Fuse<Lines<BufReader<Stdin>>>) {
            loop {
                futures::select! {
                    line = stdin.select_next_some() => self.handle_input_line(line.expect("Stdin not to close")).await,
                }
            }
        }

        /// Dial the given peer at the given address.
        pub async fn dial(
            &mut self,
            peer_id: PeerId,
            peer_addr: Multiaddr,
        ) -> Result<(), Box<dyn Error + Send>> {
            let (sender, receiver) = oneshot::channel();
            self.sender
                .send(Command::Dial {
                    peer_id,
                    peer_addr,
                    sender,
                })
                .await
                .expect("Command receiver not to be dropped.");
            receiver.await.expect("Sender not to be dropped.")
        }

        /// Advertise the local node as the provider of the given file on the DHT.
        pub async fn start_providing(&mut self, file_name: String) {
            let (sender, receiver) = oneshot::channel();
            self.sender
                .send(Command::StartProviding { file_name, sender })
                .await
                .expect("Command receiver not to be dropped.");
            receiver.await.expect("Sender not to be dropped.");
        }

        /// Find the providers for the given file on the DHT.
        pub async fn get_providers(&mut self, file_name: String) -> HashSet<PeerId> {
            let (sender, receiver) = oneshot::channel();
            self.sender
                .send(Command::GetProviders { file_name, sender })
                .await
                .expect("Command receiver not to be dropped.");
            receiver.await.expect("Sender not to be dropped.")
        }

        /// Request the content of the given file from the given peer.
        pub async fn request_file(
            &mut self,
            peer: PeerId,
            file_name: String,
        ) -> Result<Vec<u8>, Box<dyn Error + Send>> {
            let (sender, receiver) = oneshot::channel();
            self.sender
                .send(Command::RequestFile {
                    file_name,
                    peer,
                    sender,
                })
                .await
                .expect("Command receiver not to be dropped.");
            receiver.await.expect("Sender not be dropped.")
        }

        /// Respond with the provided file content to the given request.
        pub async fn respond_file(
            &mut self,
            file: Vec<u8>,
            channel: ResponseChannel<FileResponse>,
        ) {
            self.sender
                .send(Command::RespondFile { file, channel })
                .await
                .expect("Command receiver not to be dropped.");
        }

        async fn handle_input_line(&mut self, line: String) {
            let mut args = line.split(' ');

            match args.next() {
                //TODO: normal finish.
                Some("EXIT") => {}

                Some("GET") => {
                    let name: String = {
                        match args.next() {
                            Some(name) => String::from(name),
                            None => {
                                eprintln!("Expected name");
                                return;
                            }
                        }
                    };

                    // Locate all nodes providing the file.
                    let providers = self.get_providers(name.clone()).await;
                    if providers.is_empty() {
                        println!("No providers!");
                        // Err(format!("Could not find provider for file {name}.").into()).expect("NO PROVIDERS");
                    }

                    // Request the content of the file from each node.
                    let requests = providers.into_iter().map(|p| {
                        let mut network_client = self.clone();
                        let name = name.clone();
                        async move { network_client.request_file(p, name).await }.boxed()
                    });

                    // Await the requests, ignore the remaining once a single one succeeds.
                    let file_content = futures::future::select_ok(requests)
                        .await
                        .map_err(|_| "None of the providers returned file.")
                        .expect("panic")
                        .0;

                    //TODO: change it (logic when we receive file).
                    std::io::stdout()
                        .write_all(&file_content)
                        .expect("TODO: panic message");
                }

                Some("PUT") => {
                    let name: String = {
                        match args.next() {
                            Some(name) => String::from(name),
                            None => {
                                eprintln!("Expected path");
                                return;
                            }
                        }
                    };

                    self.start_providing(name.clone()).await;
                }

                // Some("PUT_PROVIDER") => {
                //
                // }
                //
                // Some("GET_PROVIDERS") => {
                //     let key = {
                //         match args.next() {
                //             Some(key) => Key::new(&key),
                //             None => {
                //                 eprintln!("Expected key");
                //                 return;
                //             }
                //         }
                //     };
                //     self.swarm.behaviour_mut().kademlia.get_providers(key);
                // }
                //
                // Some("ADDRESS_NODE") => {
                //     let key = {
                //         match args.next() {
                //             Some(key) => PeerId::from_str(key),
                //             None => {
                //                 eprintln!("Expected key");
                //                 return;
                //             }
                //         }
                //     };
                //
                //     for address in self
                //         .swarm
                //         .behaviour_mut()
                //         .kademlia
                //         .addresses_of_peer(&key.unwrap())
                //     {
                //         println!("{address:?}")
                //     }
                // }
                //
                // Some("NODES") => {
                //     self.swarm
                //         .behaviour_mut()
                //         .kademlia
                //         .get_closest_peers(local_peer);
                // }
                _ => {
                    eprintln!(
                        "expected GET, GET_PROVIDERS, NODES, ADDRESS_NODE, PUT or PUT_PROVIDER"
                    );
                }
            }
        }
    }

    pub struct EventLoop {
        swarm: Swarm<MyBehaviour>,
        command_receiver: mpsc::Receiver<Command>,
        event_sender: mpsc::Sender<Event>,
        pending_dial: HashMap<PeerId, oneshot::Sender<Result<(), Box<dyn Error + Send>>>>,
        pending_start_providing: HashMap<QueryId, oneshot::Sender<()>>,
        pending_get_providers: HashMap<QueryId, oneshot::Sender<HashSet<PeerId>>>,
        pending_request_file:
            HashMap<RequestId, oneshot::Sender<Result<Vec<u8>, Box<dyn Error + Send>>>>,
    }

    impl EventLoop {
        fn new(
            swarm: Swarm<MyBehaviour>,
            command_receiver: mpsc::Receiver<Command>,
            event_sender: mpsc::Sender<Event>,
        ) -> Self {
            Self {
                swarm,
                command_receiver,
                event_sender,
                pending_dial: Default::default(),
                pending_start_providing: Default::default(),
                pending_get_providers: Default::default(),
                pending_request_file: Default::default(),
            }
        }

        pub async fn run(mut self) {
            loop {
                futures::select! {
                    event = self.swarm.select_next_some() => self.handle_event(event).await,
                    command = self.command_receiver.next() => match command {
                        Some(c) => self.handle_command(c).await,
                        // Command channel closed, thus shutting down the network event loop.
                        None=>  return,
                    },
                }
            }
        }

        async fn handle_event(
            &mut self,
            event: SwarmEvent<
                ComposedEvent,
                EitherError<EitherError<ConnectionHandlerUpgrErr<io::Error>, io::Error>, io::Error, >,
            >,
        ) {
            match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening in {address:?}");
                }

                SwarmEvent::Behaviour(ComposedEvent::Kademlia(
                    KademliaEvent::OutboundQueryProgressed { result, id, .. },
                )) => match result {
                    QueryResult::StartProviding(_) => {
                        let sender: oneshot::Sender<()> = self
                            .pending_start_providing
                            .remove(&id)
                            .expect("Completed query to be previously pending.");
                        let _ = sender.send(());
                    }

                    QueryResult::GetProviders(Ok(GetProvidersOk::FoundProviders {
                        providers,
                        ..
                    })) => {
                        if let Some(sender) = self.pending_get_providers.remove(&id) {
                            sender.send(providers).expect("Receiver not to be dropped");

                            // Finish the query. We are only interested in the first result.
                            self.swarm
                                .behaviour_mut()
                                .kademlia
                                .query_mut(&id)
                                .unwrap()
                                .finish();
                        }
                    }

                    QueryResult::GetProviders(Ok(
                        GetProvidersOk::FinishedWithNoAdditionalRecord { .. },
                    )) => {}

                    QueryResult::GetClosestPeers(result) => {
                        match result {
                            Ok(ok) => {
                                if !ok.peers.is_empty() {
                                    println!("Query finished with closest peers: {:#?}", ok.peers)
                                } else {
                                    println!("Query finished with no closest peers.")
                                }
                            }
                            Err(GetClosestPeersError::Timeout { peers, .. }) => {
                                if !peers.is_empty() {
                                    println!("Query timed out with closest peers: {peers:#?}")
                                } else {
                                    println!("Query timed out with no closest peers.");
                                }
                            }
                        };
                    }

                    QueryResult::GetProviders(Err(err)) => {
                        eprintln!("Failed to get providers: {err:?}");
                    }

                    QueryResult::GetRecord(Ok(GetRecordOk::FoundRecord(PeerRecord {
                        record: Record { key, value, .. },
                        ..
                    }))) => {
                        println!(
                            "Got record {:?} {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap(),
                            std::str::from_utf8(&value).unwrap(),
                        );
                    }

                    QueryResult::GetRecord(Ok(_)) => {}

                    QueryResult::GetRecord(Err(err)) => {
                        eprintln!("Failed to get record: {err:?}");
                    }

                    QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                        println!(
                            "Successfully put record {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap()
                        );
                    }

                    QueryResult::PutRecord(Err(err)) => {
                        eprintln!("Failed to put record: {err:?}");
                    }
                    _ => {}
                },

                SwarmEvent::Behaviour(ComposedEvent::RequestResponse(
                    request_response::RequestResponseEvent::Message { message, .. },
                )) => match message {
                    request_response::RequestResponseMessage::Request {
                        request, channel, ..
                    } => {
                        self.event_sender
                            .send(Event::InboundRequest {
                                request: request.0,
                                channel,
                            })
                            .await
                            .expect("Event receiver not to be dropped.");
                    }
                    request_response::RequestResponseMessage::Response {
                        request_id,
                        response,
                    } => {
                        let _ = self
                            .pending_request_file
                            .remove(&request_id)
                            .expect("Request to still be pending.")
                            .send(Ok(response.0));
                    }
                },
                SwarmEvent::Behaviour(ComposedEvent::RequestResponse(
                    request_response::RequestResponseEvent::OutboundFailure {
                        request_id,
                        error,
                        ..
                    },
                )) => {
                    let _ = self
                        .pending_request_file
                        .remove(&request_id)
                        .expect("Request to still be pending.")
                        .send(Err(Box::new(error)));
                }

                SwarmEvent::Behaviour(ComposedEvent::RequestResponse(
                    request_response::RequestResponseEvent::ResponseSent { .. },
                )) => {}

                SwarmEvent::IncomingConnection { .. } => {}

                SwarmEvent::ConnectionEstablished {
                    peer_id, endpoint, ..
                } => {
                    if endpoint.is_dialer() {
                        if let Some(sender) = self.pending_dial.remove(&peer_id) {
                            let _ = sender.send(Ok(()));
                        }
                    }
                }

                SwarmEvent::ConnectionClosed { .. } => {}

                SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                    if let Some(peer_id) = peer_id {
                        if let Some(sender) = self.pending_dial.remove(&peer_id) {
                            let _ = sender.send(Err(Box::new(error)));
                        }
                    }
                }

                SwarmEvent::Behaviour(ComposedEvent::Kademlia(KademliaEvent::RoutingUpdated {
                    peer,
                    addresses,
                    ..
                })) => {
                    self.swarm
                        .behaviour_mut()
                        .identify
                        .push(iter::once(peer));
                    println!("RoutingUpdated");
                    println!("{peer:?}");
                    println!("{addresses:?}")
                }

                SwarmEvent::Behaviour(ComposedEvent::Identify(
                    libp2p_identify::Event::Received { peer_id, .. },
                )) => {
                    println!("New node identify.");
                    for address in self.swarm.behaviour_mut().addresses_of_peer(&peer_id) {
                        println!("Add new address: {address:?}");
                        self.swarm
                            .behaviour_mut()
                            .kademlia
                            .add_address(&peer_id, address);
                    }
                    // println!("{peer_id:?}")
                }

                SwarmEvent::IncomingConnectionError { .. } => {}

                SwarmEvent::Dialing(peer_id) => {
                    eprintln!("Dialing {peer_id}")
                }

                // SwarmEvent::Behaviour(event) => {
                //     println!("New event");
                //     println!("{event:?}")
                // }

                e => panic!("{e:?}"),
            }
        }

        async fn handle_command(&mut self, command: Command) {
            match command {
                Command::StartListening { addr, sender } => {
                    let _ = match self.swarm.listen_on(addr) {
                        Ok(_) => sender.send(Ok(())),
                        Err(e) => sender.send(Err(Box::new(e))),
                    };
                }

                Command::Dial {
                    peer_id,
                    peer_addr,
                    sender,
                } => {
                    if let hash_map::Entry::Vacant(e) = self.pending_dial.entry(peer_id) {
                        self.swarm
                            .behaviour_mut()
                            .kademlia
                            .add_address(&peer_id, peer_addr.clone());
                        match self
                            .swarm
                            .dial(peer_addr.with(Protocol::P2p(peer_id.into())))
                        {
                            Ok(()) => {
                                e.insert(sender);
                            }
                            Err(e) => {
                                let _ = sender.send(Err(Box::new(e)));
                            }
                        }
                    } else {
                        todo!("Already dialing peer.");
                    }
                }

                Command::StartProviding { file_name, sender } => {
                    let query_id = self
                        .swarm
                        .behaviour_mut()
                        .kademlia
                        .start_providing(file_name.into_bytes().into())
                        .expect("No store error.");
                    self.pending_start_providing.insert(query_id, sender);
                }

                Command::GetProviders { file_name, sender } => {
                    let query_id = self
                        .swarm
                        .behaviour_mut()
                        .kademlia
                        .get_providers(file_name.into_bytes().into());
                    self.pending_get_providers.insert(query_id, sender);
                }

                Command::RequestFile {
                    file_name,
                    peer,
                    sender,
                } => {
                    let request_id = self
                        .swarm
                        .behaviour_mut()
                        .request_response
                        .send_request(&peer, FileRequest(file_name));
                    self.pending_request_file.insert(request_id, sender);
                }

                Command::RespondFile { file, channel } => {
                    self.swarm
                        .behaviour_mut()
                        .request_response
                        .send_response(channel, FileResponse(file))
                        .expect("Connection to peer to be still open.");
                }
            }
        }
    }

    #[derive(NetworkBehaviour)]
    #[behaviour(out_event = "ComposedEvent")]
    struct MyBehaviour {
        request_response: request_response::RequestResponse<FileExchangeCodec>,
        kademlia: Kademlia<MemoryStore>,
        identify: libp2p_identify::Behaviour,
    }

    #[derive(Debug)]
    enum ComposedEvent {
        RequestResponse(request_response::RequestResponseEvent<FileRequest, FileResponse>),
        Kademlia(KademliaEvent),
        Identify(libp2p_identify::Event),
    }

    impl From<request_response::RequestResponseEvent<FileRequest, FileResponse>> for ComposedEvent {
        fn from(event: request_response::RequestResponseEvent<FileRequest, FileResponse>) -> Self {
            ComposedEvent::RequestResponse(event)
        }
    }

    impl From<KademliaEvent> for ComposedEvent {
        fn from(event: KademliaEvent) -> Self {
            ComposedEvent::Kademlia(event)
        }
    }

    impl From<libp2p_identify::Event> for ComposedEvent {
        fn from(event: libp2p_identify::Event) -> Self {
            ComposedEvent::Identify(event)
        }
    }

    #[derive(Debug)]
    enum Command {
        StartListening {
            addr: Multiaddr,
            sender: oneshot::Sender<Result<(), Box<dyn Error + Send>>>,
        },
        Dial {
            peer_id: PeerId,
            peer_addr: Multiaddr,
            sender: oneshot::Sender<Result<(), Box<dyn Error + Send>>>,
        },
        StartProviding {
            file_name: String,
            sender: oneshot::Sender<()>,
        },
        GetProviders {
            file_name: String,
            sender: oneshot::Sender<HashSet<PeerId>>,
        },
        RequestFile {
            file_name: String,
            peer: PeerId,
            sender: oneshot::Sender<Result<Vec<u8>, Box<dyn Error + Send>>>,
        },
        RespondFile {
            file: Vec<u8>,
            channel: ResponseChannel<FileResponse>,
        },
    }

    #[derive(Debug)]
    pub enum Event {
        InboundRequest {
            request: String,
            channel: ResponseChannel<FileResponse>,
        },
    }

    // Simple file exchange protocol

    #[derive(Debug, Clone)]
    struct FileExchangeProtocol();
    #[derive(Clone)]
    struct FileExchangeCodec();
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct FileRequest(String);
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FileResponse(Vec<u8>);

    impl ProtocolName for FileExchangeProtocol {
        fn protocol_name(&self) -> &[u8] {
            "/file-exchange/1".as_bytes()
        }
    }

    #[async_trait]
    impl request_response::RequestResponseCodec for FileExchangeCodec {
        type Protocol = FileExchangeProtocol;
        type Request = FileRequest;
        type Response = FileResponse;

        async fn read_request<T>(
            &mut self,
            _: &FileExchangeProtocol,
            io: &mut T,
        ) -> io::Result<Self::Request>
        where
            T: AsyncRead + Unpin + Send,
        {
            let vec = read_length_prefixed(io, 1_000_000).await?;

            if vec.is_empty() {
                return Err(io::ErrorKind::UnexpectedEof.into());
            }

            Ok(FileRequest(String::from_utf8(vec).unwrap()))
        }

        async fn read_response<T>(
            &mut self,
            _: &FileExchangeProtocol,
            io: &mut T,
        ) -> io::Result<Self::Response>
        where
            T: AsyncRead + Unpin + Send,
        {
            let vec = read_length_prefixed(io, 500_000_000).await?; // update transfer maximum

            if vec.is_empty() {
                return Err(io::ErrorKind::UnexpectedEof.into());
            }

            Ok(FileResponse(vec))
        }

        async fn write_request<T>(
            &mut self,
            _: &FileExchangeProtocol,
            io: &mut T,
            FileRequest(data): FileRequest,
        ) -> io::Result<()>
        where
            T: AsyncWrite + Unpin + Send,
        {
            write_length_prefixed(io, data).await?;
            io.close().await?;

            Ok(())
        }

        async fn write_response<T>(
            &mut self,
            _: &FileExchangeProtocol,
            io: &mut T,
            FileResponse(data): FileResponse,
        ) -> io::Result<()>
        where
            T: AsyncWrite + Unpin + Send,
        {
            write_length_prefixed(io, data).await?;
            io.close().await?;

            Ok(())
        }
    }
}
