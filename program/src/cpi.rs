use std::convert::TryInto;

use solana_program::{
    account_info::AccountInfo, program::invoke_signed, program_error::ProgramError,
    program_pack::Pack, rent::Rent, sysvar::Sysvar,
};
use spl_name_service::{instruction::NameRegistryInstruction, state::NameRecordHeader};

pub struct NameServiceCreateAccounts<'a, 'b> {
    pub name_account: &'b AccountInfo<'a>,
    pub fee_payer: &'b AccountInfo<'a>,
    pub name_service_program: &'b AccountInfo<'a>,
    pub system_program: &'b AccountInfo<'a>,
    pub signer: &'b AccountInfo<'a>,
    pub parent_account: &'b AccountInfo<'a>,
}

pub fn name_service_create(
    hashed_name: Vec<u8>,
    size: usize,
    accounts: NameServiceCreateAccounts,
) -> Result<(), ProgramError> {
    let lamports = Rent::get()?.minimum_balance(size.checked_add(NameRecordHeader::LEN).unwrap());
    let ix = spl_name_service::instruction::create(
        spl_name_service::ID,
        NameRegistryInstruction::Create {
            hashed_name,
            lamports,
            space: size.try_into().unwrap(),
        },
        *accounts.name_account.key,
        *accounts.fee_payer.key,
        crate::central_state::KEY,
        None,
        Some(*accounts.parent_account.key),
        Some(crate::central_state::KEY),
    )?;
    invoke_signed(
        &ix,
        &[
            accounts.name_service_program.clone(),
            accounts.system_program.clone(),
            accounts.fee_payer.clone(),
            accounts.name_account.clone(),
            accounts.signer.clone(),
            accounts.parent_account.clone(),
        ],
        &[&crate::central_state::SIGNER_SEEDS],
    )?;
    Ok(())
}

pub struct NameServiceUpdateAccounts<'a, 'b> {
    pub name_account: &'b AccountInfo<'a>,
    pub name_service_program: &'b AccountInfo<'a>,
    pub signer: &'b AccountInfo<'a>,
}

pub fn name_service_update(
    offset: u32,
    data: Vec<u8>,
    accounts: NameServiceUpdateAccounts,
) -> Result<(), ProgramError> {
    let ix = spl_name_service::instruction::update(
        spl_name_service::ID,
        offset,
        data,
        *accounts.name_account.key,
        crate::central_state::KEY,
        None,
    )?;
    invoke_signed(
        &ix,
        &[
            accounts.name_service_program.clone(),
            accounts.name_account.clone(),
            accounts.signer.clone(),
        ],
        &[&crate::central_state::SIGNER_SEEDS],
    )?;
    Ok(())
}

pub struct NameServiceDeleteAccounts<'a, 'b> {
    pub name_account: &'b AccountInfo<'a>,
    pub name_service_program: &'b AccountInfo<'a>,
    pub system_program: &'b AccountInfo<'a>,
    pub signer: &'b AccountInfo<'a>,
    pub refund_target: &'b AccountInfo<'a>,
}

pub fn name_service_delete(accounts: NameServiceDeleteAccounts) -> Result<(), ProgramError> {
    let ix = spl_name_service::instruction::delete(
        spl_name_service::ID,
        *accounts.name_account.key,
        crate::central_state::KEY,
        *accounts.refund_target.key,
    )?;
    invoke_signed(
        &ix,
        &[
            accounts.name_service_program.clone(),
            accounts.system_program.clone(),
            accounts.refund_target.clone(),
            accounts.name_account.clone(),
            accounts.signer.clone(),
        ],
        &[&crate::central_state::SIGNER_SEEDS],
    )?;
    Ok(())
}
