use clap::{Args, Parser, Subcommand};
use rug::Integer;

#[derive(Parser, Debug)]
#[command(
    author = "Loukis", 
    version = "1.1", 
    about = "RSA implemented in Rust", 
    propagate_version = true,
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// generate the public and private keys pair
    Keygen(Keygen),

    /// encrypt string using RSA public keys
    Encrypt {
        /// string to be encrypted
        text: String,

        /// RSA public key pair
        #[arg(num_args = 2)]
        keys: Vec<Integer>,
    },

    /// decrypt string using RSA private keys
    Decrypt {
        /// string to be decrypted
        text: String,

        /// RSA private key pair
        #[arg(num_args = 2)]
        keys: Vec<Integer>,
    },
}

#[derive(Debug, Args)]
struct Keygen {
    /// define the value of p, randomly generated if not passed
    #[arg(short, value_parser = prime)]
    p: Option<Integer>,

    /// define the value of q, randomly generated if not passed
    #[arg(short)]
    q: Option<Integer>,

    /// print the keys
    #[arg(long)]
    print: bool,

    /// name of the files that will store the keys
    #[arg(long, default_value = "RSA-key")]
    name: String,

    /// bit length of the generated numbers
    #[arg(long, default_value_t = 128)]
    bit_length: u32,
}

fn prime(n: &str) -> Result<Integer, String> {
    let x = Integer::from(Integer::parse(n).unwrap());
    if rsa::is_prime(&x) {
        Ok(x)
    } else {
        Err(format!("{n} is not prime"))
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Keygen(Keygen {
            p,
            q,
            print,
            name,
            bit_length,
        }) => rsa::keygen(p, q, *bit_length, name.to_owned(), *print),
        Commands::Encrypt { text, keys } => rsa::encrypt(text.to_owned(), (keys.get(0).unwrap(), keys.get(1).unwrap())),
        Commands::Decrypt { text, keys } => rsa::decrypt(text.to_owned(), (keys.get(0).unwrap(), keys.get(1).unwrap())),
    }
}
