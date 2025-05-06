use anchor_lang::prelude::*;

declare_id!("9h15EiWiXzZgMpvqcovcXkgUZGhLjcSuCKLL2HeoncxP");

#[program]
pub mod guessing_games {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
