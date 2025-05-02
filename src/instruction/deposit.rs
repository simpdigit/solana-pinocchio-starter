use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{self},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::{CreateAccount, Transfer};
use pinocchio_log::log;
//pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

use crate::{
    error::MyProgramError,
    state::{load_ix_data, DataLen, VaultState},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DepositIxData {
    pub amount: u64,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl DataLen for DepositIxData {
    const LEN: usize = core::mem::size_of::<DepositIxData>();
}

pub fn process_deposit(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // checks for accounts
    log!("Starting deposit process");
    let [deposit_acc, vault_acc, state_acc, _sysvar_rent_acc, _system_program] = accounts else {
        log!("Error: Not enough account keys");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !deposit_acc.is_signer() {
        log!("Error: Deposit account is not a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !vault_acc.data_is_empty() && unsafe {!vault_acc.owner().eq(&crate::ID) } {
        log!("Error: Invalid vault account");
        return Err(MyProgramError::InvalidAccount.into());
    }

    let ix_data = match load_ix_data::<DepositIxData>(data) {
        Ok(data) => data,
        Err(_) => {
            log!("Error loading instruction data");
            log!("Instruction data length: {}", data.len());
            return Err(ProgramError::InvalidInstructionData);
        }
    };

    log!("Instruction data loaded successfully");

    let rent = match Rent::get() {
        Ok(rent) => rent,
        Err(_) => {
            log!("Error getting rent");
            return Err(ProgramError::AccountNotRentExempt);
        }
    };

    let bump = [ix_data.state_bump];
    // Signer seeds
    let signer_seeds = [
        Seed::from("state".as_bytes()),
        Seed::from(deposit_acc.key()),
        Seed::from(&bump),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    // Check if the state account needs to be created
    if state_acc.data_is_empty() {
        // Create the governance config account
        log!("Creating state account");
        CreateAccount {
            from: deposit_acc,
            to: state_acc,
            space: VaultState::LEN as u64,
            owner: &crate::ID,
            lamports: rent.minimum_balance(VaultState::LEN),
        }
        .invoke_signed(&signers)?;

        // Initialize the state
        VaultState::initialize(state_acc, ix_data.vault_bump, ix_data.state_bump)?;
        log!("State account initialized");
    } else if unsafe { !state_acc.owner().eq(&crate::ID) } {
        log!("Error: State account already initialized by another program");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let vault_pda = match pubkey::create_program_address(
        &[
            "pinocchio_vault_pda".as_bytes(),
            deposit_acc.key(),
            &[ix_data.vault_bump],
        ],
        &crate::ID,
    ) {
        Ok(pda) => pda,
        Err(_) => {
            log!("Error creating program address");
            return Err(ProgramError::InvalidSeeds);
        }
    };

    if vault_acc.key() != &vault_pda {
        log!("Error: Invalid vault account");
        return Err(ProgramError::InvalidAccountData);
    }
    // log the amount deposited
    log!("Depositing {} lamports", ix_data.amount);

    Transfer {
        from: deposit_acc,
        to: vault_acc,
        lamports: ix_data.amount as u64,
    }
    .invoke()?;

    Ok(())
}