use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;

pub fn init_crypto_mail(
    program: &Program,
) -> Result<()> {
    let (account, _bump) = Pubkey::find_program_address(&[program.payer().as_ref()], &program.id());
    let tx: Signature = program
        .request()
        .accounts(crypto_mail::accounts::InitCryptoMail {
            account,
            user: program.payer(),
            system_program: system_program::ID,
        })
        .args(crypto_mail::instruction::InitCryptoMail {})
        .send()?;
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    println!("PDA Pubkey: {}", account.to_string());
    println!("------------------------------------------------------------");
    Ok(())
}