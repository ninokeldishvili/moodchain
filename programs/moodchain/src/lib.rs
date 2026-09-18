use anchor_lang::prelude::*;

declare_id!("BCGPooyokpwR9GVV4ffbEyVteGus3JDYB7Y7oFHKC4Cz");

#[program]
pub mod mood_tracker {
    use super::*;

    /// Creates (if needed) and/or updates the caller's mood account.
    pub fn set_mood(ctx: Context<SetMood>, mood: Mood) -> Result<()> {
        let mood_account = &mut ctx.accounts.mood_account;

        mood_account.owner = ctx.accounts.user.key();
        mood_account.mood = mood;
        mood_account.last_updated = Clock::get()?.unix_timestamp;

        emit!(MoodUpdated {
            owner: mood_account.owner,
            mood: mood_account.mood,
            last_updated: mood_account.last_updated,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetMood<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + MoodAccount::INIT_SPACE,
        seeds = [b"mood", user.key().as_ref()],
        bump
    )]
    pub mood_account: Account<'info, MoodAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct MoodAccount {
    pub owner: Pubkey,
    pub mood: Mood,
    pub last_updated: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum Mood {
    Happy,
    Sad,
    Tired,
    Excited,
    Angry,
    Neutral,
    NeedCoffee
}

#[event]
pub struct MoodUpdated {
    pub owner: Pubkey,
    pub mood: Mood,
    pub last_updated: i64,
}