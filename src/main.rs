use std::str::FromStr as _;

use polymarket_client_sdk::auth::LocalSigner;
use polymarket_client_sdk::auth::Signer as _;
use polymarket_client_sdk::clob::types::SignatureType;
use polymarket_client_sdk::clob::types::{Amount, OrderType, Side};
use polymarket_client_sdk::clob::{Client, Config};
use polymarket_client_sdk::types::{Decimal, U256, address};
use polymarket_client_sdk::{POLYGON, PRIVATE_KEY_VAR};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let private_key = std::env::var(PRIVATE_KEY_VAR).expect("Need a private key");
    let signer = LocalSigner::from_str(&private_key)?.with_chain_id(Some(POLYGON));
    let client = Client::new("https://clob.polymarket.com", Config::default())?
        .authentication_builder(&signer)
        .funder(address!("0x4625051d9a9Aa6D47AdE911D2E1B0dcAAF035C56"))
        .signature_type(SignatureType::GnosisSafe)
        .authenticate()
        .await?;

    let token_id = U256::from_str(
        "78378267119031321423877181792898597296833392678745984374814945303053666759588",
    )?;
    let nn = Decimal::from_str("0.99")?;

    let order_p = client
        .limit_order()
        .token_id(token_id)
        .size(Decimal::TWO)
        .price(nn)
        .side(Side::Buy)
        .build()
        .await?;
    let signed_order_p = client.sign(&signer, order_p).await?;

    let _instant = std::time::Instant::now();
    let order = client
        .limit_order()
        .token_id(token_id)
        .size(Decimal::TWO)
        .price(nn)
        .side(Side::Buy)
        .build()
        .await?;
    let order_step = _instant.elapsed();
    let signed_order = client.sign(&signer, order).await?;
    let signed_step = _instant.elapsed();

    let response = client.post_order(signed_order).await?;
    let response_step = _instant.elapsed();
    println!(
        "Steps {:?} {:?} {:?}",
        order_step, signed_step, response_step
    );
    println!("Order response: {:?}", response);

    let _instant2 = std::time::Instant::now();
    let response2 = client.post_order(signed_order_p).await?;
    let response_step2 = _instant2.elapsed();
    println!("Only send  {:?}", response_step2);
    Ok(())
}
