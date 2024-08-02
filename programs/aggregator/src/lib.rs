// use anchor_lang::prelude::*;
use borsh::{BorshDeserialize,BorshSerialize};
use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize, Key, solana_program::program::{invoke}};
use perpetual_cpi::perpetuals::*;
use perpetual_cpi::accounts::*;
use perpetual_cpi::program::Perpetuals;

// use presale_cpi::presale::*;
use presale_cpi::{presale::*, Pool};
use presale_cpi::accounts::*;
use presale_cpi::program::Presale;

declare_id!("zZPBTeoK4ywiLpzpHJxLDLzVe9zybTEw9zgiLxMYqx5");
#[program]
pub mod perpeutal_aggregator {
    // use perpetual_cpi::UpdateDecreasePositionRequestParams;

    use super::*;

    pub fn call_update_decrease_position_request_data(ctx: Context<UpdateDecreasePositionRequestData>, _params: perpetual_cpi::UpdateDecreasePositionRequestParams) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.perpetual_program.to_account_info(),
            perpetual_cpi::cpi::accounts::UpdateDecreasePositionRequest { 
              owner: ctx.accounts.owner.to_account_info(), 
              perpetuals: ctx.accounts.perpetuals.to_account_info(), 
              pool: ctx.accounts.pool.to_account_info(), 
              position: ctx.accounts.position.to_account_info(), 
              position_request: ctx.accounts.position_request.to_account_info(), 
              custody: ctx.accounts.custody.to_account_info(), 
              custody_oracle_account: ctx.accounts.custody_oracle_account.to_account_info()
              // custody_pythnet_price_account: ctx.accounts.custody_pythnet_price_account.to_account_info(),
              // perpetual_program: ctx.accounts.perpetual_program.to_account_info()
            },
        );
        perpetual_cpi::cpi::update_decrease_position_request(cpi_ctx, _params)?;
        Ok(())
    }

    pub fn modify_pool(ctx: Context<ModifyPoolData>, flag: bool) -> Result<()> {
      let cpi_ctx = CpiContext::new(
          ctx.accounts.presale_prgram.to_account_info(),
          presale_cpi::cpi::accounts::SetPause {
              owner: ctx.accounts.owner.to_account_info(),
              pool: ctx.accounts.pool.to_account_info(),
          }
      );
      presale_cpi::cpi::set_pause(cpi_ctx, flag)?;
      Ok(())
    }
}
#[derive(Accounts)]
pub struct ModifyPoolData<'info> {
    #[account(mut)]
    owner : Signer<'info>,

    /// CHECK: pool status
    #[account(mut)]
    pool : Account<'info, presale_cpi::Pool>,

    pub presale_prgram: Program<'info, Presale>
}

#[derive(Accounts)]
pub struct UpdateDecreasePositionRequestData<'info> {
    #[account(mut)]
    owner : Signer<'info>,
    perpetuals: Account<'info, perpetual_cpi::Perpetuals>,
    pool: Account<'info, perpetual_cpi::Pool>,
    position: Account<'info, perpetual_cpi::Position>,
    #[account(mut)]
    position_request: Account<'info, perpetual_cpi::PositionRequest>,
    custody: Account<'info, perpetual_cpi::Custody>,
    custody_oracle_account: Account<'info, perpetual_cpi::TestOracle>,
    // custody_doves_price_account: AccountInfo<'info>,
    // custody_pythnet_price_account: AccountInfo<'info>,
    perpetual_program: Program<'info, perpetual_cpi::program::Perpetuals>
}

