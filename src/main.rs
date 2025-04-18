//! CLI utility tool for one-on-one private messaging on Nostr for CLI and agent use
//!
//! It uses the `nostr_sdk` crate to interact with the Nostr network. It sends and receives direct messages that are encrypted with NIP-17 by default.
mod mcp;
mod utils;
mod process_management;

use std::sync::Arc;
use tokio::sync::Mutex;
use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;
use std::{
    io::{self, Read},
    process::exit,
};
use mcp::Chat;
use rmcp::{ServiceExt, transport::stdio};
use utils::wait_for_message;
use utils::run_command_on_message;
use utils::listen_for_messages;
use log;
use env_logger;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Pubkey of the target user to talk to via DMs (in bech32 format)
    #[arg(long, env = "TARGET_PUBKEY")]
    target_pubkey: String,

    /// The private key (nsec) identity to use on the DMs
    #[arg(long, env = "NSEC")]
    nsec: String,

    /// Relay URL to use for sending/receiving messages
    #[arg(long, env = "RELAY_URL", default_value = "wss://relay.damus.io")]
    relay: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sends a private message via NIP-17. If the message is omitted, reads it from stdin.
    Send {
        /// The message to send
        message: Option<String>,
    },
    /// Waits for a private NIP-17 message to be received and prints the decrypted contents to stdout once received.
    Wait,
    /// Listens for private NIP-17 messages to be received and prints the decrypted contents to stdout after each one is received.
    Listen,
    /// Starts an MCP server to allow an AI agent to manage the conversation
    Mcp,
    /// Runs a specified shell command each time it receives a NIP-17 direct message, passing the decrypted message contents to it via stdin.
    Onmessage {
        #[clap(required = true)]
        shell_command: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    env_logger::init();

    // Parse our keys from the provided identity (nsec)
    let keys = Keys::parse(&args.nsec)?;
    let our_pubkey = keys.public_key();

    // Parse the target public key
    let target_pk: PublicKey = args.target_pubkey.parse()?;

    // Create a client with our keys
    let client = Client::builder()
        .signer(keys.clone())
        .build();

    // Add the relay specified by the user
    client.add_relay(&args.relay).await?;
    client.connect().await;

    match args.command {
        Commands::Send { message } => {
            // Obtain the message from argument or via stdin
            let content = match message {
                Some(msg) => msg,
                None => {
                    let mut buffer = String::new();
                    io::stdin().read_to_string(&mut buffer)?;
                    buffer
                }
            };

            eprintln!("Sending direct message to {}...", args.target_pubkey);
            client.send_private_msg(target_pk, content, []).await?;
            eprintln!("Message sent!");
            exit(0);
        }
        Commands::Wait => {
            let message = wait_for_message(&client, &our_pubkey, &target_pk).await?;
            println!("{}", message);
        },
        Commands::Listen => {
            let message_callback = {
                async move |message: String| {
                    println!("{}", message);
                    false   // Never returns
                }
            };

            listen_for_messages(
                &client,
                &our_pubkey,
                &target_pk,
                Arc::new(Mutex::new(message_callback)),
            ).await?;
        },
        Commands::Mcp => {
            // Create and serve our chat service
            let service = Chat::new(client.clone(), our_pubkey, target_pk).serve(stdio()).await.inspect_err(|e| {
                log::error!("{e}");
            })?;
            service.waiting().await?;
        },
        Commands::Onmessage { shell_command } => {
            log::info!(
                "Listening for messages"
            );
            run_command_on_message(&client, &our_pubkey, &target_pk, &shell_command).await?;
        }
    }

    Ok(())
}
