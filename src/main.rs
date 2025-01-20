use std::str::FromStr;
use solana_client::{client_error::ClientError, rpc_client::RpcClient};
use solana_account::Account;
use solana_sdk::{message::Message, signature::Signature, system_instruction};
use solana_sdk::signer::keypair;
use solana_sdk::pubkey::Pubkey;
//use solana_sdk::transaction::VersionedTransaction;
//use solana_sdk::message::VersionedMessage;
use solana_sdk::transaction::Transaction;
use solana_program::native_token::LAMPORTS_PER_SOL;

const FROM_ADDRESS: &str = "C87PKotnrdj3ykFeJoM65hR4GBvrdTYfeXiSuB5xSKHm";
const TO_ADDRESS: &str = "4hXSFQ3AiiaBDS2T2ArgDAZnaAzaY5KyVjN59DAnmpUh";
const DEV_NET: &str = "https://api.devnet.solana.com";
//const MAIN_NET: &str = "https://api.mainnet-beta.solana.com";



fn get_client_info(account_address: &str) -> Result<Account, ClientError> {
    // connect to RPC node
    let client = RpcClient::new(DEV_NET);
    let pubkey = Pubkey::from_str(account_address).unwrap();
    /*
    pub struct Account {
        pub lamports: u64,
        pub data: Vec<u8>,
        pub owner: Pubkey,
        pub executable: bool,
        pub rent_epoch: u64,
    }
    */
    return client.get_account(&pubkey);
}

// differences of several data structures
fn get_from_keypair() {
    println!("from address: {}", FROM_ADDRESS);
    let pubkey = Pubkey::from_str(FROM_ADDRESS);
    println!("from Pubkey {:?}", pubkey);
    let from_keypair = keypair::read_keypair_file("/home/james/.config/solana/id.json").expect("Failed to read keypair file");
    println!("from keypair: {:?}", from_keypair);
}

fn transfer(from_address: &str, to_address: &str, amount: f32) -> Result<Signature, ClientError>{
    let client = RpcClient::new(DEV_NET);

    let from_keypair = keypair::read_keypair_file("/home/james/.config/solana/id.json").expect("Failed to read keypair file");
    println!("from keypair: {:?}", from_keypair);

    //let id_array = five8_const::decode_32_const(from_address);
    //let from_pubkey = Pubkey::new_from_array(id_array);

    let from_pubkey = Pubkey::from_str(from_address).unwrap();

    //let id_array = five8_const::decode_32_const(to_address);
    //let to_pubkey = Pubkey::new_from_array(id_array);

    let to_pubkey = Pubkey::from_str(to_address).unwrap();

    // transfer instruction
    let lanports_per_sol = LAMPORTS_PER_SOL as f32;
    let transfer_instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, (amount * lanports_per_sol).round() as u64);

    // transfer message
    let message = Message::new(&[transfer_instruction], Some(&from_pubkey));
    // for VersionedTransaction in the future
    //let versioned_message = VersionedMessage::Legacy(message);

    // create transaction
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    //#[cfg(debug_assertions)]
    println!("latest_blockhash: {}", recent_blockhash);
    let transaction = Transaction::new(&[&from_keypair], message, recent_blockhash);

    // use VersionedTransaction in the future
    //let versioned_transaction = VersionedTransaction::try_new(versioned_message, &[&from_keypair]).unwrap();

    // send transaction
    // VersionedTransaction does not work this way for:
    // RPC response error -32002: Transaction simulation failed: Blockhash not found
    return client.send_and_confirm_transaction(&transaction);
}


fn main() {
    #[cfg(debug_assertions)]
    get_from_keypair();

    let from_account: Account;
    match get_client_info(FROM_ADDRESS) {
        Ok(account) => { 
            #[cfg(debug_assertions)]
            println!("Account connected!");
            from_account = account;
        }
        Err(err) => {
            #[cfg(debug_assertions)]
            println!("Account connecting fail! {}", err);
            return;
        }
    }
    #[cfg(debug_assertions)]
    println!("FromAccount Info: {:?}", from_account);

    let to_account: Account;
    match get_client_info(TO_ADDRESS) {
        Ok(account) => { 
            #[cfg(debug_assertions)]
            println!("Account connected!");
            to_account = account;
        }
        Err(err) => {
            #[cfg(debug_assertions)]
            println!("Account connecting fail! {}", err);
            return;
        }
    }
    #[cfg(debug_assertions)]
    println!("ToAccount Info: {:?}", to_account);

    let ret = transfer(FROM_ADDRESS, TO_ADDRESS, 0.01);
    match ret {
        Ok(signature) => {
            println!("Transaction sent: {}", signature);
        }
        Err(err) => {
            println!("Error sending transaction: {}", err);
        }
    }
    
}


