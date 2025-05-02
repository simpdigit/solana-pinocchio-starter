import {
    Connection,
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    Transaction,
    sendAndConfirmTransaction,
    TransactionInstruction,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
} from '@solana/web3.js';
import fs from 'fs';

// Replace with your program ID
const PROGRAM_ID = new PublicKey('GTaQUFVc7fmZ4XHiNJtPJoHj3AJrUjckcNCFnCDVBVxe');

// Define the payer's public key
const PAYER_PUBLIC_KEY = new PublicKey('Em3BGhWEgsFwbEmSQsEnPVcyzLihfdKFTcxbjf9vj4LH');

// VAULT_PDA is derived from a seed and the program ID
const [VAULT_PDA, vaultBump] = PublicKey.findProgramAddressSync(
    [Buffer.from('pinocchio_vault_pda'), PAYER_PUBLIC_KEY.toBuffer()],
    PROGRAM_ID
);

// STATE_PDA is derived from a seed and the program ID
const [STATE_PDA, stateBump] = PublicKey.findProgramAddressSync(
    [Buffer.from('state'), PAYER_PUBLIC_KEY.toBuffer()],
    PROGRAM_ID
);

async function main() {
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

    // Load the payer keypair from the JSON file
    const payerKeypairPath = 'local-keypair.json';
    const payerKeypairData = JSON.parse(fs.readFileSync(payerKeypairPath, 'utf8'));
    const payer = Keypair.fromSecretKey(Uint8Array.from(payerKeypairData));

    // Deposit Test
    await deposit(connection, payer);

    // Withdraw Test
    await withdraw(connection, payer);
}

async function deposit(connection: Connection, payer: Keypair) {
    // Define the instruction data
    const amount = 0.526; // Example amount in SOL
    const instructionData = Buffer.alloc(17) // 1 byte for instruction type + 8 bytes for amount +
    // + 1 byte for vault_bump + 1 byte for state_bump + 6 bytes for padding (rust optimzation)
    instructionData.writeUInt8(0, 0); // Instruction type (0 for deposit)
    instructionData.writeBigUInt64LE(BigInt(Math.floor(amount * LAMPORTS_PER_SOL)), 1); // Write amount as u64
    instructionData.writeUInt8(vaultBump, 9); // Write vault_bump as u8
    instructionData.writeUInt8(stateBump, 10); // Write state_bump as u8
    
    // Create the transaction instruction
    const instruction = new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
            { pubkey: payer.publicKey, isSigner: true, isWritable: true },
            { pubkey: VAULT_PDA, isSigner: false, isWritable: true },
            { pubkey: STATE_PDA, isSigner: false, isWritable: true },
            { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        data: instructionData,
    });

    // Create and send the transaction
    const transaction = new Transaction().add(instruction);
    const signature = await sendAndConfirmTransaction(connection, transaction, [payer]);
    console.log('Deposit transaction signature:', signature);
}

// withdraw function
async function withdraw(connection: Connection, payer: Keypair) {
    // Define the instruction data
    const instructionData = Buffer.alloc(1)
    instructionData.writeUInt8(1, 0); // Instruction type (1 for withdraw)
    
    // Create the transaction instruction
    const instruction = new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
            { pubkey: payer.publicKey, isSigner: true, isWritable: true },
            { pubkey: VAULT_PDA, isSigner: false, isWritable: true },
            { pubkey: STATE_PDA, isSigner: false, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        data: instructionData,
    });
    // Create and send the transaction
    const transaction = new Transaction().add(instruction);
    const signature = await sendAndConfirmTransaction(connection, transaction, [payer]);
    console.log('Withdraw transaction signature:', signature);
}

main().catch(err => {
    console.error(err);
});