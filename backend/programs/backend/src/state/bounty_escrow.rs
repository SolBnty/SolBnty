use anchor::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BountyEscrow {
    pub seed: u64, // Unique bounty identifier
    pub company: Pubkey, // Company who created the bounty
    pub mint: Pubkey, // Token mint
    pub total_amount: u64, // Total deposited funds for bounty
    pub amount_per_completion: u64, // Amount earned per completion
    pub max_completions: u64, // Maximum number of completions
    pub current_completions: u64, // Current number of completions
    pub is_active: bool, // If bounty is active or not
    pub uri: String, // Arweave metadata link
    pub bump: u8, // PDA bump
    pub expiry: Option<i64>, // Bounty expiry timestamp
}
