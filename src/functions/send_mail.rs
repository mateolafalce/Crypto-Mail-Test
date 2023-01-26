use anchor_client::{
    anchor_lang::{
        system_program::ID
    },
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn send_mail(
    program: &Program,
    receiver: Pubkey,
    text: String
) -> Result<()> {
    let timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    let (sender, _bump) = Pubkey::find_program_address(&[program.payer().as_ref()], &program.id());
    let (mail, _bump) = Pubkey::find_program_address(&[timestamp.to_le_bytes().as_ref()], &program.id());
    let len: u16 = text.len() as u16;
    let tx: Signature = program
        .request()
        .accounts(crypto_mail::accounts::SendMail {
            mail,
            sender,
            receiver,
            user: program.payer(),
            system_program: ID,
        })
        .args(crypto_mail::instruction::SendMail {
            mail: text,
            len,
            timestamp
        })
        .send()?;
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    println!("Mail PDA: {}", mail.to_string());
    println!("------------------------------------------------------------");
    Ok(())
}