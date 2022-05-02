use anchor_lang::prelude::*;

declare_id!("C12tVda17vqMYuzEjT5NB9g4pDZeq9myejp6viPKuqsc");

#[program]
pub mod sol_it {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
