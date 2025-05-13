use bolt_lang::prelude::*;

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
