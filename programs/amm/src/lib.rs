use anchor_lang::prelude::*;

declare_id!("4huFTtn74L4MsymxXTqvBj6MqxeSr3tVXPvTxXoRzEad");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub struct PoolPair {
    pub token_one: Pubkey,
    pub token_one_amount: u64,
    pub token_two: Pubkey,
    pub token_two_amount: u64
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub enum LPAccountStatus {
    OPEN,
    CLOSED
}

#[account]
pub struct LPAccount {
    pub authority: Pubkey,
    pub lp_token_address: Pubkey,
    pub lp_pair: PoolPair,
    pub lp_token_amount: u64,
    pub status: LPAccountStatus,
    pub supply_ts: i64,
    pub updated_ts: i64,
    pub bump: u8,
}

#[account]
pub struct PoolPairAccount {
    pub lp_token_address: Pubkey,
    pub lp_pair: PoolPair,
    pub lp_token_supply: u64,
    pub name: String,
    pub created_ts: i64,
    pub updated_ts: i64,
    pub bump: u8
}
