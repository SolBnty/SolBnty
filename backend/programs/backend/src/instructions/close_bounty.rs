use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::BountyEscrow;

#[derive(Accounts)]
pub struct Close<'info> {
    // Company which created the bounty
    #[account(mut)]
    pub company: Signer<'info>,

    // Escrow account
    #[account(
        mut,
        close = company,
        seeds = [b"bounty", company.key().as_ref(), &bounty.seed.to_le_bytes()],
        bump,
    )]
    pub bounty: Account<'info, BountyEscrow>,

    // Vault account owned by escrow
    #[account(
        mut,
        seeds = [b"vault", bounty.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> Close<'info> {
    pub fn refund_vault(&mut self) -> Result<()> {
        // Setup CPI context
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.company.to_account_info()
        };
        let cpi_program = self.system_program.to_account_info();
        let company_key = self.company.to_account_info().key.as_ref();
        let seeds = &[
            b"bounty", 
            company_key, 
            &self.bounty.seed.to_le_bytes()
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Refund all lamports from vault
        let amount = self.vault.lamports();
        transfer(cpi_ctx, amount)?;

        // Deactivate bounty
        self.bounty.is_active = false;
        
        Ok(())
    }
}