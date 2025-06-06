use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "CrabChain")]
#[command(about = "A simple blockchain implementation in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // New wallet generation
    WalletNew,
    WalletAddress{
        #[arg(short, long)]
        keyfile: String,
    },
    
    Balances,
    
    Send{
        #[arg(long)]
        keyfile: String,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        amount: u64
    },
    
    
    Mine{
        #[arg(long)]
        miner: String,

    },

    WalletList
    
    
}