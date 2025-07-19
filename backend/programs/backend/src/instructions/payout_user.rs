use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp, system_program::{transfer, Transfer}};

use crate::{state::BountyEscrow, error::ErrorCode};


#[derive(Accounts)]
pub struct PayoutUser<'info> {
    // User with verified submission
    #[account(mut)]
    pub user: Signer<'info>,

    // Company paying out the bounty
    #[account()]
    pub company: SystemAccount<'info>,

    // Escrow account
    #[account(
        mut,
        seeds = [b"bounty", company.key().as_ref(), &bounty.seed.to_le_bytes()],
        bump = bounty.bump,
    )]
    pub bounty: Account<'info, BountyEscrow>,

    // Vault account
    #[account(
        mut,
        seeds = [b"vault", bounty.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> PayoutUser<'info> {
    // Transfer payout from vault to user
    pub fn claim_bounty(&mut self) -> Result<()> {
        let bounty = &mut self.bounty;

        // Ensure bounty is active
        require!(bounty.current_completions < bounty.max_completions, ErrorCode::MaxCompletionsReached);
        require!(bounty.is_active == true, ErrorCode::BountyNotActive);

        // Ensure not past bounty expiry time
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        if let Some(expiry) = bounty.expiry {
            require!(now <= expiry , ErrorCode::BountyExpired);
        };

        // Setup CPI
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };
        let cpi_program = self.system_program.to_account_info();
        let company_key = self.company.to_account_info().key.as_ref();
        let seeds = &[
            b"bounty", 
            company_key, 
            &bounty.seed.to_le_bytes()
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Transfer from vault
        transfer(cpi_ctx, bounty.amount_per_completion)?;

        // Update bounty info
        bounty.current_completions += 1;

        Ok(())
    }
}