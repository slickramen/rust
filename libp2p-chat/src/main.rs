use std::{error::Error, time::Duration};
use futures::StreamExt;
use libp2p::{ gossipsub, mdns, noise, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux };
use tokio::{ self, io::{self, AsyncBufReadExt}, select };

#[derive(NetworkBehaviour)]
struct ChatBehaviour {
    mdns: mdns::tokio::Behaviour,
    gossipsub: gossipsub::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(), 
            noise::Config::new, 
            yamux::Config::default
        )?
        .with_quic()
        .with_behaviour(|key| {
            let gossipsub_config = gossipsub::Config::default();

            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config
            )?;

            let mdns = mdns::tokio::Behaviour::new(
                mdns::Config::default(),
                key.public().to_peer_id()
            )?;

            Ok(ChatBehaviour{ mdns, gossipsub })
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();
    
        let topic = gossipsub::IdentTopic::new("chat");

        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        let mut stdin = io::BufReader::new(io::stdin()).lines();

        loop {
            select! {
                Ok(Some(line)) = stdin.next_line() => {
                    if let Err(err) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), line.as_bytes()) {
                        println!("Publish error: {err:?}");
                    }
                }

                event = swarm.select_next_some() => match event {
                    SwarmEvent::NewListenAddr { address, .. } => println!("Your node is listening on {address}"),
                    SwarmEvent::Behaviour(ChatBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, multiaddr) in list {
                            println!("mDNS discovered a new peer: {peer_id}, listening on {multiaddr}");
                            swarm.dial(multiaddr)?;
                        }
                    }
                    SwarmEvent::Behaviour(ChatBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                    })) => println!(
                            "Got message: '{}' with id: {id} from peer: {peer_id}",
                            String::from_utf8_lossy(&message.data),
                    ),
                    _ => {}
                }
            }
        }
}
