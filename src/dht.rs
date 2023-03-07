use crate::{bill_from_byte_array, write_bill_to_file};

use crate::dht::network::Client;
use async_std::io;
use async_std::task::spawn;
use clap::Parser;
use futures::prelude::*;
use libp2p::core::{Multiaddr, PeerId};
use std::error::Error;
use std::path::PathBuf;

// TODO: take bootstrap node info from config file.
const BOOTSTRAP_NODE: &str = "12D3KooWQCQqiX8fwrWpsjUDoKa7nt95Nwx3W5AV3vJifwDWssGR";
const BOOTSTRAP_ADDRESS: &str = "/ip4/172.27.106.82/tcp/35415";

pub async fn dht_main() -> Result<Client, Box<dyn Error + Send + Sync>> {
    let (mut network_client, mut network_events, mut network_event_loop) = network::new()
        .await
        .expect("Can not to create network module in dht.");

    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    spawn(network_event_loop.run());

    let network_client_to_return = network_client.clone();

    network_client
        .start_listening(
            "/ip4/0.0.0.0/tcp/0"
                .parse()
                .expect("Can not start listening."),
        )
        .await
        .expect("Listening not to fail.");

    spawn(network_client.run(stdin, network_events));

    Ok(network_client_to_return)
}

#[derive(Parser, Debug)]
#[clap(name = "Bitcredit first version dht")]
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

pub mod network {
    use super::*;
    use crate::constants::BILLS_FOLDER_PATH;
    use crate::{read_ed25519_keypair_from_file, read_peer_id_from_file, BitcreditBill};
    use async_std::io::{BufReader, Stdin};
    use async_trait::async_trait;
    use futures::channel::mpsc::Receiver;
    use futures::channel::{mpsc, oneshot};
    use futures::io::Lines;
    use futures::stream::Fuse;
    use libp2p::core::either::EitherError;
    use libp2p::core::upgrade::{read_length_prefixed, write_length_prefixed, ProtocolName};
    use libp2p::development_transport;
    use libp2p::kad::record::store::MemoryStore;
    use libp2p::kad::record::{Key, Record};
    use libp2p::kad::{
        GetProvidersOk, GetRecordError, GetRecordOk, Kademlia, KademliaEvent, PeerRecord,
        PutRecordOk, QueryId, QueryResult, Quorum,
    };
    use libp2p::multiaddr::Protocol;
    use libp2p::request_response::{
        self, ProtocolSupport, RequestId, RequestResponseEvent, RequestResponseMessage,
        ResponseChannel,
    };
    use libp2p::swarm::{ConnectionHandlerUpgrErr, NetworkBehaviour, Swarm, SwarmEvent};
    use std::collections::{hash_map, HashMap, HashSet};
    use std::iter;
    use std::path::Path;

