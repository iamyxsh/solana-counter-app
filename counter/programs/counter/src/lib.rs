use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<u64> {
        let mut count = &ctx.accounts.counter.count;
        count = &data;
        Ok(*count)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer= authority, space= 8 + 8)]
    counter: Account<'info, Counter>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub count: u64,
}
