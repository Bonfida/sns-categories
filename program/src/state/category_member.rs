use crate::error::SnsCategoriesError;
use bonfida_utils::BorshSize;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey,
};
use spl_name_service::state::NameRecordHeader;

#[derive(BorshSerialize, BorshDeserialize, BorshSize)]
#[allow(missing_docs)]
#[repr(C)]
pub struct CategoryMember {
    pub tag: super::Tag,
    pub name: String,
    pub domain_key: Pubkey,
}

impl CategoryMember {
    pub const LEN: usize = std::mem::size_of::<Self>();
}

/// An example PDA state, serialized using Borsh //TODO
#[allow(missing_docs)]
impl CategoryMember {
    pub const SEED: &'static [u8; 6] = b"member";

    pub fn new(name: &str, domain_key: &Pubkey) -> Self {
        Self {
            tag: super::Tag::CategoryMember,
            name: name.to_owned(),
            domain_key: *domain_key,
        }
    }

    pub fn from_buffer(a: &AccountInfo, expected_tag: super::Tag) -> Result<Self, ProgramError> {
        let mut data = &a.data.borrow()[NameRecordHeader::LEN..] as &[u8];
        if data[0] != expected_tag as u8 && data[0] != super::Tag::Uninitialized as u8 {
            return Err(SnsCategoriesError::DataTypeMismatch.into());
        }
        let result = Self::deserialize(&mut data)?;
        Ok(result)
    }
}
