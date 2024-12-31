use anchor_lang::prelude::*;

declare_id!("8cUTFacQRJT3wCRDiB4QiziawoS1HAGdcpoMZwPke1uK");

#[program]
pub mod counter_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Dervie the PDA and the bump:
        let ( counter_pda, bump ) = Pubkey::find_program_address(
            &[b"counter", ctx.accounts.user.key().as_ref()],
            ctx.program_id,
        );

        // Fetched the counter account and initializing to 0:
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.bump = bump;    // storing the bump fetched for later use.
        msg!("Counter account created for you at {:?}", counter_pda);

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Counter value is increased to {:?}", counter.count);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 + 1,   // Total space for the account, calculated using InitSpace
        seeds = [b"counter", user.key().as_ref()],  // seeds to create the PDA
        bump
    )]
    pub counter: Account<'info, Counter>,   // The counter PDA account
    #[account(mut)]
    pub user: Signer<'info>, // The user who is initializing the counter account
    pub system_program: Program<'info, System>  // All the account creation is done by system program
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"counter", user.key().as_ref()],
        bump = counter.bump
    )]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
    pub bump: u8
}

//#[derive(InitSpace)] - Macro provided by anchor which automatically calculates the space required by the account.
