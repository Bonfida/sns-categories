//! Remove category member
use crate::{
    error::SnsCategoriesError, state::category_metadata::CategoryMetadata, state::Tag,
    utils::get_hashed_name,
};
use {
    bonfida_utils::{
        checks::{check_account_key, check_account_owner, check_signer},
        BorshSize, InstructionsAccount,
    },
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program::invoke_signed,
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program,
    },
    spl_name_service::state::get_seeds_and_key,
};

#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Params {
    // The category name
    pub category_name: String,
    // The category member
    pub category_member: String,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The system program account
    pub system_program: &'a T,

    /// The SPL name service program
    pub name_service_program: &'a T,

    /// The fee payer account
    #[cons(writable, signer)]
    pub fee_payer: &'a T,

    /// The category metadata account
    #[cons(writable)]
    pub category_metadata: &'a T,

    /// The category metadata account
    #[cons(writable)]
    pub category_member: &'a T,

    pub central_state: &'a T,

    /// The required instruction signer
    #[cons(signer)]
    #[cfg(not(feature = "no-signer"))]
    pub signer: &'a T,
}

impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    pub fn parse(
        accounts: &'a [AccountInfo<'b>],
        _program_id: &Pubkey,
    ) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Accounts {
            system_program: next_account_info(accounts_iter)?,
            name_service_program: next_account_info(accounts_iter)?,
            fee_payer: next_account_info(accounts_iter)?,
            category_metadata: next_account_info(accounts_iter)?,
            category_member: next_account_info(accounts_iter)?,
            central_state: next_account_info(accounts_iter)?,
            #[cfg(not(feature = "no-signer"))]
            signer: next_account_info(accounts_iter)?,
        };

        // Check keys
        check_account_key(accounts.system_program, &system_program::ID)?;
        check_account_key(accounts.name_service_program, &spl_name_service::ID)?;

        // Check owners
        check_account_owner(accounts.category_metadata, &spl_name_service::ID)?;
        check_account_owner(accounts.category_member, &spl_name_service::ID)?;

        // Check signer
        check_signer(accounts.fee_payer)?;
        #[cfg(not(feature = "no-signer"))]
        check_signer(accounts.signer)?;

        Ok(accounts)
    }
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], params: Params) -> ProgramResult {
    let Params {
        category_name,
        category_member,
    } = params;
    let accounts = Accounts::parse(accounts, program_id)?;

    // Update parent information
    let hashed = get_hashed_name(&category_name);
    let (key, _) = get_seeds_and_key(
        &spl_name_service::ID,
        hashed,
        Some(&crate::central_state::KEY),
        None,
    );
    check_account_key(accounts.category_metadata, &key)?;

    let mut category_metadata =
        CategoryMetadata::from_buffer(accounts.category_metadata, Tag::CategoryMetadata)?;

    category_metadata.nb_of_registered_domains = category_metadata
        .nb_of_registered_domains
        .checked_sub(1)
        .ok_or(SnsCategoriesError::Overflow)?;

    let seeds: &[&[u8]] = &[&program_id.to_bytes(), &[crate::central_state::NONCE]];
    let ix = spl_name_service::instruction::update(
        spl_name_service::ID,
        0,
        category_metadata.try_to_vec().unwrap(),
        key,
        crate::central_state::KEY,
        None,
    )?;
    invoke_signed(
        &ix,
        &[
            accounts.name_service_program.clone(),
            accounts.category_metadata.clone(),
            accounts.central_state.clone(),
        ],
        &[seeds],
    )?;

    // Remove category member
    let hashed = get_hashed_name(&category_member);
    let (key, _) = get_seeds_and_key(
        &spl_name_service::ID,
        hashed.clone(),
        None,
        Some(accounts.category_metadata.key),
    );
    check_account_key(accounts.category_member, &key)?;

    let ix = spl_name_service::instruction::delete(
        spl_name_service::ID,
        key,
        crate::central_state::KEY,
        *accounts.fee_payer.key,
    )?;
    invoke_signed(
        &ix,
        &[
            accounts.name_service_program.clone(),
            accounts.system_program.clone(),
            accounts.fee_payer.clone(),
            accounts.category_member.clone(),
            accounts.central_state.clone(),
        ],
        &[seeds],
    )?;

    Ok(())
}
