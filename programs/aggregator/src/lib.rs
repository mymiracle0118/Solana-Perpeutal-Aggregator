// use anchor_lang::prelude::*;
use borsh::{BorshDeserialize,BorshSerialize};
use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize, Key, solana_program::program::{invoke}};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use perpetual_cpi::perpetuals::*;
use perpetual_cpi::accounts::*;
use perpetual_cpi::program::Perpetuals;

declare_id!("zZPBTeoK4ywiLpzpHJxLDLzVe9zybTEw9zgiLxMYqx5");
#[program]
pub mod perpeutal_aggregator {
    use perpetual_cpi::UpdateDecreasePositionRequestParams;

    use super::*;

    pub fn call_update_decrease_position_request_data(ctx: Context<UpdateDecreasePositionRequestData>, _size_usd_delta: u64, _trigger_price: u64) -> Result<()> {
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

        let params = perpetual_cpi::UpdateDecreasePositionRequestParams {
            size_usd_delta: _size_usd_delta,
            trigger_price: _trigger_price,
        };
        perpetual_cpi::cpi::update_decrease_position_request(cpi_ctx, params)?;
        Ok(())
    }

    pub fn call_create_increase_position_request(ctx: Context<CreateIncreasePositionRequestData>, _size_usd_delta: u64, _trigger_price: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.program.to_account_info(),
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

        let params = perpetual_cpi::UpdateDecreasePositionRequestParams {
            size_usd_delta: _size_usd_delta,
            trigger_price: _trigger_price,
        };
        perpetual_cpi::cpi::update_decrease_position_request(cpi_ctx, params)?;
        Ok(())
    }
}

pub struct UpdateDecreasePositionRequestParams {
    pub size_usd_delta: u64,
    pub trigger_price: u64,
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

#[derive(Accounts)]
pub struct CreateIncreasePositionRequestData<'info> {
    #[account(mut)]
    owner : Signer<'info>,    
    #[account(mut)]
    funding_account: AccountInfo<'info>,
    perpetuals: Account<'info, perpetual_cpi::Perpetuals>,
    pool: Account<'info, perpetual_cpi::Pool>,
    #[account(mut)]
    position: Account<'info, perpetual_cpi::Position>,
    #[account(mut)]
    position_request: Account<'info, perpetual_cpi::PositionRequest>,
    #[account(mut)]
    position_request_ata: AccountInfo<'info>,
    custody: Account<'info, perpetual_cpi::Custody>,
    custody_oracle_account: Account<'info, perpetual_cpi::TestOracle>,
    collateral_custody: Account<'info, perpetual_cpi::Custody>,
    input_mint: AccountInfo<'info>,
    referral: Program<'info, perpetual_cpi::program::Perpetuals>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, Token>,
    system_program : Program<'info, System>,
    program: Program<'info, perpetual_cpi::program::Perpetuals>
}

