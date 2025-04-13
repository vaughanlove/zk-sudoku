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

use alloy_sol_types::SolType;
use clap::Parser;
use hex;
use fibonacci_lib::PublicValuesStruct;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

extern crate alloc;
use alloc::vec::Vec;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long, default_value = "20")]
    n: u32,

    // #[clap(value_parser = parse_hex)]
    // hex_input: Vec<u8>,
}
fn parse_hex(arg: &str) -> Result<Vec<u8>, hex::FromHexError> {
    let cleaned = arg.trim_start_matches("0x")  // Remove optional 0x prefix
        .replace(" ", "");                      // Remove any whitespace
    hex::decode(cleaned)
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);

    let user_input: Vec<u8> = vec![
        7, 5, 3, 8, 2, 1, 6, 9, 4, 1, 2, 4, 3, 6, 9, 5, 7, 8, 6, 8, 9, 4, 5, 7, 1, 2, 3, 2, 9, 1,
        5, 7, 3, 8, 4, 6, 8, 4, 7, 2, 1, 6, 9, 3, 5, 5, 3, 6, 9, 4, 8, 2, 1, 7, 3, 7, 2, 1, 8, 5,
        4, 6, 9, 4, 6, 5, 7, 9, 2, 3, 8, 1, 9, 1, 8, 6, 3, 4, 7, 5, 2,
    ];

    stdin.write(&user_input);

    println!("n: {}", args.n);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(FIBONACCI_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        let PublicValuesStruct { valid } = decoded;
        println!("valid: {}", valid);

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(FIBONACCI_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
