use anchor_lang::prelude::*;

declare_id!("7zP3k3zQxUYBsVj4dGVJ1xvhtto6oLTTWT9pk1CJzJ1s");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
