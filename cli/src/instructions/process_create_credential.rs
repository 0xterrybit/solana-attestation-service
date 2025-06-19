use borsh::BorshDeserialize;
use solana_attestation_service_client::{
    accounts::Credential, 
    instructions::CreateCredentialBuilder
};
use solana_sdk::{
    signature::Signer,
    system_program,
    transaction::Transaction,
};

use crate::*;

pub fn process_create_credential(args: &Args, sub_args: &CreateCredentialArgs) {

    let client = args.get_client();
    let payer = args.get_keypair(); 
    let authority = &payer;
    let program_id = args.program_id;

    // 1. 创建 Credential
    let credential_name =  &sub_args.credential_name; 

    let (credential_pda, _) = get_credential_pda(&program_id, &authority.pubkey(), credential_name);
    println!("Credential PDA: {}", credential_pda);

    let create_credential_ix = CreateCredentialBuilder::new()
        .payer(payer.pubkey())
        .credential(credential_pda)
        .authority(authority.pubkey())
        .system_program(system_program::ID)
        .name(credential_name.to_string())
        .signers(vec![authority.pubkey()])
        .instruction();
    
    // 获取最新的区块哈希
    let recent_blockhash = client.get_latest_blockhash().unwrap();

    // 创建并发送交易
    let transaction = Transaction::new_signed_with_payer(
        &[create_credential_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    println!("发送创建 Credential 和 Schema 的交易...");
    let signature = client.send_and_confirm_transaction(&transaction);
    println!("交易已确认: {:#?}", signature);

    println!("Done!");
    println!("Credential: {}", credential_pda);


    let credential_account = client.get_account(&credential_pda).expect("get_account");
    let credential = Credential::try_from_slice(&credential_account.data).unwrap();
    assert_eq!(credential.authority, authority.pubkey());
    assert_eq!(credential.name, credential_name.as_bytes());
    assert_eq!(credential.authorized_signers[0], authority.pubkey());
}