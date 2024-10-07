//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use fibonacci_lib::{fibonacci, PublicValuesStruct};
use swiftness::{json_parser, stark_proof};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.

    let input = include_str!("../../examples/proofs/dex/cairo0_stone5_example_proof.json");
    let proof_json = serde_json::from_str::<json_parser::StarkProof>(input).unwrap();
    let stark_proof: swiftness::types::StarkProof = swiftness::types::StarkProof::try_from(proof_json).unwrap();
    let security_bits = stark_proof.config.security_bits();
    // let result = stark_proof.verify::<Layout>(security_bits)?;
    // println!("{:?}", result);

    let n = sp1_zkvm::io::read::<u32>();

    // Compute the n'th fibonacci number using a function from the workspace lib crate.
    let (a, b) = fibonacci(n);

    // Encode the public values of the program.
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { n, a, b });

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&bytes);
}
