use anyhow::Result;

use solana_attestation_service_client::accounts::{
    fetch_all_attestation, 
    fetch_all_credential, 
    fetch_all_schema, 
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::Signer
};
use solana_client::rpc_client::RpcClient;
use crate::*;

pub fn process_query(args: &Args, sub_args: &QueryArgs) -> Result<()> {

    let client = args.get_client();
    let program_id = args.program_id;
    
    // 获取凭证和模式的 PDA
    let credential_name = &sub_args.credential_name;
    let schema_name = &sub_args.schema_name;
    let query_type = &sub_args.query_type;

    // 获取凭证 PDA
    let authority = args.get_keypair().pubkey();
    let (credential_pda, _) = get_credential_pda(&program_id, &authority, credential_name);
    let (schema_pda, _) = get_schema_pda(&program_id, &credential_pda, schema_name);

    println!("Credential PDA: {}", credential_pda);
    println!("Schema PDA: {}", schema_pda);
 
    match query_type {
        QueryType::Credential => {
            let _ = get_all_credential_list(&program_id, &client);
        },
        QueryType::Schema => {
            let _ = get_all_schema_list(&program_id, &client, &credential_pda);
        },
        QueryType::Attestation => {
            let _ = get_all_attestation_list(&program_id, &client, &credential_pda, &schema_pda);
        },
    }
    Ok(())

}

fn get_all_schema_list(
    program_id: &Pubkey,
    client: &RpcClient,
    credential_pda: &Pubkey,
) -> Result<()> {

    let schema_addresses = get_all_schema_addresses(&program_id, &client, &credential_pda)?;
    let schema_accounts = fetch_all_schema(&client, &schema_addresses)?;

    println!("schemas lenth: {:?}", schema_addresses.len());

    for (i, account) in schema_accounts.iter().enumerate() {
        println!("schema {:?} name: {:?}", i, String::from_utf8(account.data.name.clone())? );
        println!("schema {:?} description: {:?}", i, String::from_utf8(account.data.description.clone())? );
        println!("schema {:?} pda address: {:?}", i, account.address.to_string());
        println!("schema {:?} credential : {:?}", i, account.data.credential.to_string());
        
        let field_names = account.data.field_names.clone();
        let layout = account.data.layout.clone();
        let field_count = layout.len() as u32; 

        // 序列化字段名称
        let mut complete_field_names_data = Vec::new();
        complete_field_names_data.extend_from_slice(&field_count.to_le_bytes());    // 添加长度前缀
        complete_field_names_data.extend_from_slice(&field_names);                  // 添加实际数据
        
        let deserialized_field_names: Vec<String> = borsh::BorshDeserialize::try_from_slice(&complete_field_names_data)
            .expect("反序列化字段名称失败");
        
        println!("schema {:?} layout: {:?}", i, layout);
        println!("schema {:?} field_count: {:?}", i, field_count);
        println!("schema {:?} field_names: {:?}", i, deserialized_field_names);
        println!("schema {:?} version: {:?}", i, account.data.version.to_string());
        println!("schema {:?} is_paused: {:?}", i, account.data.is_paused.to_string());
        // println!("schema {:?} {:?}", i, account.data);

    }
    Ok(())
}

fn get_all_attestation_list(
    program_id: &Pubkey,
    client: &RpcClient,
    credential_pda: &Pubkey,
    schema_pda: &Pubkey,
) -> Result<()> {
    
    let attestations = get_all_attestation_addresses(&program_id, &client, &credential_pda, &schema_pda)?;
    println!("attestations lenth: {}", &attestations.len());

    let attestation_accounts = fetch_all_attestation(&client, &attestations)?;
    for (i, attestation_account) in attestation_accounts.iter().enumerate() {

        let deserialized_data = Jurisdiction::try_from_slice(&attestation_account.data.data).unwrap();
        println!("attestation {:?} deserialized_data: {:?}", i, deserialized_data);

        println!("attestation {:?} pda address: {:?}", i, attestation_account.address.to_string());
        println!("attestation {:?} credential : {:?}", i, attestation_account.data.credential.to_string());
        println!("attestation {:?} nonce: {:?}", i, attestation_account.data.nonce.to_string());
        println!("attestation {:?} schema: {:?}", i, attestation_account.data.schema.to_string());
        println!("attestation {:?} expiry: {:?}", i, attestation_account.data.expiry.to_string());

        // println!("attestation {:?} data: {:?}", i, attestation_account.data);
    }

    Ok(())
}

fn get_all_credential_list(
    program_id: &Pubkey, 
    client: &RpcClient
) -> Result<()> {

    let credentials = get_all_credential_addresses(&program_id, &client)?;
    let credential_accounts = fetch_all_credential(&client, &credentials)?;

    for (i, credential_account) in credential_accounts.iter().enumerate() {
        println!("credential {:?} authority: {:?}", i, credential_account.data.authority.to_string());
        // println!("credential {:?} Data: {:?}", i, credential_account.data);
        println!("credential {:?} name: {:?}", i, String::from_utf8(credential_account.data.name.clone())? );
    }
    Ok(())
}