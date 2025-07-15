use anchor_lang::prelude::*;

declare_id!("2tdojmL85Ri9CzW8K8m7cXHAfxVDzSBjRhpG4fzF9biD");

#[program]
pub mod backend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
