use solana_program::entrypoint::{entrypoint, ProgramResult};
use solana_program::pubkey::{Pubkey};
use solana_program::account_info::{AccountInfo, next_account_info};

entrypoint!(process_instruction);

#[derive[BorshSerialize, BorshDeserialize]]
struct Counter{
    count: u32
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

    let mut iterate = accounts.iter();
    let counter_acc= next_account_info(&mut iterate)?;

    if !counter_acc.is_signer {
        return Err(solana_program::program_error::Missing_Required_Signatures)
    }

    let counter = Counter::try_from_slice(*counter_account.data.borrow());
    Ok(())
}
