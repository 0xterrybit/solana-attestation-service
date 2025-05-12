use solana_attestation_service_client::instructions::CreateSchemaBuilder;
use solana_sdk::{
    signature::Signer,
    system_program,
    transaction::Transaction,
};

use crate::*;

pub fn process_create_schema(args: &Args, sub_args: &CreateSchemaArgs) {

    let client = args.get_client();
    let payer = args.get_keypair(); 
    let authority = &payer;

    // 程序 ID
    let program_id = args.program_id;
    
    let credential_name = &sub_args.credential_name; 
    let (credential_pda, _) = get_credential_pda(&program_id, &authority.pubkey(), credential_name);


    let schema_layout;
    let field_names;
    let schema_type = &sub_args.schema_type;
    match schema_type {
        SchemaType::AgeOver18 => {
            schema_layout = AgeOver18::get_serialized_representation();
            field_names = vec!["age_over18".into()];
        },
        SchemaType::AgeOver21 => {
            schema_layout = AgeOver21::get_serialized_representation();
            field_names = vec!["age_over21".into()];
        },
        SchemaType::Jurisdiction => {
            schema_layout = Jurisdiction::get_serialized_representation();
            field_names = vec!["jurisdiction".into()];
        },
        SchemaType::Gender => {
            schema_layout = Gender::get_serialized_representation();
            field_names = vec!["gender".into()];
        },
        SchemaType::BirthYear => {
            schema_layout = BirthYear::get_serialized_representation();
            field_names = vec!["birth_year".into()];
        }
    }

    let schema_name =  &sub_args.schema_name; 
    let description =  &sub_args.description; 

    let (schema_pda, _) = get_schema_pda(&program_id, &credential_pda, schema_name);

    // 3. 创建 Schema 
    let create_schema_ix = CreateSchemaBuilder::new()
        .system_program(system_program::ID)
        .payer(payer.pubkey())
        .authority(authority.pubkey())
        .credential(credential_pda)
        .schema(schema_pda)
        .name(schema_name.to_string())
        .description(description.to_string())
        .layout(schema_layout)
        .field_names(field_names)
        .instruction();

    
    // 获取最新的区块哈希
    let recent_blockhash = client.get_latest_blockhash().unwrap();

    // 创建并发送交易
    let transaction = Transaction::new_signed_with_payer(
        &[create_schema_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    println!("发送创建 Schema 的交易...");
    let signature = client.send_and_confirm_transaction(&transaction);
    println!("交易已确认: {:#?}", signature);

    println!("Done!");
    println!("Credential: {}", credential_pda);
    println!("Schema: {}", schema_pda);

}
