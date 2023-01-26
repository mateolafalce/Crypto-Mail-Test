use anyhow::Result;
pub mod functions;
mod constants;
use constants::program;

fn main() -> Result<()> {
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use anchor_client::solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;
    #[test]//cargo test init_crypto_mail -- --show-output
    fn init_crypto_mail() {
        functions::init_crypto_mail::init_crypto_mail(
            &program::program().unwrap()
        )
        .unwrap();
    }
    #[test]//cargo test send_mail -- --show-output
    fn send_mail() {
        let receiver: Pubkey = Pubkey::from_str("4TL6nFzPXt29NcrmixgMyG7EWk9bY4kTpVJdg8bpmov4").unwrap();
        functions::send_mail::send_mail(
            &program::program().unwrap(),
            receiver,
            "Hello friend".to_string()
        )
        .unwrap();
    }
}