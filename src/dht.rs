use async_std::io;
use futures::StreamExt;
use futures::{prelude::*, select};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{record::Key, AddProviderOk, PeerRecord, PutRecordOk, Quorum, Record};
use libp2p::kad::{GetClosestPeersError, Kademlia, KademliaEvent, QueryResult};
use libp2p::swarm::NetworkBehaviour;
use libp2p::{
    development_transport, identity,
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use libp2p_kad::{GetProvidersOk, GetRecordOk};
use std::str::FromStr;
use std::{env, error::Error, time::Duration};

#[async_std::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // TODO: take from files.
    let local_key = identity::Keypair::generate_ed25519();
    let key_copy = local_key.clone();

    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id:?}");

    let transport = development_transport(local_key).await?;

    #[derive(NetworkBehaviour)]
    #[behaviour(out_event = "MyBehaviourEvent")]
    struct MyBehaviour {
        kademlia: Kademlia<MemoryStore>,
        identify: libp2p_identify::Behaviour,
    }

    #[allow(clippy::large_enum_variant)]
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

    // Create a swarm to manage peers and events.
    let mut swarm = {
        let store = MemoryStore::new(local_peer_id);
        let kademlia = Kademlia::new(local_peer_id, store);

        let mut cfg_identify = libp2p_identify::Config::new("a".to_string(), key_copy.public());
        let identify = libp2p_identify::Behaviour::new(cfg_identify);

        let mut behaviour = MyBehaviour { kademlia, identify };

        // TODO: take it from arrays.
        behaviour.kademlia.add_address(
            &"12D3KooWHVsb87bNGcdw53UPBm4MRQJaLgwgPMrqkXrsnn6iRimk".parse()?,
            "/ip4/172.27.140.48/tcp/41377".parse()?,
        );

        //TODO: what executor use
        Swarm::with_async_std_executor(transport, behaviour, local_peer_id)
    };

    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    swarm
        .behaviour_mut()
        .kademlia
        .bootstrap()
        .expect("Can't bootstrap.");

    swarm.behaviour_mut().kademlia.kbuckets();

    loop {
        select! {
        line = stdin.select_next_some() => handle_input_line(&mut swarm.behaviour_mut().kademlia, local_peer_id, line.expect("Stdin not to close")),
        event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening in {address:?}");
                },
            SwarmEvent::Behaviour(MyBehaviourEvent::Kademlia(KademliaEvent::OutboundQueryProgressed { result, ..})) => {
                match result {
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
                        QueryResult::GetProviders(Ok(GetProvidersOk::FoundProviders { key, providers, .. })) => {
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
                        QueryResult::GetRecord(Ok(
                            GetRecordOk::FoundRecord(PeerRecord {
                                record: Record { key, value, .. },
                                ..
                            })
                        )) => {
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
                    }
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Kademlia(KademliaEvent::RoutingUpdated { peer, addresses, is_new_peer, bucket_range, old_peer })) => {
                    swarm.behaviour_mut().identify.push(std::iter::once(peer));
                    println!("RoutingUpdated");
                    println!("{peer:?}");
                    println!("{addresses:?}")
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Identify(libp2p_identify::Event::Received {peer_id, info})) => {
                    println!("New node identify.");
                    for address in  swarm.behaviour_mut().addresses_of_peer(&peer_id) {
                        println!("Add new address: {address:?}");
                        swarm.behaviour_mut().kademlia.add_address(&peer_id, address);
                    }
                    // println!("{peer_id:?}")
                },
                SwarmEvent::Behaviour(event) => {
                    println!("New event");
                    println!("{event:?}")
                },
                _ => {}
            }
        }
    }
}

fn handle_input_line(kademlia: &mut Kademlia<MemoryStore>, local_peer: PeerId, line: String) {
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
            kademlia.get_record(key);
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
            kademlia.get_providers(key);
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
            kademlia
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

            kademlia
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

            for address in kademlia.addresses_of_peer(&key.unwrap()) {
                println!("{address:?}")
            }
        }
        Some("NODES") => {
            kademlia.get_closest_peers(local_peer);
        }
        _ => {
            eprintln!("expected GET, GET_PROVIDERS, NODES, ADDRESS_NODE, PUT or PUT_PROVIDER");
        }
    }
}
