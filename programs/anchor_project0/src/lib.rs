use anchor_lang::prelude::*;

declare_id!("FUBvQ9hMJ3RdDHW2oGKNiR1rp3hfdA5WmMnptbXAK38F");

#[program]
pub mod anchor_project0 {
    use super::*;

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {
        //todo
        Ok(())
    }

    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
        //todo
        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        //todo
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(initalizer_amount: u64)]
pub struct InitializeEscrow<'info> {
    pub initializer: AccountInfo<'info>,
    pub initializer_deposit_token_account: AccountInfo<'info>,
    pub initializer_receive_token_account: AccountInfo<'info>,
    pub vault_account: Account<'info, TokenAccount>,
    pub token_program: AccountInfo<'info>,
    pub escrow_account: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    pub taker: AccountInfo<'info>,
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    pub initializer_main_account: AccountInfo<'info>,
    pub escrow_account: ProgramAccount<'info, EscrowAccount>,
    pub vault_account: Account<'info, TokenAccount>,
    pub vault_authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    //todo
}

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    pub initializer: AccountInfo<'info>,
    pub initializer_deposit_token_account: AccountInfo<'info>,
    pub vault_account: Account<'info, TokenAccount>,
    pub vault_authority: AccountInfo<'info>,
    pub escrow_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initalizer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
}
