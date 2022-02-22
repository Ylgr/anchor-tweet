use anchor_lang::prelude::*;

declare_id!("B4bnDwYieT1r8Kqc2SRKovp2TVyQJ4hxFFHznyNzkuBi");

#[program]
pub mod anchor_tweet {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
