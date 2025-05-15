use bolt_lang::prelude::*;
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::{commit, delegate, ephemeral};
use ephemeral_rollups_sdk::cpi::DelegateConfig;
use ephemeral_rollups_sdk::ephem::{commit_accounts, commit_and_undelegate_accounts};


pub const TEST_PDA_SEED: &[u8] = b"test-pda";

declare_id!("48mfBprbc35YicVWfEZUk2VkaWPVHiTpMG7a4VkMSVpP");

#[program]
pub mod amoca_climate_insurance {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
