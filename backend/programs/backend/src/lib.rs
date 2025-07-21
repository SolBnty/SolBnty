#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod error;

use state::*;
use instructions::*;

declare_id!("2tdojmL85Ri9CzW8K8m7cXHAfxVDzSBjRhpG4fzF9biD");

#[program]
pub mod backend {
    use super::*;

    pub fn initialize_bounty(
        ctx: Context<InitBounty>, 
        seed: u64, 
        total_amount: u64,
        amount_per_completion: u64,
        max_completions: u32,
        current_completions: u32,
        uri: String, 
        expiry: Option<i64>
    ) -> Result<()> {
        ctx.accounts.initialize_escrow(seed, total_amount, amount_per_completion, max_completions, current_completions, uri, &ctx.bumps, expiry)?;
        ctx.accounts.transfer_sol_to_vault()
    }

    pub fn payout_user(ctx: Context<PayoutUser>) -> Result<()> {
        ctx.accounts.claim_bounty()
    }

    pub fn close_bounty(ctx: Context<CloseBounty>) -> Result<()> {
        ctx.accounts.refund_vault()
    }
}
