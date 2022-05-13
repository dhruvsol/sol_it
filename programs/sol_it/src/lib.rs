use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("C12tVda17vqMYuzEjT5NB9g4pDZeq9myejp6viPKuqsc");

#[program]
pub mod sol_it {
    use super::*;
pub fn send_reddit(ctx:Context<SendRedit>,content:String,imghash:String)->Result<()>{
    let reddit : &mut Account<SubMsg> = &mut ctx.accounts.sol_it;
    let author :&Signer=&ctx.accounts.author;
    let clock:Clock =Clock::get().unwrap();

    reddit.author = *author.key;
    reddit.timestamp = clock.unix_timestamp;
    reddit.content = content;
    reddit.image_hash= imghash;
    Ok(())
}
 
}

#[derive(Accounts)]
pub struct  SendRedit<'info>{
    #[account(init,payer=author,space=8+32+8+8+(280*4)+8+32)]
    pub sol_it:Account<'info,SubMsg>,
    #[account(mut)]
    pub author :Signer<'info>,
    pub system_program:AccountInfo<'info>,
}



#[account]
pub struct SubMsg {
    pub author : Pubkey,
    pub timestamp:i64,
    pub content : String,
    pub image_hash : String 
}
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH : usize = 8;
const STRING_PREFIX :usize=8; //to store the string 
const MAX_CONTENT :usize = 4*280 ; // content max length
const IMG_HASH :usize = 32;

