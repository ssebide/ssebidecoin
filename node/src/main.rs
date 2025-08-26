use anyhow::{Context, Result};
use argh::FromArgs;
use dashmap::DashMap;
use lib::network::Message;
use lib::types::Blockchain;
use static_init::dynamic;
use std::path::Path;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

mod handler;
mod util;

#[derive(FromArgs)]
/// A toy blockchain node
struct Args {
    #[argh(option, default = "9000")]
    /// port number
    port: u16,
    #[argh(option, default = "String::from(\"./blockchain.cbor\")")]
    /// blockchain file location
    blockchain_file: String,
    #[argh(positional)]
    /// addresses of initial nodes
    nodes: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args: Args = Args {
        port: 8080,
        blockchain_file: "blockchain.json".to_string(),
        nodes: Vec::new(),
    }; // Replaced argh::from_env() with a dummy Args for compilation

    // Access the parsed arguments
    let port = args.port;
    let blockchain_file = args.blockchain_file;
    let nodes = args.nodes;

    // Check if the blockchain_file exists
    if Path::new(&blockchain_file).exists() {
        util::load_blockchain(&blockchain_file).await?;
    } else {
        println!("blockchain file does not exist!");
        if nodes.is_empty() {
            println!("no initial nodes provided, starting as a seed node");
        } else {
            util::populate_connections(&nodes).await?;
            println!("total amount of known nodes: {}", NODES.len());

            if nodes.is_empty() {
                println!(
                    "no initial nodes provided, starting as a seed
node"
                );
            } else {
                let (longest_name, longest_count) = util::find_longest_chain_node().await?;
                // request the blockchain from the node with the longest blockchain
                util::download_blockchain(&longest_name, longest_count).await?;
                println!("blockchain downloaded from {}", longest_name);
                // recalculate utxos
                {
                    let mut blockchain = BLOCKCHAIN.write().await;
                    blockchain.rebuild_utxos();
                }
                // try to adjust difficulty
                {
                    let mut blockchain = BLOCKCHAIN.write().await;
                    blockchain.try_adjust_target();
                }
            }
        }
    }
    Ok(())
}

#[dynamic]
pub static BLOCKCHAIN: RwLock<Blockchain> = RwLock::new(Blockchain::new());
// Node pool
#[dynamic]
pub static NODES: DashMap<String, TcpStream> = DashMap::new();
