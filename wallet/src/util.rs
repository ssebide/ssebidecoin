use crate::core::{Config, Core, FeeConfig, FeeType, Recipient};
use anyhow::Result;
use kanal::Sender;
use std::panic;
use std::path::PathBuf;
use tracing::*;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Initialize tracing to save logs into the logs/ folder
pub fn setup_tracing() -> Result<()> {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "wallet.log");
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(file_appender))
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .init();
    Ok(())
}

/// Make sure tracing is able to log panics occurring in the wallet
pub fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let backtrace = std::backtrace::Backtrace::force_capture();
        error!("Application panicked!");
        error!("Panic info: {:?}", panic_info);
        error!("Backtrace: {:?}", backtrace);
    }));
}

/// Generate a dummy config
pub fn generate_dummy_config(path: &PathBuf) -> Result<()> {
    let dummy_config = Config {
        my_keys: vec![],
        contacts: vec![
            Recipient {
                name: "Alice".to_string(),
                key: PathBuf::from("alice.pub.pem"),
            },
            Recipient {
                name: "Bob".to_string(),
                key: PathBuf::from("bob.pub.pem"),
            },
        ],
        default_node: "127.0.0.1:9000".to_string(),
        fee_config: FeeConfig {
            fee_type: FeeType::Percent,
            value: 0.1,
        },
    };
    let config_str = toml::to_string_pretty(&dummy_config)?;
    std::fs::write(path, config_str)?;
    info!("Dummy config generated at: {}", path.display());
    Ok(())
}

/// Convert satoshis to a BTC string
pub fn sats_to_btc(sats: u64) -> String {
    let btc = sats as f64 / 100_000_000.0;
    format!("{} BTC", btc)
}

/// Make it big lmao
pub fn big_mode_btc(core: &Core) -> String {
    text_to_ascii_art::convert(sats_to_btc(core.get_balance())).unwrap()
}

pub struct Core {
    pub config: Config,
    utxos: UtxoStore,
    pub tx_sender: Sender<Transaction>,
    pub stream: Mutex<TcpStream>,
}

use tokio::net::TcpStream;
use tokio::sync::Mutex;
/// Create a new Core instance.
fn new(config: Config, utxos: UtxoStore, stream: TcpStream) -> Self {
    let (tx_sender, _) = kanal::bounded(10);
    Core {
        config,
        utxos,
        tx_sender,
        stream: Mutex::new(stream),
    }
}
/// Load the Core from a configuration file.
pub async fn load(config_path: PathBuf) -> Result<Self> {
    info!("Loading core from config: {:?}", config_path);
    let config: Config = toml::from_str(&fs::read_to_string(&config_path)?)?;
    let mut utxos = UtxoStore::new();
    let stream = TcpStream::connect(&config.default_node).await?;
    // Load keys from config
    for key in &config.my_keys {
        debug!("Loading key pair: {:?}", key.public);
        let public = PublicKey::load_from_file(&key.public)?;
        let private = PrivateKey::load_from_file(&key.private)?;
        utxos.add_key(LoadedKey { public, private });
    }
    Ok(Core::new(config, utxos, stream))
}

pub async fn fetch_utxos(&self) -> Result<()> {
    debug!("Fetching UTXOs from node: {}", self.config.default_node);
    for key in &self.utxos.my_keys {
        let message = Message::FetchUTXOs(key.public.clone());
        message.send_async(&mut *self.stream.lock().await).await?;
        if let Message::UTXOs(utxos) =
            Message::receive_async(&mut *self.stream.lock().await).await?
        {
            debug!("Received {} UTXOs for key: {:?}", utxos.len(), key.public);
            // Replace the entire UTXO set for this key
            self.utxos.utxos.insert(
                key.public.clone(),
                utxos
                    .into_iter()
                    .map(|(output, marked)| (marked, output))
                    .collect(),
            );
        } else {
            error!("Unexpected response from node");
            return Err(anyhow::anyhow!("Unexpected response from node"));
        }
    }
    info!("UTXOs fetched successfully");
    Ok(())
}
/// Send a transaction to the node.
pub async fn send_transaction(&self, transaction: Transaction) -> Result<()> {
    debug!("Sending transaction to node: {}", self.config.default_node);
    let message = Message::SubmitTransaction(transaction);
    message.send_async(&mut *self.stream.lock().await).await?;
    info!("Transaction sent successfully");
    Ok(())
}

pub fn send_transaction_async(&self, recipient: &str, amount: u64) -> Result<()> {
    info!("Preparing to send {} satoshis to {}", amount, recipient);
    let recipient_key = self
        .config
        .contacts
        .iter()
        .find(|r| r.name == recipient)
        .ok_or_else(|| anyhow::anyhow!("Recipient not found"))?
        .load()?
        .key;
    let transaction = self.create_transaction(&recipient_key, amount)?;
    debug!("Sending transaction asynchronously");
    self.tx_sender.send(transaction)?;
    Ok(())
}
