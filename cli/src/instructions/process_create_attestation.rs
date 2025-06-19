use std::str::FromStr;

use borsh::BorshDeserialize;
use solana_sdk::{
    signature::Signer,
    system_program,
    clock::Clock,
    transaction::Transaction,
    sysvar
};
use crate::*;
use solana_attestation_service_client::{
    accounts::Attestation,
    instructions::CreateAttestationBuilder
};

pub fn process_create_attestation(args: &Args, sub_args: &CreateAttestationInput) -> Result<()> {

    let client = args.get_client();

    // user for frontend
    // let frontend_payer = args.get_payer_keypair(); 

    // Issuer Admin for keeping on backend
    let authority = args.get_keypair(); 

    let recipient = Pubkey::from_str(&sub_args.recipient)?;

    // 程序 ID
    let program_id = args.program_id;

    let credential_name = &sub_args.credential_name; 

    let (credential_pda, _) = get_credential_pda(&program_id, &authority.pubkey(), credential_name);

    let schema_name =  &sub_args.schema_name; 
    let (schema_pda, _bump) = Pubkey::find_program_address(
        &[
            b"schema",
            &credential_pda.to_bytes(),
            schema_name.as_bytes(),
            &[1],
        ],
        &solana_attestation_service_client::programs::SOLANA_ATTESTATION_SERVICE_ID,
    );

    let attestation_data = sub_args.attestation_data.clone();

    println!("Credential PDA: {}", credential_pda);
    println!("Schema PDA: {}", schema_pda);

    println!("request program_id: {}", program_id.to_string());
    println!("request authority: {}", authority.pubkey().to_string());

    println!("request credential_name: {}", credential_name);
    println!("request schema_name: {}", schema_name);
    
    let attestation_pda = Pubkey::find_program_address(
        &[
            b"attestation",
            &credential_pda.to_bytes(),
            &schema_pda.to_bytes(),
            &recipient.to_bytes(),
        ],
        &solana_attestation_service_client::programs::SOLANA_ATTESTATION_SERVICE_ID,
    ).0;
    
    println!("Attestation PDA: {}", attestation_pda);

    let clock_account = client.get_account(&sysvar::clock::id())?;

    let clock: Clock = bincode::deserialize(&clock_account.data).unwrap();

    let expiry: i64 = clock.unix_timestamp + 36000;

    let create_attestation_ix = CreateAttestationBuilder::new()
        .payer(authority.pubkey())
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
 
    let transaction = Transaction::new_signed_with_payer(
        &[create_attestation_ix],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction);
    println!("交易已确认: {:#?}", signature);

    // println!("Done!");
    // let attestation_account = client.get_account(&attestation_pda).expect("get_account");
    // let attestation = Attestation::try_from_slice(&attestation_account.data).unwrap();
    // assert_eq!(attestation.schema, schema_pda);
    // assert_eq!(attestation.signer, authority.pubkey());
    // assert_eq!(attestation.expiry, expiry);
    // assert_eq!(attestation.nonce, recipient);

    Ok(())
}