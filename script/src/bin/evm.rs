//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can have an
//! EVM-Compatible proof generated which can be verified on-chain.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release --bin evm -- --system groth16
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release --bin evm -- --system plonk
//! ```

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use sp1_sdk::{HashableKey, ProverClient, SP1ProofWithPublicValues};
use std::{fs, path::PathBuf};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SWIFTNESS_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the EVM command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct EVMArgs {
    #[clap(long, value_enum, default_value = "groth16")]
    system: ProofSystem,
    #[clap(long)]
    proof: String,
}

/// Enum representing the available proof systems
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ProofSystem {
    Plonk,
    Groth16,
}

/// A fixture that can be used to test the verification of SP1 zkVM proofs inside Solidity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SP1SwiftnessFixture {
    vkey: String,
    public_values: String,
    proof: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = EVMArgs::parse();

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the program.
    let (_pk, vk) = client.setup(SWIFTNESS_ELF);

    let proof: SP1ProofWithPublicValues =
        bincode::deserialize(&fs::read(args.proof).unwrap()).expect("Deserialization failed");

    // Deserialize the public values.
    let bytes = proof.public_values.as_slice();

    // Create the testing fixture so we can test things end-to-end.
    let fixture = SP1SwiftnessFixture {
        vkey: vk.bytes32().to_string(),
        public_values: format!("0x{}", hex::encode(bytes)),
        proof: format!("0x{}", hex::encode(proof.bytes())),
    };

    // The verification key is used to verify that the proof corresponds to the execution of the
    // program on the given input.
    //
    // Note that the verification key stays the same regardless of the input.
    println!("Verification Key: {}", fixture.vkey);

    // The public values are the values which are publicly committed to by the zkVM.
    //
    // If you need to expose the inputs or outputs of your program, you should commit them in
    // the public values.
    println!("Public Values: {}", fixture.public_values);

    // The proof proves to the verifier that the program was executed with some inputs that led to
    // the give public values.
    println!("Proof Bytes: {}", fixture.proof);

    // Save the fixture to a file.
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../contracts/src/fixtures");
    std::fs::create_dir_all(&fixture_path).expect("failed to create fixture path");
    std::fs::write(
        fixture_path.join(format!("{:?}-fixture.json", args.system).to_lowercase()),
        serde_json::to_string_pretty(&fixture).unwrap(),
    )
    .expect("failed to write fixture");
}
