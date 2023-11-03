use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    msg!("Initialize the account data");

    let account_info_iter = &mut accounts.iter();
    let account = match next_account_info(account_info_iter) {
        Ok(account_info) => account_info,
        Err(error) => return Err(error),
    };

    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut data = match account.try_borrow_mut_data() {
        Ok(borrowed_data) => borrowed_data,
        Err(error) => return Err(error),
    };

    for byte in data.iter_mut() {
        *byte = 0;
    }

    Ok(())
}
