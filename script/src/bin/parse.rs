//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use clap::Parser;
use hex::ToHex;
use sp1_sdk::SP1ProofWithPublicValues;
use std::fs;

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    proof: String,
}

fn main() {
    // Parse the command line arguments.
    let args = Args::parse();

    let proof = fs::read(args.proof).unwrap();

    let parsed_proof: SP1ProofWithPublicValues =
        bincode::deserialize(&proof).expect("Deserialization failed");

    println!("{:?}", parsed_proof.bytes().encode_hex::<String>());
}
