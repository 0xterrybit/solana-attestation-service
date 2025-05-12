use borsh::{BorshDeserialize, BorshSerialize};
use helpers::program_test_context;
use solana_attestation_service_client::{
    accounts::Request,
    instructions::{
        CreateCredentialBuilder, 
        CreateRequestBuilder, 
        CreateSchemaBuilder
    },
};

use solana_attestation_service_macros::SchemaStructSerialize;
use solana_program_test::ProgramTestContext;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    transaction::Transaction,
};

mod helpers;


#[derive(BorshSerialize, BorshDeserialize, SchemaStructSerialize, Debug)]
// #[derive(BorshSerialize, SchemaStructSerialize)]
struct TestData {
    name: String,
    location: u8,
}

struct TestFixtures {
    ctx: ProgramTestContext,
    credential: Pubkey,
    schema: Pubkey,
    authority: Keypair
}

async fn setup() -> TestFixtures {
    let mut ctx = program_test_context().await;

    let authority = Keypair::new();
    let credential_name = "test";
    let (credential_pda, _bump) = Pubkey::find_program_address(
        &[
            b"credential",
            &authority.pubkey().to_bytes(),
            credential_name.as_bytes(),
        ],
        &Pubkey::from(solana_attestation_service_client::programs::SOLANA_ATTESTATION_SERVICE_ID),
    );

    let create_credential_ix = CreateCredentialBuilder::new()
        .payer(ctx.payer.pubkey())
        .credential(credential_pda)
        .authority(authority.pubkey())
        .system_program(system_program::ID)
        .name(credential_name.to_string())
        .signers(vec![authority.pubkey()])
        .instruction();

    // Create Schema
    let schema_name = "test_data";
    let description = "schema for test data";
    let schema_data = TestData::get_serialized_representation();
    let field_names = vec!["name".into(), "location".into()];
    let (schema_pda, _bump) = Pubkey::find_program_address(
        &[
            b"schema",
            &credential_pda.to_bytes(),
            schema_name.as_bytes(),
            &[1],
        ],
        &Pubkey::from(solana_attestation_service_client::programs::SOLANA_ATTESTATION_SERVICE_ID),
    );
    let create_schema_ix = CreateSchemaBuilder::new()
        .payer(ctx.payer.pubkey())
        .authority(authority.pubkey())
        .credential(credential_pda)
        .schema(schema_pda)
        .system_program(system_program::ID)
        .description(description.to_string())
        .name(schema_name.to_string())
        .layout(schema_data.clone())
        .field_names(field_names)
        .instruction();

    let transaction = Transaction::new_signed_with_payer(
        &[create_credential_ix, create_schema_ix],
        Some(&ctx.payer.pubkey()),
        &[
            &ctx.payer, 
            &authority
        ],
        ctx.last_blockhash,
    );
    ctx.banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    TestFixtures {
        ctx,
        credential: credential_pda,
        schema: schema_pda,
        authority: authority,
    }
}

#[tokio::test]
async fn create_request_success() {

    let TestFixtures {
        ctx,
        credential,
        schema,
        authority,
    } = setup().await;

    let req_data = TestData {
        name: "attest".to_string(),
        location: 11,
    };

    // let frontend_payer = authority;
    // let recipient = authority.pubkey();
    let recipient = Pubkey::new_unique();

    let expiry: i64 = 1000_000_0000;
    let mut serialized_req_data = Vec::new();
    req_data
        .serialize(&mut serialized_req_data)
        .unwrap();


    let request_pda = Pubkey::find_program_address(
        &[
            b"request",                     // 
            &credential.to_bytes(),         // 
            &schema.to_bytes(),             // 
            &recipient.to_bytes(),          // 
        ],
        &solana_attestation_service_client::programs::SOLANA_ATTESTATION_SERVICE_ID,
    )
    .0;

    let create_req_ix = CreateRequestBuilder::new()
        .payer(ctx.payer.pubkey())
        .authority(authority.pubkey())
        .credential(credential)
        .schema(schema)
        .request(request_pda)
        .system_program(system_program::ID)
        .data(serialized_req_data.clone())
        .expiry(expiry)
        .nonce(recipient)
        .instruction();

    let transaction: Transaction = Transaction::new_signed_with_payer(
        &[create_req_ix],
        Some(&ctx.payer.pubkey()),
        &[
            &ctx.payer, 
            // &authority
        ],
        ctx.last_blockhash,
    );

    ctx.banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    let request_account = ctx
        .banks_client
        .get_account(request_pda)
        .await
        .unwrap()
        .unwrap();

    let req = Request::try_from_slice(&request_account.data).unwrap();

    println!("credential: {:?}", req.credential.to_string());
    println!("schema: {:?}", req.schema.to_string());
    println!("signer: {:?}", req.signer.to_string());
    println!("recipient: {:?}", req.nonce.to_string());
    println!("expiry: {:?}", req.expiry);
    println!("data: {:?}", req.data);
    
    let deserialized_data = TestData::try_from_slice(&req.data).unwrap();

    println!("deserialized_data: {:?}", deserialized_data);
 

}
