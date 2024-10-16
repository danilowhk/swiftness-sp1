//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use swiftness::TransformTo;
use swiftness_air::layout::starknet_with_keccak::Layout;

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.

    let stark_proof: swiftness::types::StarkProof =
        sp1_zkvm::io::read::<swiftness::StarkProof>().transform_to();

    let security_bits = stark_proof.config.security_bits();
    let result = stark_proof.verify::<Layout>(security_bits).unwrap();

    println!("{:?}", result);

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit(&result);
}
