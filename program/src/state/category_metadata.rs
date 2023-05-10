use crate::error::SnsCategoriesError;
use bonfida_utils::BorshSize;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, program_pack::Pack};
use spl_name_service::state::NameRecordHeader;

#[derive(BorshSerialize, BorshDeserialize, BorshSize)]
#[allow(missing_docs)]
#[repr(C)]
pub struct CategoryMetadata {
    pub tag: super::Tag,
    pub nb_of_registered_domains: u32,
    pub name: String,
}

impl CategoryMetadata {
    pub fn new(name: &str) -> Self {
        Self {
            tag: super::Tag::CategoryMetadata,
            name: name.to_owned(),
            nb_of_registered_domains: 0,
        }
    }

    pub fn from_buffer(a: &AccountInfo, expected_tag: super::Tag) -> Result<Self, ProgramError> {
        let mut data = &a.data.borrow()[NameRecordHeader::LEN..] as &[u8];
        if data[0] != expected_tag as u8 {
            return Err(SnsCategoriesError::DataTypeMismatch.into());
        }
        let result = Self::deserialize(&mut data)?;
        Ok(result)
    }
}
