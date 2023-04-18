pub use crate::processor::{add_member, create_category, remove_member};
use {
    bonfida_utils::InstructionsAccount,
    borsh::{BorshDeserialize, BorshSerialize},
    num_derive::FromPrimitive,
    solana_program::{instruction::Instruction, pubkey::Pubkey},
};
#[allow(missing_docs)]
#[derive(BorshDeserialize, BorshSerialize, FromPrimitive)]
pub enum ProgramInstruction {
    /// Create category account
    /// 
    /// | Index | Writable | Signer | Description                     |
    /// | ----------------------------------------------------------- |
    /// | 0     | ❌        | ❌      | The system program account      |
    /// | 1     | ❌        | ❌      | The SPL name service program    |
    /// | 2     | ✅        | ✅      | The fee payer account           |
    /// | 3     | ✅        | ❌      | The category metadata account   |
    /// | 4     | ❌        | ❌      |                                 |
    /// | 5     | ❌        | ✅      | The required instruction signer |
    CreateCategory,
    /// Create category account
    /// 
    /// | Index | Writable | Signer | Description                     |
    /// | ----------------------------------------------------------- |
    /// | 0     | ❌        | ❌      | The system program account      |
    /// | 1     | ❌        | ❌      | The SPL name service program    |
    /// | 2     | ✅        | ✅      | The fee payer account           |
    /// | 3     | ✅        | ❌      | The category metadata account   |
    /// | 4     | ✅        | ❌      | The category metadata account   |
    /// | 5     | ❌        | ❌      |                                 |
    /// | 6     | ❌        | ✅      | The required instruction signer |
    AddMember,
    /// Remove category member
    /// 
    /// | Index | Writable | Signer | Description                     |
    /// | ----------------------------------------------------------- |
    /// | 0     | ❌        | ❌      | The system program account      |
    /// | 1     | ❌        | ❌      | The SPL name service program    |
    /// | 2     | ✅        | ✅      | The fee payer account           |
    /// | 3     | ✅        | ❌      | The category metadata account   |
    /// | 4     | ✅        | ❌      | The category metadata account   |
    /// | 5     | ❌        | ❌      |                                 |
    /// | 6     | ❌        | ✅      | The required instruction signer |
    RemoveMember,
}
#[allow(missing_docs)]
pub fn create_category(
    accounts: create_category::Accounts<Pubkey>,
    params: create_category::Params,
) -> Instruction {
    accounts.get_instruction(crate::ID, ProgramInstruction::CreateCategory as u8, params)
}
#[allow(missing_docs)]
pub fn add_member(
    accounts: add_member::Accounts<Pubkey>,
    params: add_member::Params,
) -> Instruction {
    accounts.get_instruction(crate::ID, ProgramInstruction::AddMember as u8, params)
}
pub fn remove_member(
    accounts: remove_member::Accounts<Pubkey>,
    params: remove_member::Params,
) -> Instruction {
    accounts.get_instruction(crate::ID, ProgramInstruction::RemoveMember as u8, params)
}
