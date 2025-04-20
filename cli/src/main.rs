use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use clap::{Parser, Subcommand};
use solana_attestation_service_macros::SchemaStructSerialize;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair}, signer::Signer
};
use solana_client::rpc_client::RpcClient;
use solana_attestation_service_client::programs::SOLANA_ATTESTATION_SERVICE_ID;
use dotenv::dotenv;
pub mod instructions;
use instructions::*;


pub const CREDENTIAL_NAME: &str = "rns_credential_1";
pub const SCHEMA_NAME: &str = "jurisdiction_3";
// pub const SCHEMA_VERSION: u32 = 1;
// pub const CREDENTIAL_VERSION: u32 = 1;
// pub const ATTESSTATION_NAME: &str = "jurisdiction_2";

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct AgeOver18Data {
    age_over18: bool,
    recipient: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct AgeOver21Data {
    age_over21: bool,
    recipient: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct GenderData {
    gender: bool,
    recipient: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct BirthYear {
    birth_year: u32,
    recipient: Pubkey,
}


// #[derive(BorshSerialize, SchemaStructSerialize)]
// struct TestData {
//     name: String,
//     location: u8,
// }

#[derive(BorshSerialize, BorshDeserialize, SchemaStructSerialize, Debug)]
struct Jurisdiction {
    recipient: String,
    jurisdiction: String,
}

// AgeOver18:bool
// AgeOver21:bool
// Gender:M/F(男/女)
// Birth Year: 1980--2010
// Jurisdiction: String, China/Palau/USA

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
    
    // /// Payer keypair
    // #[clap(long, env, default_value_t = dotenv_default("KEYPAIR_PATH"))]
    // pub keypair_path: String,

    /// RPC url
    #[clap(long, env, default_value_t = dotenv_default("RPC_URL"))]
    pub rpc_url: String,

    /// Program id
    #[clap(long, env, default_value_t = Pubkey::from(SOLANA_ATTESTATION_SERVICE_ID))]
    pub program_id: Pubkey,

    /// Priority fee
    #[clap(long, env)]
    pub priority_fee: Option<u64>,
}

// 添加一个辅助函数来获取环境变量，如果不存在则使用默认值
fn dotenv_default(key: &str) -> String {
    std::env::var(key).expect(&format!("Environment variable not found: {}", key))
}

impl Args {

    fn get_client(&self) -> RpcClient {
        println!("rpc_url: {}", self.rpc_url); 
        let commitment = CommitmentConfig::confirmed();
        RpcClient::new_with_commitment(self.rpc_url.clone(), commitment)
    }

    fn get_keypair(&self) -> Keypair {
        let keypair_path = dotenv_default("KEYPAIR_PATH");
        let keypair = read_keypair_file(keypair_path).unwrap();
        keypair
    }

    fn get_program_id(&self) -> Pubkey {
        let program_id = Pubkey::from(SOLANA_ATTESTATION_SERVICE_ID);
        program_id
    }
}

// CreateCredential subcommand args
#[derive(Parser, Debug)]
pub struct CreateCredentialArgs {
    #[clap(long, env, default_value = CREDENTIAL_NAME )]
    credential_name: String,
}

// CreateSchema subcommand args
#[derive(Parser, Debug)]
pub struct CreateSchemaArgs {

    #[clap(long, env, default_value = CREDENTIAL_NAME )]
    credential_name: String,

    #[clap(long, env, default_value = SCHEMA_NAME )]
    schema_name: String,

    #[clap(long, env, default_value = "description" )]
    description: String,
    #[clap(long, env)]
    schema_layout: Vec<u8>,
    #[clap(long, env)]
    field_names: Vec<String>,
}

// CreateAttestation subcommand args
#[derive(Parser, Debug)]
pub struct CreateAttestationArgs {

    #[clap(long, env, default_value = CREDENTIAL_NAME )]
    credential_name: String,

    #[clap(long, env, default_value = SCHEMA_NAME )]
    schema_name: String,
    
    #[clap(long, env)]
    attestation_data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QueryType {
    Credential,
    Schema,
    Attestation,
}

impl std::str::FromStr for QueryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "credential" => Ok(QueryType::Credential),
            "schema" => Ok(QueryType::Schema),
            "attestation" => Ok(QueryType::Attestation),
            _ => Err(format!("未知的查询类型: {}", s)),
        }
    }
}

// Query subcommand args
#[derive(Parser, Debug)]
pub struct QueryArgs {
    #[clap(long, env, default_value = CREDENTIAL_NAME )]
    credential_name: String,

    #[clap(long, env, default_value = SCHEMA_NAME )]
    schema_name: String,

    #[clap(long, env, default_value = "attestation")]
    query_type: QueryType,
}

// Subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    CreateCredential(CreateCredentialArgs),
    CreateSchema(CreateSchemaArgs),
    CreateAttestation(CreateAttestationArgs),
    Query(QueryArgs),
}

#[tokio::main] 
async fn main() -> Result<()> {
    dotenv().ok();

    let args = Args::parse();

    match &args.command {
        Commands::CreateCredential(sub_args) => {
            let _ = process_create_credential(&args, sub_args);
        }
        Commands::CreateSchema(sub_args) => {

            let schema_layout = Jurisdiction::get_serialized_representation();
            println!("schema_layout: {:?}", schema_layout);
            let field_names = vec!["recipient".into(), "jurisdiction".into()];

            let new_args = CreateSchemaArgs {
                schema_layout,
                field_names,
                credential_name: sub_args.credential_name.clone(),
                schema_name: sub_args.schema_name.clone(),
                description: sub_args.description.clone()
            };
            let _ = process_create_schema(&args, &new_args);
        }
        Commands::CreateAttestation(sub_args) => {
            
            println!("recipient:  {:?}", args.get_keypair().pubkey().to_string());
             
            let data = Jurisdiction {
                jurisdiction: "china".to_string(),
                recipient: "49TFFiQk2m9yizbfB4hxdGfFXQGkhoaRWAadXEbJ5wvN".to_string()//args.get_keypair().pubkey().to_string()
            };
        
            let mut attestation_data = Vec::new();
            data
                .serialize(&mut attestation_data)
                .unwrap();
            println!("attestation_data: {:?}", attestation_data);
 
            let new_args = CreateAttestationArgs {
                credential_name: sub_args.credential_name.clone(),
                schema_name: sub_args.schema_name.clone(),
                attestation_data: attestation_data.clone(),
            };

            let _ = process_create_attestation(&args, &new_args);
        }
        Commands::Query(sub_args) => {
            let _ = process_query(&args, sub_args);
        }
    }

    Ok(())
}
