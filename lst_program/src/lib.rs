use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Burn, Token, TokenAccount};

declare_id!("64pMzn8nuvgJ9ja7gh2hV6wpg5Jt7xoReLxjVJkcwY5k");

#[program]
pub mod lst_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, admin: Pubkey, fee_basis_points: u16) -> Result<()> {
        let pool_state = &mut ctx.accounts.pool_state;
        pool_state.admin = admin;
        pool_state.total_staked = 0;
        pool_state.total_minted = 0;
        pool_state.rewards_compounded = 0;
        pool_state.fee_basis_points = fee_basis_points;
        pool_state.paused = false;
        Ok(())
    }

    pub fn stake_sol(ctx: Context<StakeSOL>, amount: u64) -> Result<()> {
         let pool_state = &mut ctx.accounts.pool_state;
         require!(!pool_state.paused, ErrorCode::Paused);

         // Calculate fees and net amount
          let fee = (amount * pool_state.fee_basis_points as u64) / 10_000;
          let net_amount = amount - fee;

               pool_state.total_staked += net_amount;
               pool_state.total_minted += net_amount;

           // Create CpiContext for minting tokens
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // Mint LST tokens to the user's associated token account
    token::mint_to(cpi_ctx, net_amount)?;

    // Transfer fees to the admin's fee account
    **ctx.accounts.admin_fee_account.to_account_info().lamports.borrow_mut() += fee;

    emit!(StakeEvent {
        user: *ctx.accounts.user.key,
        amount,
        fee,
    });

    Ok(())
 }

    pub fn withdraw_sol(ctx: Context<WithdrawSOL>, amount: u64) -> Result<()> {
          let pool_state = &mut ctx.accounts.pool_state;
          require!(!pool_state.paused, ErrorCode::Paused);

          // Calculate fees and net amount
          let fee = (amount * pool_state.fee_basis_points as u64) / 10_000;
          let net_amount = amount - fee;

           pool_state.total_staked -= net_amount;
           pool_state.total_minted -= net_amount;

       // Create CpiContext for burning tokens
        let cpi_accounts = Burn {
             mint: ctx.accounts.mint.to_account_info(),
             from: ctx.accounts.user_token_account.to_account_info(),
             authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // Burn LST tokens
    token::burn(cpi_ctx, amount)?;

    // Transfer net amount of SOL back to the user
    **ctx.accounts.user.to_account_info().lamports.borrow_mut() += net_amount;

    // Transfer fees to the admin's fee account
    **ctx.accounts.admin_fee_account.to_account_info().lamports.borrow_mut() += fee;

    emit!(WithdrawEvent {
        user: *ctx.accounts.user.key,
        amount,
        fee,
    });

    Ok(())
}

    pub fn auto_compound_rewards(ctx: Context<AutoCompoundRewards>) -> Result<()> {
        let pool_state = &mut ctx.accounts.pool_state;

        let rewards = calculate_rewards(pool_state.total_staked);
        pool_state.rewards_compounded += rewards;
        pool_state.total_staked += rewards;

        emit!(CompoundRewardsEvent {
            rewards,
            total_staked: pool_state.total_staked,
        });

        Ok(())
    }

    pub fn redelegate(ctx: Context<Redelegate>, new_validator: Pubkey) -> Result<()> {
        // Add redelegation logic
        emit!(RedelegateEvent {
            validator: new_validator,
        });

        Ok(())
    }

    pub fn admin_update(ctx: Context<AdminUpdate>, new_admin: Pubkey) -> Result<()> {
        let pool_state = &mut ctx.accounts.pool_state;
        pool_state.admin = new_admin;

        emit!(AdminUpdateEvent {
            new_admin,
        });

        Ok(())
    }

    pub fn pause(ctx: Context<AdminUpdate>, paused: bool) -> Result<()> {
        let pool_state = &mut ctx.accounts.pool_state;
        pool_state.paused = paused;

        emit!(PauseEvent {
            paused,
        });

        Ok(())
    }

    pub fn get_pool_stats(ctx: Context<GetStats>) -> Result<PoolStats> {
        let pool_state = &ctx.accounts.pool_state;
        Ok(PoolStats {
            total_staked: pool_state.total_staked,
            total_minted: pool_state.total_minted,
            rewards_compounded: pool_state.rewards_compounded,
            fee_basis_points: pool_state.fee_basis_points,
        })
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + PoolState::LEN)]
    pub pool_state: Account<'info, PoolState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub admin_fee_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeSOL<'info> {
    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe because we control the mint authority
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub admin_fee_account: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSOL<'info> {
    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub admin_fee_account: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AutoCompoundRewards<'info> {
    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,
}

#[derive(Accounts)]
pub struct Redelegate<'info> {
    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,
    #[account(signer)]
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct AdminUpdate<'info> {
    #[account(mut, has_one = admin)]
    pub pool_state: Account<'info, PoolState>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetStats<'info> {
    pub pool_state: Account<'info, PoolState>,
}

#[account]
pub struct PoolState {
    pub admin: Pubkey,
    pub total_staked: u64,
    pub total_minted: u64,
    pub rewards_compounded: u64,
    pub fee_basis_points: u16,
    pub paused: bool,
}

impl PoolState {
    const LEN: usize = 8 + 32 + 8 + 8 + 8 + 2 + 1; // Discriminator + fields
}

#[event]
pub struct StakeEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub fee: u64,
}

#[event]
pub struct WithdrawEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub fee: u64,
}

#[event]
pub struct CompoundRewardsEvent {
    pub rewards: u64,
    pub total_staked: u64,
}

#[event]
pub struct RedelegateEvent {
    pub validator: Pubkey,
}

#[event]
pub struct AdminUpdateEvent {
    pub new_admin: Pubkey,
}

#[event]
pub struct PauseEvent {
    pub paused: bool,
}

#[account]
pub struct PoolStats {
    pub total_staked: u64,
    pub total_minted: u64,
    pub rewards_compounded: u64,
    pub fee_basis_points: u16,
}

fn calculate_rewards(total_staked: u64) -> u64 {
    total_staked / 100 // 1% rewards
}

#[error_code]
pub enum ErrorCode {
    #[msg("The program is currently paused.")]
    Paused,
}
