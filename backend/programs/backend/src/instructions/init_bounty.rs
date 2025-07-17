use anchor::prelude::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitBounty<'info> {
    #[account(mut)]
    pub company: Signer<'info>,

    #[account(
        init,
        seeds = [b"bounty", company.key().as_ref(), &seed.to_le_bytes()],
        bump,
        space = 8 + BountyEscrow::INIT_SPACE,
    )]
    pub bounty: Account<'info, BountyEscrow>,
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}