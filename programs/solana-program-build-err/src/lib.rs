pub use solana_program::alt_bn128::prelude::*;

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[error_code]
pub enum MyError {
    #[msg("alt_bn128 syscall error")]
    AltBn128,
}

#[program]
pub mod solana_program_build_err {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let input = [0u8; 128];
        let _ = alt_bn128_addition(&input).map_err(|_| MyError::AltBn128)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
