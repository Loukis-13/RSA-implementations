use clap::{Args, Parser, Subcommand};
use num_bigint::BigUint;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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
        keys: Vec<BigUint>,
    },

    /// decrypt string using RSA private keys
    Decrypt {
        /// string to be decrypted
        text: String,

        /// RSA private key pair
        #[arg(num_args = 2)]
        keys: Vec<BigUint>,
    },
}

#[derive(Debug, Args)]
struct Keygen {
    /// define the value of p, randomly generated if not passed
    #[arg(short, value_parser = prime)]
    p: Option<BigUint>,

    /// define the value of q, randomly generated if not passed
    #[arg(short)]
    q: Option<BigUint>,

    /// print the keys
    #[arg(long)]
    print: bool,

    /// name of the files that will store the keys
    #[arg(long, default_value = "RSA-key")]
    name: String,

    /// bit length of the generated numbers
    #[arg(long, default_value_t = 128)]
    bit_length: u64,
}

fn prime(n: &str) -> Result<BigUint, String> {
    let x = BigUint::parse_bytes(n.as_bytes(), 10).unwrap_or_default();
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

    // rsa::keygen();

    // let e = BigUint::parse_bytes("A82B9B0614D0B026860852C340C35D8F1ED3D4B63AA555BD2DDB7ECF53543DD5".as_bytes(), 16).unwrap();
    // let d = BigUint::parse_bytes("C6154303669DD585D882854F0E1DE226291D388EEFDCF4DAD45FE145710E4B51".as_bytes(), 16).unwrap();
    // let n = BigUint::parse_bytes("E40A773D799B788DB0E247C689A1B015361C35ADAA6AEB8125B3E1BE24E63CB5".as_bytes(), 16).unwrap();

    // rsa::encrypt("FATEC Ribeirão Preto".to_string(), (&e, &n));
    // rsa::decrypt("88625869962292453309365246213140538441884451653796843924987677378868465989301".to_string(), (&d, &n));
}
