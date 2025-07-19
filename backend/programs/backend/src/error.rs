use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum completions reached for this bounty.")]
    MaxCompletionsReached,
    #[msg("Vault does not have enough funds.")]
    InsufficientVaultFunds,
    #[msg("Bounty is not currently active.")]
    BountyNotActive,
    #[msg("This bounty has expired.")]
    BountyExpired
}