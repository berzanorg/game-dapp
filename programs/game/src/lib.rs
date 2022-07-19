use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod game {
    use super::*;

    pub fn create_user_stats(ctx: Context<CreateUserStats>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateUserStats {}
