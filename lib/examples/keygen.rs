use lib::crypto::PrivateKey;
use lib::utils::Saveable;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <private_key_path> <public_key_path>", args[0]);
        eprintln!("Example: {} wallet.priv.cbor wallet.pub.pem", args[0]);
        std::process::exit(1);
    }

    let private_key_path = PathBuf::from(&args[1]);
    let public_key_path = PathBuf::from(&args[2]);

    println!("Generating new keypair...");
    
    // Generate a new private key
    let private_key = PrivateKey::new_key();
    let public_key = private_key.public_key();

    // Save the private key
    println!("Saving private key to: {}", private_key_path.display());
    private_key.save_to_file(&private_key_path)?;

    // Save the public key
    println!("Saving public key to: {}", public_key_path.display());
    public_key.save_to_file(&public_key_path)?;

    println!("✓ Keypair generated successfully!");
    println!("  Private key: {}", private_key_path.display());
    println!("  Public key: {}", public_key_path.display());
    println!("\nIMPORTANT: Keep your private key secure and never share it!");

    Ok(())
}
