use anchor_lang::prelude::*;
// use anchor_lang::prelude::Pubkey;
// use anchor_lang::prelude::{Key, Signer};

declare_id!("GjgwVLdzuYpkivHkR9XtJ1r4ttwRtAB4tUv246uH3U4T");

#[program]
pub mod test_for_signature {

    // use anchor_lang::solana_program::{
    //     program::{invoke},
    //     system_instruction::{transfer}
    // };

    use super::*;
    pub fn create_user(ctx: Context<CreateUser>, user_name:String) -> ProgramResult {
        let multi_sig_wallet = &mut ctx.accounts.user_account;
        multi_sig_wallet.user_name = user_name;
        multi_sig_wallet.key = ctx.accounts.user.key();
        Ok(())
    }

    pub fn create_message_account(ctx: Context<CreateMessageAccount>, message: String) -> ProgramResult {
        let message_account = &mut ctx.accounts.message_account;
        let user_1_obj = &mut ctx.accounts.user_1;
        message_account.user_1 = *user_1_obj.to_account_info().unsigned_key();
        message_account.message = message;
        Ok(())
    }

    pub fn update_message(ctx: Context<UpdateMessage>,new_message: String) -> ProgramResult {
        let message_account = &mut ctx.accounts.message_account;
        if *ctx.accounts.user_1.to_account_info().key != message_account.user_1 {
            // return Err(ErrorCode::InvalidSigner.into());
        }
        message_account.message = new_message;
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init, payer = user, space = 8 + 64 + 64 + 64 + 64)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMessage<'info> {
    #[account(mut)]
    pub message_account: Account<'info, Message>,
    #[account(mut)]
    pub user_1: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateMessageAccount<'info> {
    #[account(init, payer = owner, space = 8 + 64 + 64 + 64 + 64)]
    pub message_account: Account<'info, Message>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub user_1: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct User {
    pub user_name: String,
    pub key: Pubkey
}

#[account]
pub struct Message {
    pub user_1: Pubkey,
    pub message: String
}

// #[error]
// pub enum ErrorCode {
//     #[msg("Not enough lamports in wallet")]
//     NotEnoughLamports,
//     #[msg("previous contribution sum doesnot match with new contribution sum")]
//     InvalidBalances,
//     #[msg("Not a valid signer")]
//     InvalidSigner,
// }