use std::str::FromStr;

use borsh::BorshDeserialize;
use solana_attestation_service_client::{
    accounts::Attestation,
    instructions::CreateAttestationBuilder
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::Signer,
    system_program,
    transaction::Transaction,
};

use crate::*;

pub fn process_create_attestation(args: &Args, sub_args: &CreateAttestationArgs) -> Result<()> {

    let client = args.get_client();
    let payer = args.get_keypair(); 
    let authority = &payer;
    // let recipient = &sub_args.recipient;
    let recipient = Pubkey::from_str(&sub_args.recipient)?;
    // 程序 ID
    let program_id = args.get_program_id();

    let credential_name = &sub_args.credential_name; 

    let (credential_pda, _) = get_credential_pda(&program_id, &authority.pubkey(), credential_name);

    let schema_name =  &sub_args.schema_name; 
    let (schema_pda, _) = get_schema_pda(&program_id, &credential_pda, schema_name);

    let attestation_data = sub_args.attestation_data.clone();
    // 创建唯一的 nonce
    // let nonce = Pubkey::from_str("1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM").expect("Invalid nonce value"); // Pubkey::new_unique();
    println!("Credential PDA: {}", credential_pda);
    println!("Schema PDA: {}", schema_pda);
    // println!("Nonce: {}", nonce);
    
    let (attestation_pda, _) = get_attestation_pda(
        &program_id, 
        &credential_pda, 
        &schema_pda, 
        &authority.pubkey(), 
        &recipient
    );
    println!("Attestation PDA: {}", attestation_pda);

    // 设置过期时间 (当前时间 + 1 小时，以秒为单位)
    let expiry: i64 = 3600;

    let create_attestation_ix = CreateAttestationBuilder::new()
        .payer(payer.pubkey())
        .authority(authority.pubkey())
        .credential(credential_pda)
        .schema(schema_pda)
        .attestation(attestation_pda)
        .system_program(system_program::ID)
        .data(attestation_data.clone())
        .expiry(expiry)
        .nonce(recipient)
        .instruction();

     // 获取最新的区块哈希
     let recent_blockhash = client.get_latest_blockhash().unwrap();

     // 创建并发送交易
     let transaction = Transaction::new_signed_with_payer(
         &[create_attestation_ix],
         Some(&payer.pubkey()),
         &[&payer],
         recent_blockhash,
     );
    let signature = client.send_and_confirm_transaction(&transaction);
    println!("交易已确认: {:#?}", signature);

    println!("Done!");

    let attestation_account = client.get_account(&attestation_pda).expect("get_account");

    let attestation = Attestation::try_from_slice(&attestation_account.data).unwrap();
    // assert_eq!(attestation.data, serialized_attestation_data);
    // assert_eq!(attestation.credential, attestation_data);
    assert_eq!(attestation.schema, schema_pda);
    assert_eq!(attestation.signer, authority.pubkey());
    assert_eq!(attestation.expiry, expiry);
    assert_eq!(attestation.nonce, recipient);

    Ok(())
}
