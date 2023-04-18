use {
    bonfida_utils::BorshSize,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::pubkey,
    solana_program::pubkey::Pubkey,
};

pub mod category_member;
pub mod category_metadata;

pub const SIGNER: Pubkey = pubkey!("G9tP6ZonwNj2qTdPpCrTsrCQgDovppxjCddfidNwFq5n"); // TODO change

#[derive(BorshSerialize, BorshDeserialize, BorshSize, PartialEq)]
#[allow(missing_docs)]
pub enum Tag {
    Uninitialized,
    CategoryMetadata,
    CategoryMetadataClosed,
    CategoryMember,
    CategoryMemberClosed,
}
