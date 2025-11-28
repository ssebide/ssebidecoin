use anyhow::{Context, Result};
use argh::FromArgs;
use dashmap::DashMap;
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

#[dynamic]
pub static BLOCKCHAIN: RwLock<Blockchain> = RwLock::new(Blockchain::new());
// Node pool
#[dynamic]
pub static NODES: DashMap<String, TcpStream> = DashMap::new();

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args: Args = argh::from_env();

    // Access the parsed arguments
    let port = args.port;
    let blockchain_file = args.blockchain_file;
    let nodes = args.nodes;

    // Start a task to periodically clean up the mempool
    tokio::spawn(util::cleanup());

    // Check if the blockchain_file exists
    if Path::new(&blockchain_file).exists() {
        util::load_blockchain(&blockchain_file)
            .await
            .context("Failed to load blockchain")?;
    } else {
        println!("Blockchain file does not exist!");
        if nodes.is_empty() {
            println!("No initial nodes provided, starting as a seed node");
        } else {
            util::populate_connections(&nodes)
                .await
                .context("Failed to populate connections")?;
            println!("Total amount of known nodes: {}", NODES.len());

            let (longest_name, longest_count) = util::find_longest_chain_node()
                .await
                .context("Failed to find longest chain node")?;
            // Request the blockchain from the node with the longest blockchain
            util::download_blockchain(&longest_name, longest_count)
                .await
                .context("Failed to download blockchain")?;
            println!("Blockchain downloaded from {}", longest_name);
            // Recalculate UTXOs
            {
                let mut blockchain = BLOCKCHAIN.write().await;
                blockchain.rebuild_utxos();
            }
            // Try to adjust difficulty
            {
                let mut blockchain = BLOCKCHAIN.write().await;
                blockchain.try_adjust_target();
            }
        }
    }

    // Start a task to periodically save the blockchain (after loading/initializing)
    tokio::spawn(util::save(blockchain_file.clone()));

    // Start the TCP server
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr)
        .await
        .context("Failed to bind TCP listener")?;
    println!("Listening on {}", addr);

    // Accept incoming connections
    loop {
        let (socket, _) = listener
            .accept()
            .await
            .context("Failed to accept connection")?;
        tokio::spawn(handler::handle_connection(socket));
    }
}
