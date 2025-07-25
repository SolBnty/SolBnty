use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::BountyEscrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitBounty<'info> {
    // Company creating the bounty
    #[account(mut)]
    pub company: Signer<'info>,

    // Escrow account
    #[account(
        init,
        payer = company,
        seeds = [b"bounty", company.key().as_ref(), &seed.to_le_bytes()],
        bump,
        space = 8 + BountyEscrow::INIT_SPACE,
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

impl<'info> InitBounty<'info> {
    // Set fields of BountyEscrow struct
    pub fn initialize_escrow(&mut self, 
        seed: u64, 
        total_amount: u64,
        amount_per_completion: u64,
        max_completions: u32,
        current_completions: u32,
        latitude: f64,
        longitude: f64,
        title: String, 
        description: String,
        bumps: &InitBountyBumps,
        expiry: Option<i64>,
    ) -> Result<()> {
        self.bounty.set_inner(BountyEscrow { 
            seed, 
            company: self.company.key(), 
            expiry, 
            total_amount, 
            amount_per_completion, 
            max_completions, 
            current_completions, 
            is_active: true, 
            latitude,
            longitude,
            title,
            description, 
            bump: bumps.bounty, 
            vault_bump: bumps.vault, 
        });

        Ok(())
    }

    // Transfer bounty funds from company to the vault
    pub fn transfer_sol_to_vault(&mut self) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.company.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_program = self.system_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, self.bounty.total_amount)
    }
}