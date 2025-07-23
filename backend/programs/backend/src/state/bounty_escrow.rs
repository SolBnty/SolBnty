use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BountyEscrow {
    pub seed: u64, // Unique bounty identifier
    pub company: Pubkey, // Company who created the bounty
    pub expiry: Option<i64>, // Bounty expiry timestamp
    pub total_amount: u64, // Total deposited funds for bounty
    pub amount_per_completion: u64, // Amount earned per completion
    pub max_completions: u32, // Maximum number of completions
    pub current_completions: u32, // Current number of completions
    pub is_active: bool, // If bounty is active or not
    pub latitude: f64,
    pub longitude: f64,
    #[max_len(30)]
    pub title: String, // Title of bounty
    #[max_len(125)]
    pub description: String, // Description of bounty
    pub bump: u8, // PDA bump
    pub vault_bump: u8,

    // #[max_len(32)]
    // pub uri: String, // Arweave metadata link
}
