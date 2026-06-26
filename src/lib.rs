use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::{entrypoint, ProgramResult};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize)]
struct Counter {
    count: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
enum InstructionData {
    Increase,
    Decrease,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut iterate = accounts.iter();
    let counter_acc = next_account_info(&mut iterate)?;

    if !counter_acc.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut counter = Counter::try_from_slice(&counter_acc.data.borrow())?;
    let instruction = InstructionData::try_from_slice(instruction_data)?;

    match instruction {
        InstructionData::Decrease => {
            counter.count = counter.count.saturating_sub(1);
        }
        InstructionData::Increase => {
            counter.count = counter.count.saturating_add(1);
        }
    }

    counter.serialize(&mut *counter_acc.data.borrow_mut())?;

    Ok(())
}
