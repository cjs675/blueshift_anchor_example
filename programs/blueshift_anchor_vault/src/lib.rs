use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("22222222222222222222222222222222222222222222");



#[program]
pub mod blueshift_anchor {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        // Check if vault is empty
        require_eq!(ctx.accounts.vault.lamports(), 0, VaultError::VaultAlreadyExists);

        // Ensure amount exceeds rent-exempt minimum
        let rent = Rent::get()?;
        require_gt!(amount, rent.minimum_balance(0), VaultError::InvalidAmount);

        // Transfer lamports from signer to vault
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                }
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        // Check if vault has any lamports
        let vault_lamports = ctx.accounts.vault.lamports();
        require_neq!(vault_lamports, 0, VaultError::InvalidAmount);

        // Create PDA signer seeds
        let signer_key = ctx.accounts.signer.key();
        let signer_seeds: &[&[u8]] = &[
            b"vault",
            signer_key.as_ref(),
            &[ctx.bumps.vault],
        ];

        // Transfer all lamports from vault to signer
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.signer.to_account_info(),
                },
                &[signer_seeds],
            ),
            vault_lamports,
        )?;

        Ok(())
    }
} 

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account( 
        mut,
        seeds = [b"vault", signer.key().as_ref()], 
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,
        

}

/*
#[derive(Accounts)]
pub struct VaultAction<'info> { 
    #[account(mut)] // mut constraint needed b/c we'll be mod'ing its lamports during transfers 
    pub signer: Signer<'info>, // owner of the vault, only one able to withdraw lamports after
                               // creating 
    #[account(
      mut,
      seeds = [b"vault", signer.key().as_ref()],
      bump,
    )] 
    pub vault: SystemAccount<'info>, // PDA derived from following seeds: [b"vault",
                                     // signer.key().as_ref()] that holds the lamports for the
                                     // signer 
                                     //
                                     // mut b/c will be mod'ing its lamports 
                                     // seeds & bump defines how to derive valid PDA from seeds 
    pub system_program: Program<'info, System>, // system program account that needs to be included 
                                                // since we're going to use the transfer ixn CPI 
                                                // from system program 
                                                //
                                                // checks if account is set to executable & that
                                                // the address is the System Program one 
}



 dont need many errors for small programs 
 we create 2 enums: 
    - VaultAlreadyExists: let us know if there are already some lamports in the account since it would 
                          mean that the vault exists already 
    - InvalidAmount: we cant deposit an amount that is less than the min. rent for basic acc 
                     so we check that the amount is > than that 


  Deposit 
    - deposit ixn performs following steps: 
        1. verifies vault is empty (has zero lamports) to prevent double deposits 
        2. ensures deposit amount exceeds rent-exempt min. for SystemAccount
        3. transfers lamports from signer to vault using CPI to system program 

 we implement these checks first: 

    - The two 'require' macros act like custom guard clauses: 
        - 'require_eq!' confirms vault is empty (preventing double deposits)
        - 'require_gt!' checks the amount clears the rent-exempt threshold 

 


    Withdraw 
        - ixn performs following steps: 
            1. verifies vault contains lamports (is not empty)
            2. uses vault's PDA to sign transfer on its own behalf 
            3. transfers all lamports from the vault back to the signer 

 - The security of this wd is guaranteed by 2 factors: 
    1. the vault's PDA is derived using the signer's pubkey, ensuring only the og depositor can withdraw
    2. PDAs ability to transfer is verified through seeds we provide to CpiContext::new_with_signer 

    - We can test our program against our unit test using: 
        
        anchor build 
    
    - will generate a .so file directly in target/deploy folder 
 */