    pub async fn new() -> Result<(Client, Receiver<Event>, EventLoop), Box<dyn Error>> {
        //TODO: If its first time login?
        let local_key = read_ed25519_keypair_from_file();
        let key_copy = local_key.clone();
        let local_peer_id = read_peer_id_from_file();

        // let local_key = identity::Keypair::generate_ed25519();
        // let key_copy = local_key.clone();
        // let local_peer_id = PeerId::from(local_key.public());

        println!("Local peer id: {local_peer_id:?}");

        let transport = development_transport(local_key).await?;

        let mut swarm = {
            let store = MemoryStore::new(local_peer_id);
            let kademlia = Kademlia::new(local_peer_id, store);

            let cfg_identify = libp2p::identify::Config::new(
                "protocol identify version 1".to_string(),
                key_copy.public(),
            );
            let identify = libp2p::identify::Behaviour::new(cfg_identify);

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

            behaviour
                .kademlia
                .add_address(&BOOTSTRAP_NODE.parse()?, BOOTSTRAP_ADDRESS.parse()?);

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

        pub async fn run(
            mut self,
            mut stdin: Fuse<Lines<BufReader<Stdin>>>,
            mut network_events: Receiver<Event>,
        ) {
            loop {
                futures::select! {
                    line = stdin.select_next_some() => self.handle_input_line(line.expect("Stdin not to close.")).await,
                    event = network_events.next() => self.handle_event(event.expect("Swarm stream to be infinite.")).await,
                }
            }
        }

        pub async fn check_new_bills(&mut self, node_id: String) {
            //1) GET_RECORD
            let node_request = "BILLS".to_string() + &node_id;
            println!("Request {node_request:?}");
            let mut list_bills_for_node = self.get_record(node_request.clone()).await;
            let value = list_bills_for_node.value;
            if !value.is_empty() {
                let record_for_saving_in_dht = std::str::from_utf8(&value)
                    .expect("Cant get value.")
                    .to_string();
                let split = record_for_saving_in_dht.split(",");
                for bill_id in split {
                    // Check if we have bill in folder.
                    if !Path::new((BILLS_FOLDER_PATH.to_string() + "/" + bill_id).as_str()).exists()
                    {
                        // 2)GET
                        println!("Look for bill {}", bill_id);
                        let bill_bytes = self.get(bill_id.to_string()).await;

                        // Wright bill in files.
                        if !bill_bytes.is_empty() {
                            let bill: BitcreditBill = bill_from_byte_array(&bill_bytes);
                            bill.name.clone();
                            write_bill_to_file(&bill);
                        }
                    }
                }
            }
        }

        pub async fn add_bill_to_dht(&mut self, bill_name: &String, node_id: String) {
            //1) GET_RECORD
            let node_request = "BILLS".to_string() + &node_id;
            let mut record_for_saving_in_dht = "".to_string();
            let mut list_bills_for_node = self.get_record(node_request.clone()).await;
            let value = list_bills_for_node.value;
            if !value.is_empty() {
                record_for_saving_in_dht = std::str::from_utf8(&value)
                    .expect("Cant get value.")
                    .to_string();
                record_for_saving_in_dht = record_for_saving_in_dht.to_string() + "," + bill_name;
            } else {
                record_for_saving_in_dht = bill_name.clone();
            }

            //2) PUT_RECORD
            self.put_record(node_request.clone(), record_for_saving_in_dht.to_string())
                .await;
        }

        pub async fn put(&mut self, name: &String) {
            self.start_providing(name.clone()).await;
        }

        pub async fn get(&mut self, name: String) -> Vec<u8> {
            // Locate all nodes providing the file.
            let providers = self.get_providers(name.clone()).await;
            if providers.is_empty() {
                eprintln!("No providers was found.");
                Vec::new()
            } else {
                println!("Providers {providers:?}");

                // Request the content of the file from each node.
                //TODO: if it's me - don't continue.
                let requests = providers.into_iter().map(|peer| {
                    let mut network_client = self.clone();
                    let name = name.clone();
                    async move { network_client.request_file(peer, name).await }.boxed()
                });

                // Await the requests, ignore the remaining once a single one succeeds.
                let file_content = futures::future::select_ok(requests)
                    .await
                    .map_err(|_| "None of the providers returned file.")
                    .expect("Can not get file content.")
                    .0;

                println!("{file_content:?}");

                file_content
            }
        }

        /// Dial the given peer at the given address.
        async fn dial(
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

        async fn put_record(&mut self, key: String, value: String) {
            self.sender
                .send(Command::PutRecord { key, value })
                .await
                .expect("Command receiver not to be dropped.");
        }

        async fn get_record(&mut self, key: String) -> Record {
            let (sender, receiver) = oneshot::channel();
            self.sender
                .send(Command::GetRecord { key, sender })
                .await
                .expect("Command receiver not to be dropped.");
            receiver.await.expect("Sender not to be dropped.")
        }

        async fn start_providing(&mut self, file_name: String) {
            let (sender, receiver) = oneshot::channel();
            self.sender
                .send(Command::StartProviding { file_name, sender })
                .await
                .expect("Command receiver not to be dropped.");
            receiver.await.expect("Sender not to be dropped.");
        }

        async fn get_providers(&mut self, file_name: String) -> HashSet<PeerId> {
            let (sender, receiver) = oneshot::channel();
            self.sender
                .send(Command::GetProviders { file_name, sender })
                .await
                .expect("Command receiver not to be dropped.");
            receiver.await.expect("Sender not to be dropped.")
        }

        async fn request_file(
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

        async fn respond_file(&mut self, file: Vec<u8>, channel: ResponseChannel<FileResponse>) {
            self.sender
                .send(Command::RespondFile { file, channel })
                .await
                .expect("Command receiver not to be dropped.");
        }

        async fn handle_event(&mut self, event: Event) {
            match event {
                Event::InboundRequest { request, channel } => {
                    //The place where we explicitly specify to look for the bill is in the bills folder.
                    println!("{request:?}");
                    let path_to_bill = BILLS_FOLDER_PATH.to_string() + "/" + &request;
                    self.respond_file(
                        std::fs::read(&path_to_bill).expect("Can not respond."),
                        channel,
                    )
                    .await;
                }

                _ => {}
            }
        }

        //TODO: dont delete. Need for testing.
        async fn handle_input_line(&mut self, line: String) {
            let mut args = line.split(' ');

            match args.next() {
                Some("PUT") => {
                    let name: String = {
                        match args.next() {
                            Some(name) => String::from(name),
                            None => {
                                eprintln!("Expected name.");
                                return;
                            }
                        }
                    };
                    self.put(&name).await;
                }

                Some("GET") => {
                    let name: String = {
                        match args.next() {
                            Some(name) => String::from(name),
                            None => {
                                eprintln!("Expected name.");
                                return;
                            }
                        }
                    };
                    self.get(name).await;
                    println!("Bill was successfully saved.");
                }

                Some("PUT_RECORD") => {
                    let key = {
                        match args.next() {
                            Some(key) => String::from(key),
                            None => {
                                eprintln!("Expected key");
                                return;
                            }
                        }
                    };
                    let value = {
                        match args.next() {
                            Some(value) => String::from(value),
                            None => {
                                eprintln!("Expected value");
                                return;
                            }
                        }
                    };

                    self.put_record(key, value).await;
                }

                Some("GET_RECORD") => {
                    let key = {
                        match args.next() {
                            Some(key) => String::from(key),
                            None => {
                                eprintln!("Expected key");
                                return;
                            }
                        }
                    };
                    self.get_record(key).await;
                }

                Some("GET_PROVIDERS") => {
                    let key = {
                        match args.next() {
                            Some(key) => String::from(key),
                            None => {
                                eprintln!("Expected key");
                                return;
                            }
                        }
                    };
                    self.get_providers(key).await;
                }

                _ => {
                    eprintln!("expected GET, PUT, GET_RECORD, PUT_RECORD or GET_PROVIDERS.");
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
        pending_get_records: HashMap<QueryId, oneshot::Sender<Record>>,
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
                pending_get_records: Default::default(),
                pending_request_file: Default::default(),
            }
        }

        pub async fn run(mut self) {
            loop {
                futures::select! {
                    event = self.swarm.next() => self.handle_event(event.expect("Swarm stream to be infinite.")).await,
                    command = self.command_receiver.next() => match command {
                        Some(c) => self.handle_command(c).await,

                        _ => {}
                    },
                }
            }
        }

        async fn handle_event(
            &mut self,
            event: SwarmEvent<
                ComposedEvent,
                //TODO: change to normal error type.
                EitherError<
                    EitherError<ConnectionHandlerUpgrErr<std::io::Error>, std::io::Error>,
                    std::io::Error,
                >,
            >,
        ) {
            match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    let local_peer_id = *self.swarm.local_peer_id();
                    println!(
                        "Local node is listening on {:?}",
                        address.with(Protocol::P2p(local_peer_id.into()))
                    );
                }

                SwarmEvent::Behaviour(ComposedEvent::Kademlia(
                    KademliaEvent::OutboundQueryProgressed { result, id, .. },
                )) => match result {
                    QueryResult::StartProviding(Ok(libp2p::kad::AddProviderOk { key })) => {
                        let sender: oneshot::Sender<()> = self
                            .pending_start_providing
                            .remove(&id)
                            .expect("Completed query to be previously pending.");
                        let _ = sender.send(());
                        println!(
                            "Successfully put provider record {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap()
                        );
                    }

                    QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                        println!(
                            "Successfully put record {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap()
                        );
                    }

                    QueryResult::GetRecord(Ok(GetRecordOk::FoundRecord(PeerRecord {
                        record,
                        ..
                    }))) => {
                        if let Some(sender) = self.pending_get_records.remove(&id) {
                            println!(
                                "Got record {:?} {:?}",
                                std::str::from_utf8(&record.key.as_ref()).unwrap(),
                                std::str::from_utf8(&record.value).unwrap(),
                            );

                            sender.send(record).expect("Receiver not to be dropped.");

                            // Finish the query. We are only interested in the first result.
                            //TODO: think how to do it better.
                            self.swarm
                                .behaviour_mut()
                                .kademlia
                                .query_mut(&id)
                                .unwrap()
                                .finish();
                        }
                    }

                    QueryResult::GetRecord(Ok(GetRecordOk::FinishedWithNoAdditionalRecord {
                        ..
                    })) => {
                        self.pending_get_records.remove(&id);
                        println!("No records.");
                    }

                    QueryResult::GetRecord(Err(GetRecordError::NotFound { key, .. })) => {
                        //TODO: its bad.
                        let record = Record {
                            key: key,
                            value: vec![],
                            publisher: None,
                            expires: None,
                        };
                        let _ = self
                            .pending_get_records
                            .remove(&id)
                            .expect("Request to still be pending.")
                            .send(record);
                        println!("NotFound.");
                    }

                    QueryResult::GetRecord(Err(GetRecordError::Timeout { key })) => {
                        //TODO: its bad.
                        let record = Record {
                            key: key,
                            value: vec![],
                            publisher: None,
                            expires: None,
                        };
                        let _ = self
                            .pending_get_records
                            .remove(&id)
                            .expect("Request to still be pending.")
                            .send(record);
                        println!("Timeout.");
                    }

                    QueryResult::GetRecord(Err(GetRecordError::QuorumFailed { key, .. })) => {
                        //TODO: its bad.
                        let record = Record {
                            key: key,
                            value: vec![],
                            publisher: None,
                            expires: None,
                        };
                        let _ = self
                            .pending_get_records
                            .remove(&id)
                            .expect("Request to still be pending.")
                            .send(record);
                        println!("QuorumFailed.");
                    }

                    QueryResult::StartProviding(Err(err)) => {
                        //TODO: do some logic.
                        eprintln!("Failed to put provider record: {err:?}");
                    }

                    QueryResult::GetProviders(Ok(GetProvidersOk::FoundProviders {
                        providers,
                        ..
                    })) => {
                        if let Some(sender) = self.pending_get_providers.remove(&id) {
                            for peer in &providers {
                                println!("PEER {peer:?}");
                            }

                            sender.send(providers).expect("Receiver not to be dropped.");

                            // Finish the query. We are only interested in the first result.
                            //TODO: think how to do it better.
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
                    )) => {
                        //TODO: do some logic.
                    }

                    QueryResult::GetProviders(Err(err)) => {
                        //TODO: do some logic.
                        eprintln!("Failed to get providers: {err:?}");
                    }

                    _ => {}
                },

                SwarmEvent::Behaviour(ComposedEvent::Kademlia(KademliaEvent::RoutingUpdated {
                    peer,
                    ..
                })) => {
                    //TODO: do some logic. Dont push always.
                    self.swarm.behaviour_mut().identify.push(iter::once(peer));
                }

                SwarmEvent::Behaviour(ComposedEvent::RequestResponse(
                    RequestResponseEvent::OutboundFailure {
                        request_id, error, ..
                    },
                )) => {
                    let _ = self
                        .pending_request_file
                        .remove(&request_id)
                        .expect("Request to still be pending.")
                        .send(Err(Box::new(error)));
                }

                SwarmEvent::Behaviour(ComposedEvent::RequestResponse(
                    RequestResponseEvent::Message { message, .. },
                )) => match message {
                    RequestResponseMessage::Request {
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

                    RequestResponseMessage::Response {
                        request_id,
                        response,
                    } => {
                        let _ = self
                            .pending_request_file
                            .remove(&request_id)
                            .expect("Request to still be pending.")
                            .send(Ok(response.0));
                    }

                    _ => {}
                },

                SwarmEvent::Behaviour(ComposedEvent::RequestResponse(
                    RequestResponseEvent::ResponseSent { .. },
                )) => {
                    //TODO: do some logic.
                    println!("{event:?}")
                }

                SwarmEvent::Behaviour(ComposedEvent::Identify(
                    libp2p::identify::Event::Received { peer_id, .. },
                )) => {
                    println!("New node identify.");
                    for address in self.swarm.behaviour_mut().addresses_of_peer(&peer_id) {
                        self.swarm
                            .behaviour_mut()
                            .kademlia
                            .add_address(&peer_id, address);
                    }
                }

                SwarmEvent::IncomingConnection { .. } => {
                    //TODO: do some logic.
                    println!("{event:?}")
                }

                SwarmEvent::ConnectionEstablished {
                    peer_id, endpoint, ..
                } => {
                    if endpoint.is_dialer() {
                        if let Some(sender) = self.pending_dial.remove(&peer_id) {
                            let _ = sender.send(Ok(()));
                        }
                    }
                }

                SwarmEvent::ConnectionClosed { .. } => {
                    //TODO: do some logic.;
                    println!("{event:?}")
                }

                SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                    if let Some(peer_id) = peer_id {
                        if let Some(sender) = self.pending_dial.remove(&peer_id) {
                            let _ = sender.send(Err(Box::new(error)));
                        }
                    }
                }

                SwarmEvent::IncomingConnectionError { .. } => {
                    //TODO: do some logic.
                    println!("{event:?}")
                }

                SwarmEvent::Dialing(peer_id) => {
                    println!("Dialing {peer_id}")
                }

                SwarmEvent::Behaviour(event) => {
                    println!("New event");
                    println!("{event:?}")
                }

                _ => {}
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

                Command::StartProviding { file_name, sender } => {
                    let query_id = self
                        .swarm
                        .behaviour_mut()
                        .kademlia
                        .start_providing(file_name.into_bytes().into())
                        .expect("Can not provide.");
                    self.pending_start_providing.insert(query_id, sender);
                }

                Command::PutRecord { key, value } => {
                    let key_record = Key::new(&key);
                    let value_bytes = value.as_bytes().to_vec();
                    let record = Record {
                        key: key_record,
                        value: value_bytes,
                        publisher: None,
                        expires: None,
                    };
                    let query_id = self
                        .swarm
                        .behaviour_mut()
                        .kademlia
                        //TODO: what quorum use.
                        .put_record(record, Quorum::All)
                        .expect("Can not provide.");
                }

                Command::GetRecord { key, sender } => {
                    let key_record = Key::new(&key);
                    let query_id = self.swarm.behaviour_mut().kademlia.get_record(key_record);
                    self.pending_get_records.insert(query_id, sender);
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
                        //TODO: Already dialing peer?
                    }
                }
            }
        }
    }

