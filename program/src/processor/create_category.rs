//! Create category account

use crate::{
    cpi,
    state::{category_metadata::CategoryMetadata, CATEGORY_TLD},
    utils::{get_category_metadata_key, get_hashed_name},
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
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program,
    },
};

#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Params {
    // The category name
    pub category_name: String,
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

    /// The central state
    pub central_state: &'a T,

    /// The Category TLD
    pub category_tld: &'a T,

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
            central_state: next_account_info(accounts_iter)?,
            category_tld: next_account_info(accounts_iter)?,
            #[cfg(not(feature = "no-signer"))]
            signer: next_account_info(accounts_iter)?,
        };

        // Check keys
        check_account_key(accounts.system_program, &system_program::ID)?;
        check_account_key(accounts.name_service_program, &spl_name_service::ID)?;
        check_account_key(accounts.central_state, &crate::central_state::KEY)?;
        check_account_key(accounts.category_tld, &CATEGORY_TLD)?;
        #[cfg(not(feature = "no-signer"))]
        check_account_key(accounts.signer, &crate::state::SIGNER)?;

        // Check owners
        check_account_owner(accounts.category_metadata, &system_program::ID)?;

        // Check signer
        check_signer(accounts.fee_payer)?;
        #[cfg(not(feature = "no-signer"))]
        check_signer(accounts.signer)?;

        Ok(accounts)
    }
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], params: Params) -> ProgramResult {
    let Params { category_name } = params;
    let accounts = Accounts::parse(accounts, program_id)?;

    let hashed = get_hashed_name(&category_name);
    let key = get_category_metadata_key(&category_name);
    check_account_key(accounts.category_metadata, &key)?;

    let category_metadata = CategoryMetadata::new(&category_name);
    let size = category_metadata.borsh_len();

    cpi::name_service_create(
        hashed,
        size,
        cpi::NameServiceCreateAccounts {
            name_account: accounts.category_metadata,
            fee_payer: accounts.fee_payer,
            name_service_program: accounts.name_service_program,
            system_program: accounts.system_program,
            signer: accounts.central_state,
            parent_account: accounts.category_tld,
        },
    )?;

    cpi::name_service_update(
        0,
        category_metadata.try_to_vec()?,
        cpi::NameServiceUpdateAccounts {
            name_account: accounts.category_metadata,
            name_service_program: accounts.name_service_program,
            signer: accounts.central_state,
        },
    )?;

    Ok(())
}
