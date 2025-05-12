use std::str::FromStr;

use borsh::BorshDeserialize;
use solana_sdk::{
    signature::Signer,
    system_program,
    transaction::Transaction,
};
use crate::*;
use solana_attestation_service_client::{
    accounts::Request,
    instructions::CreateRequestBuilder,
    
};

pub fn process_create_request(args: &Args, sub_args: &CreateRequestInput) -> Result<()> {

    let client = args.get_client();

    // user for frontend
    let frontend_payer = args.get_payer_keypair(); 
    let recipient = frontend_payer.pubkey();

    // Issuer Admin for keeping on backend
    let authority = args.get_keypair(); 


    // 程序 ID
    let program_id = args.program_id;

    let credential_name = &sub_args.credential_name; 

    let (credential_pda, _) = get_credential_pda(&program_id, &authority.pubkey(), credential_name);

    let schema_name =  &sub_args.schema_name; 
    let (schema_pda, _) = get_schema_pda(&program_id, &credential_pda, schema_name);

    let attestation_data = sub_args.attestation_data.clone();
     
    

    // 设置过期时间 (当前时间 + 1000_000 小时，以秒为单位)
    // let expiry: i64 = 3600_000_000;

    let create_attestation_ix = CreateRequestBuilder::new()
        .payer(frontend_payer.pubkey())
        .schema(schema_pda)
        .data(attestation_data.clone())
        .nonce(recipient)
        .system_program(system_program::ID)
        .instruction();

    // 获取最新的区块哈希
    let recent_blockhash = client.get_latest_blockhash().unwrap();
 
    let transaction = Transaction::new_signed_with_payer(
        &[create_attestation_ix],
        Some(&frontend_payer.pubkey()),
        &[&frontend_payer],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction);
    println!("交易已确认: {:#?}", signature);

    println!("Done!");

    let (request_pda, _) = get_request_pda(
        &program_id,            // 
        &credential_pda,        // attest
        &schema_pda,            // attest
        &authority.pubkey(),    // rns owner
        &recipient
    );

    let attestation_account = client.get_account(&request_pda).expect("get_account");
    
    let attestation = Request::try_from_slice(&attestation_account.data).unwrap();
    assert_eq!(attestation.schema, schema_pda);
    assert_eq!(attestation.nonce, recipient);

    Ok(())
}

