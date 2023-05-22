use {
    borsh::BorshDeserialize,
    num_traits::FromPrimitive,
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
        pubkey::Pubkey,
    },
};

use crate::instruction::ProgramInstruction;

pub mod add_member;
pub mod create_category;
pub mod remove_member;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Beginning processing");
        let instruction = FromPrimitive::from_u8(instruction_data[0])
            .ok_or(ProgramError::InvalidInstructionData)?;
        let instruction_data = &instruction_data[1..];
        msg!("Instruction unpacked");

        match instruction {
            ProgramInstruction::CreateCategory => {
                msg!("Instruction: Create category");
                let params = create_category::Params::try_from_slice(instruction_data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                create_category::process(program_id, accounts, params)?;
            }
            ProgramInstruction::AddMember => {
                msg!("Instruction: Add member");
                let params = add_member::Params::try_from_slice(instruction_data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                add_member::process(program_id, accounts, params)?;
            }
            ProgramInstruction::RemoveMember => {
                msg!("Instruction: Remove member");
                let params = remove_member::Params::try_from_slice(instruction_data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                remove_member::process(program_id, accounts, params)?;
            }
        }

        Ok(())
    }
}
