use std::str::FromStr;

use borsh::BorshDeserialize;
use solana_sdk::{
    signature::Signer,
    system_program,
    transaction::Transaction,
};
use crate::*;
use solana_attestation_service_client::{
    accounts::Attestation,
    instructions::CreateAttestationBuilder
};

pub fn process_create_attestation(args: &Args, sub_args: &CreateAttestationInput) -> Result<()> {

    let client = args.get_client();

    // user for frontend
    let frontend_payer = args.get_payer_keypair(); 

    // Issuer Admin for keeping on backend
    let authority = args.get_keypair(); 

    let recipient = Pubkey::from_str(&sub_args.recipient)?;

    // 程序 ID
    let program_id = args.program_id;

    let credential_name = &sub_args.credential_name; 

    let (credential_pda, _) = get_credential_pda(&program_id, &authority.pubkey(), credential_name);

    let schema_name =  &sub_args.schema_name; 
    let (schema_pda, _) = get_schema_pda(&program_id, &credential_pda, schema_name);

    let attestation_data = sub_args.attestation_data.clone();
    println!("Credential PDA: {}", credential_pda);
    println!("Schema PDA: {}", schema_pda);

    println!("request program_id: {}", program_id.to_string());
    println!("request authority: {}", authority.pubkey().to_string());

    println!("request credential_name: {}", credential_name);
    println!("request schema_name: {}", schema_name);
    
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
        .payer(frontend_payer.pubkey())
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

     // 创建部分签名的交易
    //  let mut transaction = Transaction::new_with_payer(
    //     &[create_attestation_ix],
    //     Some(&frontend_payer.pubkey()),  
    // );

    // // 后端签名者部分签名交易
    // transaction.partial_sign(&[&authority], recent_blockhash);

    // // 序列化交易以发送给前端
 
    // let serialized_transaction = bincode::serialize(&transaction)?;
    // println!("Partially signed transaction: {:?}", serialized_transaction);
    // let mut deserialized_tx: Transaction = bincode::deserialize(&serialized_transaction).expect("反序列化交易失败");
    // 注意：前端需要使用以下代码完成签名并提交交易
    // deserialized_tx.partial_sign(&[frontend_payer], recent_blockhash);

    let transaction = Transaction::new_signed_with_payer(
        &[create_attestation_ix],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction);
    println!("交易已确认: {:#?}", signature);

    println!("Done!");

    let attestation_account = client.get_account(&attestation_pda).expect("get_account");

    let attestation = Attestation::try_from_slice(&attestation_account.data).unwrap();
    assert_eq!(attestation.schema, schema_pda);
    assert_eq!(attestation.signer, authority.pubkey());
    assert_eq!(attestation.expiry, expiry);
    assert_eq!(attestation.nonce, recipient);

    Ok(())
}