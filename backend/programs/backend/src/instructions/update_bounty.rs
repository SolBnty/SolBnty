use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::BountyEscrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct UpdateBounty<'info> {
    // Company creating the bounty
    #[account(mut)]
    pub company: Signer<'info>,

    // Escrow account
    #[account(
        seeds = [b"bounty", company.key().as_ref(), &seed.to_le_bytes()],
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

impl<'info> UpdateBounty<'info> {
    // Set fields of BountyEscrow struct
    pub fn update_bounty(
        &mut self, 
        total_amount: Option<u64>,
        amount_per_completion: Option<u64>,
        max_completions: Option<u32>,
        current_completions: Option<u32>,
        longitude: Option<f64>,
        latitude: Option<f64>,
        title: Option<String>, 
        description: Option<String>,
        expiry: Option<Option<i64>>, // Option wrapped for nullable field updates
    ) -> Result<()>{
        // Update bounty data
        if let Some(t) = total_amount {
            if self.bounty.total_amount != t {
                self.update_vault_sol()?;
            }
            self.bounty.total_amount = t;
        }
        if let Some(a) = amount_per_completion {
            self.bounty.amount_per_completion = a;
        }
        if let Some(m) = max_completions {
            self.bounty.max_completions = m;
        }
        if let Some(c) = current_completions {
            self.bounty.current_completions = c;
        }
        if let Some(lat) = latitude {
            self.bounty.latitude = lat;
        }
        if let Some(long) = longitude {
            self.bounty.longitude = long;
        }
        if let Some(t) = title {
            self.bounty.title = t;
        }
        if let Some(d) = description {
            self.bounty.description = d;
        }
        if let Some(e) = expiry {
            self.bounty.expiry = e;
        }

        Ok(())
    }

    // Transfer bounty funds from company to the vault
    pub fn update_vault_sol(&mut self) -> Result<()> {
        let vault_lamports = self.vault.to_account_info().lamports();
        let total_amount = self.bounty.total_amount;

        // Calculate delta between desired total and current vault
        if total_amount > vault_lamports {
            // Needs more funds: transfer from company to vault
            let amount_needed = total_amount - vault_lamports;
            let cpi_accounts = Transfer {
                from: self.company.to_account_info(),
                to: self.vault.to_account_info(),
            };
            let cpi_program = self.system_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            transfer(cpi_ctx, amount_needed)?;
        } else if total_amount < vault_lamports {
            // Excess funds: refund back to company
            let amount_needed = vault_lamports - total_amount;
            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.company.to_account_info(),
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
            transfer(cpi_ctx, amount_needed)?;
        }

        Ok(())
    }
}