use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke};

use anchor_lang::solana_program::instruction::{
    get_processed_sibling_instruction,
    get_stack_height,
    TRANSACTION_LEVEL_STACK_HEIGHT,
};

#[cfg(not(feature = "child"))]
declare_id!("H4vTCnnisqDnZW2ByCcUVbxuB73Xni4D4VzAt7ywuoHj");

#[cfg(feature = "child")]
declare_id!("ExQUucxSnQbAuLoTQxKo4m5woiMmh1YiLgZ2UW11zhEN");

#[program]
pub mod instructions_reflection {
    use super::*;

    pub fn parent_call(ctx: Context<ParentCall>) -> Result<()> {
        let stack_height = get_stack_height();
        msg!("[parent] Current stack height: {} (transaction level height: {})", stack_height, TRANSACTION_LEVEL_STACK_HEIGHT);

        invoke(&Instruction {
            program_id: ctx.accounts.memo_program.key(),
            accounts: vec![],
            data: vec![b'm', b'e', b'm', b'o', b'#', b'p', b'.', b'1'], // memo#p.1
        }, &vec![])?;

        invoke(&Instruction {
            program_id: ctx.accounts.memo_program.key(),
            accounts: vec![],
            data: vec![b'm', b'e', b'm', b'o', b'#', b'p', b'.', b'2'], // memo#p.2
        }, &vec![])?;

        invoke(&Instruction {
            program_id: ctx.accounts.child_program.key(),
            accounts: vec![
                AccountMeta {
                    pubkey: ctx.accounts.memo_program.key(),
                    is_signer: false,
                    is_writable: false,
                },
            ],
            data: vec![0xf8, 0x51, 0xda, 0x23, 0x3c, 0x1d, 0x3d, 0x90], // disc for child_call
        }, &vec![])?;

        invoke(&Instruction {
            program_id: ctx.accounts.child_program.key(),
            accounts: vec![
                AccountMeta {
                    pubkey: ctx.accounts.memo_program.key(),
                    is_signer: false,
                    is_writable: false,
                },
            ],
            data: vec![0xf8, 0x51, 0xda, 0x23, 0x3c, 0x1d, 0x3d, 0x90], // disc for child_call
        }, &vec![])?;

        // get processed sibling
        msg!("Processing sibling instructions in parent_call...");
        let mut index = 0;
        while let Some(instruction) = get_processed_sibling_instruction(index) {
            msg!("[parent] Sibling instruction program: {:?}, data: {:?}", instruction.program_id, String::from_utf8(instruction.data).unwrap_or("Invalid UTF-8".to_string()));
            index += 1;
        }

        Ok(())
    }

    pub fn child_call(ctx: Context<ChildCall>) -> Result<()> {
        let stack_height = get_stack_height();
        msg!("[child] Current stack height: {}", stack_height);

        invoke(&Instruction {
            program_id: ctx.accounts.memo_program.key(),
            accounts: vec![],
            data: vec![b'm', b'e', b'm', b'o', b'#', b'c', b'.', b'1'], // memo#c.1
        }, &vec![])?;

        invoke(&Instruction {
            program_id: ctx.accounts.memo_program.key(),
            accounts: vec![],
            data: vec![b'm', b'e', b'm', b'o', b'#', b'c', b'.', b'2'], // memo#c.2
        }, &vec![])?;

        // get processed sibling
        msg!("Processing sibling instructions in child_call...");
        let mut index = 0;
        while let Some(instruction) = get_processed_sibling_instruction(index) {
            msg!("[child] Sibling instruction program: {:?}, data: {:?}", instruction.program_id, String::from_utf8(instruction.data).unwrap_or("Invalid UTF-8".to_string()));
            index += 1;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ParentCall<'info> {
    /// CHECK: just for test
    pub memo_program: UncheckedAccount<'info>,
    /// CHECK: just for test
    pub child_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct ChildCall<'info> {
    /// CHECK: just for test
    pub memo_program: UncheckedAccount<'info>,
}
