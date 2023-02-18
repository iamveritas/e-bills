use crate::dht::network::*;

use async_std::io;
use async_std::task::spawn;
use clap::Parser;
use futures::channel::mpsc::Receiver;
use futures::prelude::*;
use libp2p::core::{Multiaddr, PeerId};
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

#[async_std::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut network_event_loop = network::new().await?;

    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    spawn(network_event_loop.run(stdin));

    loop {}
}

mod network {
    use super::*;

    use async_std::io::{BufReader, Stdin};
    use async_trait::async_trait;

    use futures::channel::mpsc::Receiver;
    use futures::channel::{mpsc, oneshot};
    use futures::io::Lines;
    use futures::stream::Fuse;
    use libp2p::core::either::EitherError;
    use libp2p::core::upgrade::{read_length_prefixed, write_length_prefixed, ProtocolName};

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

    pub async fn new() -> Result<(EventLoop), Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let key_copy = local_key.clone();

        let local_peer_id = PeerId::from(local_key.public());
        println!("Local peer id: {local_peer_id:?}");

        let transport = development_transport(local_key).await?;

        let mut swarm = {
            let store = MemoryStore::new(local_peer_id);
            let kademlia = Kademlia::new(local_peer_id, store);

            let cfg_identify = libp2p_identify::Config::new("a".to_string(), key_copy.public());
            let identify = libp2p_identify::Behaviour::new(cfg_identify);

            let mut behaviour = MyBehaviour { kademlia, identify };

            // TODO: take it from arrays.
            behaviour.kademlia.add_address(
                &"12D3KooWBmmKMQDdnJUmUSd2w5yFYY3XSZJYwKFZfFLJxEGtHh2J".parse()?,
                "/ip4/172.28.148.154/tcp/46433".parse()?,
            );

            //TODO: what executor use
            Swarm::with_threadpool_executor(transport, behaviour, local_peer_id)
        };

        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        swarm
            .behaviour_mut()
            .kademlia
            .bootstrap()
            .expect("Can't bootstrap.");
        Ok((EventLoop::new(swarm)))
    }

    pub struct EventLoop {
        swarm: Swarm<MyBehaviour>,
    }

    impl EventLoop {
        fn new(swarm: Swarm<MyBehaviour>) -> Self {
            Self { swarm }
        }

        pub async fn run(mut self, mut stdin: Fuse<Lines<BufReader<Stdin>>>) {
            loop {
                futures::select! {
                    event = self.swarm.next() => self.handle_event(event.unwrap()).await,
                    line = stdin.select_next_some() => self.handle_input_line(line.expect("Stdin not to close")).await,
                }
            }
        }

        async fn handle_event(
            &mut self,
            event: SwarmEvent<MyBehaviourEvent, EitherError<io::Error, io::Error>>,
        ) {
            match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening in {address:?}");
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Kademlia(
                    KademliaEvent::OutboundQueryProgressed { result, .. },
                )) => match result {
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
                    QueryResult::GetProviders(Ok(GetProvidersOk::FoundProviders {
                        key,
                        providers,
                        ..
                    })) => {
                        for peer in providers {
                            println!(
                                "Peer {peer:?} provides key {:?}",
                                std::str::from_utf8(key.as_ref()).unwrap()
                            );
                        }
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
                    QueryResult::StartProviding(Ok(AddProviderOk { key })) => {
                        println!(
                            "Successfully put provider record {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap()
                        );
                    }
                    QueryResult::StartProviding(Err(err)) => {
                        eprintln!("Failed to put provider record: {err:?}");
                    }
                    _ => {}
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Kademlia(
                    KademliaEvent::RoutingUpdated {
                        peer,
                        addresses,
                        is_new_peer: _,
                        bucket_range: _,
                        old_peer: _,
                    },
                )) => {
                    self.swarm
                        .behaviour_mut()
                        .identify
                        .push(std::iter::once(peer));
                    println!("RoutingUpdated");
                    println!("{peer:?}");
                    println!("{addresses:?}")
                }
                SwarmEvent::Behaviour(MyBehaviourEvent::Identify(
                    libp2p_identify::Event::Received { peer_id, info: _ },
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
                SwarmEvent::Behaviour(event) => {
                    println!("New event");
                    println!("{event:?}")
                }
                _ => {}
            }
        }

        async fn handle_input_line(&mut self, line: String) {
            let mut args = line.split(' ');

            match args.next() {
                Some("GET") => {
                    let key = {
                        match args.next() {
                            Some(key) => Key::new(&key),
                            None => {
                                eprintln!("Expected key");
                                return;
                            }
                        }
                    };
                    self.swarm.behaviour_mut().kademlia.get_record(key);
                }
                Some("GET_PROVIDERS") => {
                    let key = {
                        match args.next() {
                            Some(key) => Key::new(&key),
                            None => {
                                eprintln!("Expected key");
                                return;
                            }
                        }
                    };
                    self.swarm.behaviour_mut().kademlia.get_providers(key);
                }
                Some("PUT") => {
                    let key = {
                        match args.next() {
                            Some(key) => Key::new(&key),
                            None => {
                                eprintln!("Expected key");
                                return;
                            }
                        }
                    };
                    let value = {
                        match args.next() {
                            Some(value) => value.as_bytes().to_vec(),
                            None => {
                                eprintln!("Expected value");
                                return;
                            }
                        }
                    };
                    let record = Record {
                        key,
                        value,
                        publisher: None,
                        expires: None,
                    };
                    self.swarm
                        .behaviour_mut()
                        .kademlia
                        .put_record(record, Quorum::One)
                        .expect("Failed to store record locally.");
                }
                Some("PUT_PROVIDER") => {
                    let key = {
                        match args.next() {
                            Some(key) => Key::new(&key),
                            None => {
                                eprintln!("Expected key");
                                return;
                            }
                        }
                    };

                    self.swarm
                        .behaviour_mut()
                        .kademlia
                        .start_providing(key)
                        .expect("Failed to start providing key");
                }
                Some("ADDRESS_NODE") => {
                    let key = {
                        match args.next() {
                            Some(key) => PeerId::from_str(key),
                            None => {
                                eprintln!("Expected key");
                                return;
                            }
                        }
                    };

                    for address in self
                        .swarm
                        .behaviour_mut()
                        .kademlia
                        .addresses_of_peer(&key.unwrap())
                    {
                        println!("{address:?}")
                    }
                }
                // Some("NODES") => {
                //     self.swarm.behaviour_mut().kademlia.get_closest_peers(self.swarm.behaviour_mut();
                // }
                _ => {
                    eprintln!("expected GET, GET_PROVIDERS, ADDRESS_NODE, PUT or PUT_PROVIDER");
                }
            }
        }
    }

    #[derive(NetworkBehaviour)]
    #[behaviour(out_event = "MyBehaviourEvent")]
    struct MyBehaviour {
        kademlia: Kademlia<MemoryStore>,
        identify: libp2p_identify::Behaviour,
    }

    // #[allow(clippy::large_enum_variant)]
    #[derive(Debug)]
    enum MyBehaviourEvent {
        Kademlia(KademliaEvent),
        Identify(libp2p_identify::Event),
    }

    impl From<KademliaEvent> for MyBehaviourEvent {
        fn from(event: KademliaEvent) -> Self {
            MyBehaviourEvent::Kademlia(event)
        }
    }

    impl From<libp2p_identify::Event> for MyBehaviourEvent {
        fn from(event: libp2p_identify::Event) -> Self {
            MyBehaviourEvent::Identify(event)
        }
    }
}
