#[macro_use]
extern crate serde_derive;

mod blockchain_address;
mod blockchain_info;
mod blockchain_status;
mod blockchain_transaction;

use {
    crate::blockchain_address::BlockchainAddress,
    crate::blockchain_status::BlockchainStatus,
    crate::blockchain_transaction::BlockchainTransaction,
    dotenv,
    std::{io, thread, time},
};

fn blochain_info_app(address: &str) {
    let blockchain_status: BlockchainStatus = blockchain_info::blockchain_status_request();
    println!(
        "\n\nQuerying {} - chain: {}\n\n",
        &blockchain_status.blockbook.coin, &blockchain_status.backend.chain
    );

    let blockchain_address: BlockchainAddress =
        blockchain_info::blockchain_address_request(address);
    println!(
        "\n\nAnalyzing transaction for Bitcoin address {}",
        blockchain_address.address
    );

    let sleep_time = time::Duration::from_millis(3 * 1000);
    thread::sleep(sleep_time);

    println!(
        "\nYou have a total of {} transactions",
        blockchain_address.txids.len()
    );

    println!("\nDo you want to query these transactions? (y/n)");

    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Error in reading");

    if command.trim().eq("y") {
        println!("\nWe will look up the following transactions:\n");
        println!("{:#?}", &blockchain_address.txids);

        let mut balance: i32 = 0;
        for tx_id in &blockchain_address.txids {
            let mut subtotal_vin: i32 = 0;
            let mut subtotal_vout: i32 = 0;

            let blockchain_transaction = blockchain_info::blockchain_transaction_request(&tx_id);

            let match_adress = String::from(address);
            for tx in &blockchain_transaction.vin {
                if tx.addresses.contains(&match_adress) {
                    subtotal_vin += tx.value.parse::<i32>().unwrap();
                }
            }

            for tx in &blockchain_transaction.vout {
                if tx.addresses.contains(&match_adress) {
                    subtotal_vout += tx.value.parse::<i32>().unwrap();
                }
            }

            balance += &subtotal_vout - &subtotal_vin;

            println!("-----------------------------------------------------");
            println!("TX ID:           {}", &blockchain_transaction.txid);
            println!("SATOSHIS IN:     {}", &subtotal_vout);
            println!("SATOSHIS OUT:    {}", &subtotal_vin);
            println!("BALANCE:         {}", &balance);
            println!("-----------------------------------------------------");
        }

        println!("CURRENT BALANCE:     {}", &balance);
        println!("         IN BTC:     {}\n\n", balance as f32 * 0.00000001);
    }
}

fn main() {
    let address = dotenv::var("WALLET").expect("Error in reading WALLET env");
    blochain_info_app(&address);
}
