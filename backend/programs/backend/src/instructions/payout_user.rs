use anchor_lang::prelude::*;

use crate::state::BountyEscrow;


#[derive(Accounts)]
pub struct PayoutUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub company: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bounty", company.key().as_ref(), &bounty.seed.to_le_bytes()],
        bump = bounty.bump,
    )]
    pub bounty: Account<'info, BountyEscrow>,

    #[account(
        mut,
        seeds = [b"vault", bounty.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}