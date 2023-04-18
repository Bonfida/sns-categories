use {
    bonfida_utils::BorshSize,
    borsh::{BorshDeserialize, BorshSerialize},
};

pub mod category_member;
pub mod category_metadata;

#[derive(BorshSerialize, BorshDeserialize, BorshSize, PartialEq)]
#[allow(missing_docs)]
pub enum Tag {
    Uninitialized,
    CategoryMetadata,
    CategoryMetadataClosed,
    CategoryMember,
    CategoryMemberClosed,
}
