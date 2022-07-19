use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod game {
    use super::*;

    pub fn create_user_stats(ctx: Context<CreateUserStats>, name: String) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.level = 0;
        // Error handling
        require!(name.as_bytes().len() <= 64, MyError::NameTooLong);

        user_stats.name = name;
        user_stats.bump = *ctx.bumps.get("user-stats").unwrap();

        Ok(())
    }
}

#[account]
pub struct UserStats {
    level: u8,
    name: String,
    bump: u8,
}

#[error_code]
pub enum MyError {
    #[msg("Name is longer than 64 bytes")]
    NameTooLong,
}

// Validation struct 
#[derive(Accounts)]
pub struct CreateUserStats<'info> {
    // The user is the signer
    #[account(mut)]
    pub user: Signer<'info>,

    // space: 8 discriminator + 1 level + 4 name length + 64 name + 1 bump
    #[account(
        init, payer = user, space = 8 + 1 + 4 + 64 + 1,
        seeds = [b"user-stats", user.key().as_ref()], bump
    )]
    user_stats: Account<'info, UserStats>,

    // System program itself
    system_program: Program<'info, System>,
}
