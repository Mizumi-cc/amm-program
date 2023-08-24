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
