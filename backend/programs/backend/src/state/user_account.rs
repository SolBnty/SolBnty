use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub id: u64,
    #[max_len(30)]
    pub first_name: String,
    #[max_len(30)]
    pub last_name: String,
    pub pubkey: Pubkey,
    #[max_len(30)]
    pub x_username: String,
}