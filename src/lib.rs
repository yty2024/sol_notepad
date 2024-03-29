use anchor_lang::prelude::*;

declare_id!("7yViLKrasUtNHjdi5yknS1NnUfr7qs6e8zi9Tn2NaK6G");

#[error_code]
pub enum ErrorCode {
    #[msg("The provided content should not be longer than 500 characters")]
    ContentTooLong,
}

#[program]
pub mod solana_notepad {
    use super::*;

    pub fn send_note(ctx: Context<SendNote>, content: String) -> Result<()> {
        let note: &mut Account<Note> = &mut ctx.accounts.note;
        let author: &Signer = &&ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        if content.chars().count() > 500 {
            return Err(ErrorCode::ContentTooLong.into());
        }

        note.author = *author.key;
        note.timestamp = clock.unix_timestamp;
        note.content = content;

        Ok(())
    }

    pub fn update_note(ctx: Context<UpdateNote>, content: String) -> Result<()> {
        let note: &mut Account<Note> = &mut ctx.accounts.note;

        if content.chars().count() > 500 {
            return Err(ErrorCode::ContentTooLong.into());
        }

        note.content = content;

        Ok(())
    }

    pub fn delete_note(_ctx: Context<DeleteNote>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateNote<'info> {
    #[account(mut, has_one = author)]
    pub note: Account<'info, Note>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct SendNote<'info> {
    #[account(init, payer = author, space = Note::LEN)]
    pub note: Account<'info, Note>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteNote<'info> {
    #[account(mut, has_one = author, close = author)]
    pub note: Account<'info, Note>,
    pub author: Signer<'info>,
}

#[account]
pub struct Note {
    pub author: Pubkey,
    pub timestamp: i64,
    pub content: String,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_CONTENT_LENGTH: usize = 500 * 4;

impl Note {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH;
}
