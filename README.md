# Solana Pinocchio Starter

This project is a starter template for building Solana programs using the Pinocchio framework. It provides a basic structure for developing, testing, and deploying Solana smart contracts with enhanced productivity and safety features.

## Features

- Basic Solana program structure
- Integration with Pinocchio framework
- Example instructions for deposit and withdraw operations
- Unit testing setup with Mollusk SVM

## Prerequisites

- Rust and Cargo (latest stable version)
- Solana CLI tools
- Node.js and npm (for client-side development, if needed)

## Project Structure

- `src/`: Contains the main program logic
  - `entrypoint.rs`: Program entrypoint
  - `instruction/`: Instruction implementations
  - `state/`: Program state management
  - `error.rs`: Custom error definitions
  - `lib.rs`: Main library file
- `tests/`: Unit tests
- `Cargo.toml`: Project dependencies and configuration

## Development

To add new instructions or modify existing ones, update the relevant files in the `src/instruction/` directory. Make sure to update the `process_instruction` function in `lib.rs` to handle any new instructions.

## Testing

This project uses Mollusk SVM for testing. You can add or modify tests in the `tests/unit_tests.rs` file.


## Getting Started

1. Clone the repository:

    `git clone https://github.com/simpdigit/solana-pinocchio-starter.git`
    
    `cd solana-pinocchio-starter`

2. Build the program:

    `cargo build-sbf`

3. Test the program locally:

    `cargo test-sbf`

4. Deploy using the Solana CLI:

    `solana-keygen -o program-keypair.json`
    
    `solana program deploy target/deploy/solana_pinocchio_starter.so --program-id program-keypair.json`

5. Test the deployed program:

    `npm run test:devnet`