    #[derive(NetworkBehaviour)]
    #[behaviour(out_event = "ComposedEvent")]
    struct MyBehaviour {
        request_response: request_response::RequestResponse<FileExchangeCodec>,
        kademlia: Kademlia<MemoryStore>,
        identify: libp2p::identify::Behaviour,
    }

    #[derive(Debug)]
    enum ComposedEvent {
        RequestResponse(RequestResponseEvent<FileRequest, FileResponse>),
        Kademlia(KademliaEvent),
        Identify(libp2p::identify::Event),
    }

    impl From<RequestResponseEvent<FileRequest, FileResponse>> for ComposedEvent {
        fn from(event: request_response::RequestResponseEvent<FileRequest, FileResponse>) -> Self {
            ComposedEvent::RequestResponse(event)
        }
    }

    impl From<KademliaEvent> for ComposedEvent {
        fn from(event: KademliaEvent) -> Self {
            ComposedEvent::Kademlia(event)
        }
    }

    impl From<libp2p::identify::Event> for ComposedEvent {
        fn from(event: libp2p::identify::Event) -> Self {
            ComposedEvent::Identify(event)
        }
    }

    #[derive(Debug)]
    enum Command {
        StartListening {
            addr: Multiaddr,
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
        PutRecord {
            key: String,
            value: String,
        },
        GetRecord {
            key: String,
            sender: oneshot::Sender<Record>,
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
        Dial {
            peer_id: PeerId,
            peer_addr: Multiaddr,
            sender: oneshot::Sender<Result<(), Box<dyn Error + Send>>>,
        },
    }

    #[derive(Debug)]
    pub enum Event {
        InboundRequest {
            request: String,
            channel: ResponseChannel<FileResponse>,
        },
    }

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
            let vec = read_length_prefixed(io, 1_000_000).await?; // TODO: update transfer maximum.

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
            let vec = read_length_prefixed(io, 500_000_000).await?; // TODO: update transfer maximum.

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
