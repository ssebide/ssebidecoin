mod core;
mod tasks;
mod ui;
mod util;
use anyhow::Result;
use clap::{Parser, Subcommand};
use core::{Config, Core, FeeConfig, FeeType, Recipient};
use cursive::views::TextContent;
use kanal;
use lib::types::Transaction;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tasks::{handle_transactions, ui_task, update_balance, update_utxos};
use tokio::time::{self, Duration};
use tracing::{debug, info};
use util::{big_mode_btc, generate_dummy_config, setup_panic_hook, setup_tracing};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, value_name = "FILE", default_value_os_t =
PathBuf::from("wallet_config.toml"))]
    config: PathBuf,
    #[arg(short, long, value_name = "ADDRESS")]
    node: Option<String>,
}
#[derive(Subcommand)]
enum Commands {
    GenerateConfig {
        #[arg(short, long, value_name = "FILE", default_value_os_t
= PathBuf::from("wallet_config.toml"))]
        output: PathBuf,
    },
    Send {
        #[arg(short, long)]
        recipient: String,
        #[arg(short, long)]
        amount: u64,
    },
    Balance,
}


#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing()?;
    setup_panic_hook();
    info!("Starting wallet application");
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::GenerateConfig { output }) => {
            debug!("Generating dummy config at: {:?}", output);
            return generate_dummy_config(output);
        }
        Some(Commands::Send { recipient, amount }) => {
            info!("Loading config from: {:?}", cli.config);
            let core = Core::load(cli.config.clone()).await?;
            
            println!("Fetching UTXOs...");
            core.fetch_utxos().await?;
            
            let balance = core.get_balance();
            println!("Current balance: {} satoshis ({} BTC)", balance, balance as f64 / 100_000_000.0);
            
            println!("Sending {} satoshis to {}...", amount, recipient);
            let recipient_key = core
                .config
                .contacts
                .iter()
                .find(|r| &r.name == recipient)
                .ok_or_else(|| anyhow::anyhow!("Recipient '{}' not found in contacts", recipient))?
                .load()?
                .key;
            
            let transaction = core.create_transaction(&recipient_key, *amount)?;
            core.send_transaction(transaction).await?;
            
            println!("✓ Transaction sent successfully!");
            println!("Remember to mine a new block to confirm the transaction.");
            return Ok(());
        }
        Some(Commands::Balance) => {
            info!("Loading config from: {:?}", cli.config);
            let core = Core::load(cli.config.clone()).await?;
            
            println!("Fetching UTXOs from node...");
            match core.fetch_utxos().await {
                Ok(_) => println!("✓ UTXOs fetched successfully"),
                Err(e) => {
                    println!("✗ Failed to fetch UTXOs: {}", e);
                    println!("Make sure the node is running at {}", core.config.default_node);
                    return Err(e);
                }
            }
            
            let balance = core.get_balance();
            println!("\n💰 Balance: {} satoshis", balance);
            println!("   ({} BTC)", balance as f64 / 100_000_000.0);
            return Ok(());
        }
        None => (),
    }

    info!("Loading config from: {:?}", cli.config);
    let mut core = Core::load(cli.config.clone()).await?;
    if let Some(node) = cli.node {
        info!("Overriding default node with: {}", node);
        core.config.default_node = node;
    }
    let (tx_sender, tx_receiver) = kanal::bounded(10);
    core.tx_sender = tx_sender;
    let core = Arc::new(core);
    info!("Starting background tasks");
    let balance_content = TextContent::new(big_mode_btc(&core));
    tokio::select! {
    _ = ui_task(core.clone(), balance_content.clone()) =>
    (),
    _ = update_utxos(core.clone()) => (),
    _ = handle_transactions(tx_receiver.clone_async(), core.
    clone()) => (),
    _ = update_balance(core.clone(), balance_content) =>
    (),
    }
    info!("Application shutting down");
    Ok(())
}
