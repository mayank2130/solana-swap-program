use anchor_lang::prelude::*;

declare_id!("AjR1S8tTbCkiw18urVfP8GErdLYYpQVQfX815AACP5rj");

#[program]
pub mod swap_program {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
