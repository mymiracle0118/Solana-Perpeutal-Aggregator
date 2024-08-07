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

    pub fn call_create_increase_position_request(
        ctx: Context<CreateIncreasePositionRequestData>, 
        _size_usd_delta: u64,
        _collateral_token_delta: u64,
        _side: Side,
        _request_type: RequestType,
        _price_slippage: u64,
        _jupiter_minimum_out: u64,
        _trigger_price: u64,
        _trigger_above_threshold: bool,
        _counter: u64
    ) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.program.to_account_info(),
            perpetual_cpi::cpi::accounts::CreateIncreasePositionRequest { 
              owner: ctx.accounts.owner.to_account_info(), 
              funding_account: ctx.accounts.funding_account.to_account_info(), 
              perpetuals: ctx.accounts.perpetuals.to_account_info(), 
              pool: ctx.accounts.pool.to_account_info(), 
              position: ctx.accounts.position.to_account_info(), 
              position_request: ctx.accounts.position_request.to_account_info(),
              position_request_ata: ctx.accounts.position_request_ata.to_account_info(),
              custody: ctx.accounts.custody.to_account_info(), 
              custody_oracle_account: ctx.accounts.custody_oracle_account.to_account_info(),
              collateral_custody: ctx.accounts.collateral_custody.to_account_info(),
              input_mint: ctx.accounts.input_mint.to_account_info(),
              referral: ctx.accounts.referral.to_account_info(),
              token_program: ctx.accounts.token_program.to_account_info(),
              associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
              system_program: ctx.accounts.system_program.to_account_info(),
              event_authority: ctx.accounts.event_authority.to_account_info(),
              program: ctx.accounts.program.to_account_info()
            },
        );

        let params = perpetual_cpi::CreateIncreasePositionRequestParams {
            size_usd_delta: _size_usd_delta,
            collateral_token_delta: _collateral_token_delta,
            side: _side.into(),
            request_type: _request_type.into(),
            price_slippage: Some(_price_slippage),
            jupiter_minimum_out: Some(_jupiter_minimum_out),
            trigger_price: Some(_jupiter_minimum_out),
            trigger_above_threshold: Some(_trigger_above_threshold),
            counter: _counter
        };
        perpetual_cpi::cpi::create_increase_position_request(cpi_ctx, params)?;
        Ok(())
    }

    // Conversion from local Side to perpetual_cpi::Side
    impl From<Side> for perpetual_cpi::Side {
        fn from(item: Side) -> Self {
            match item {
                Side::None => perpetual_cpi::Side::None,
                Side::Long => perpetual_cpi::Side::Long,
                Side::Short => perpetual_cpi::Side::Short,
            }
        }
    }

    // Conversion from local RequestType to perpetual_cpi::RequestType
    impl From<RequestType> for perpetual_cpi::RequestType {
        fn from(item: RequestType) -> Self {
            match item {
                RequestType::Market => perpetual_cpi::RequestType::Market,
                RequestType::Trigger => perpetual_cpi::RequestType::Trigger,
            }
        }
    }
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum Side {
    None,
    Long,
    Short
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum RequestType {
    Market,
    Trigger,
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

    /// CHECK:
    #[account(mut)]
    funding_account: AccountInfo<'info>,
    perpetuals: Account<'info, perpetual_cpi::Perpetuals>,
    pool: Account<'info, perpetual_cpi::Pool>,
    #[account(mut)]
    position: Account<'info, perpetual_cpi::Position>,
    #[account(mut)]
    position_request: Account<'info, perpetual_cpi::PositionRequest>,
    /// CHECK
    #[account(mut)]
    position_request_ata: AccountInfo<'info>,
    custody: Account<'info, perpetual_cpi::Custody>,
    custody_oracle_account: Account<'info, perpetual_cpi::TestOracle>,
    collateral_custody: Account<'info, perpetual_cpi::Custody>,
    /// CHECK
    input_mint: AccountInfo<'info>,
    referral: Program<'info, perpetual_cpi::program::Perpetuals>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, Token>,
    system_program : Program<'info, System>,
    /// CHECK
    event_authority: AccountInfo<'info>,
    program: Program<'info, perpetual_cpi::program::Perpetuals>
}

