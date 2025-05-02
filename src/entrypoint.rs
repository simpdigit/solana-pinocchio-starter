#![allow(unexpected_cfgs)]

use crate::instruction::{self, MyProgramInstrution};
use pinocchio::{
    account_info::AccountInfo,    
    program_entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult
};

#[cfg(target_os = "solana")]
use pinocchio::nostd_panic_handler;

#[cfg(any(target_os = "solana", test))]
use pinocchio::default_allocator;

use pinocchio_log::log;

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);

#[cfg(any(target_os = "solana", test))]
default_allocator!();

#[cfg(target_os = "solana")]
nostd_panic_handler!();


#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    //log instruction data
    log!("Instruction data length: {}", instruction_data.len());
    
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match MyProgramInstrution::try_from(ix_disc)? {
        MyProgramInstrution::Deposit => {
            log!("Ix:0");
            instruction::process_deposit(accounts, instruction_data)
        }
        MyProgramInstrution::Withdraw => {
            log!("Ix:1");
            instruction::process_withdraw(accounts)
        }
    }
}
