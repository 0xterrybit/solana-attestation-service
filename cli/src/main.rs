use std::str::FromStr;

use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use clap::{Parser, Subcommand};
use solana_attestation_service_macros::SchemaStructSerialize;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair}
};
use solana_client::rpc_client::RpcClient;
use dotenv::dotenv;
pub mod instructions;
use instructions::*;

pub const CREDENTIAL_NAME: &str = "RNS_PROOF";
pub const SCHEMA_NAME: &str = "jurisdiction";


#[derive(BorshSerialize, SchemaStructSerialize)]
struct TestData {
    name: String,
    // location: u8,
}


#[derive(BorshSerialize, BorshDeserialize, SchemaStructSerialize, Debug)]
struct AgeOver18 {
    age_over18: bool,
}

#[derive(BorshSerialize, BorshDeserialize, SchemaStructSerialize, Debug)]
struct AgeOver21 {
    age_over21: bool,
}

#[derive(BorshSerialize, BorshDeserialize, SchemaStructSerialize, Debug)]
struct Gender {
    gender: bool,
}

#[derive(BorshSerialize, BorshDeserialize, SchemaStructSerialize, Debug)]
struct BirthYear {
    birth_year: u32,
}

#[derive(BorshSerialize, BorshDeserialize, SchemaStructSerialize, Debug)]
struct Jurisdiction {
    jurisdiction: String,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,

    /// RPC url
    #[clap(long, env, default_value_t = dotenv_default("RPC_URL"))]
    pub rpc_url: String,

    /// Program id
    #[clap(long, env, default_value_t = Pubkey::from_str_const(&dotenv_default("PROGRAM_ID")))]
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

    // 后端权威签名者
    fn get_keypair(&self) -> Keypair {
        let keypair_path = dotenv_default("KEYPAIR_PATH");
        let keypair = read_keypair_file(keypair_path).unwrap();
        keypair
    }

    // // 前端用户
    // fn get_payer_keypair(&self) -> Keypair {
    //     let keypair_path = dotenv_default("PAYER_KEYPAIR_PATH");
    //     let keypair = read_keypair_file(keypair_path).unwrap();
    //     keypair
    // }
    
}

// CreateCredential subcommand args
#[derive(Parser, Debug)]
pub struct CreateCredentialArgs {
    /// Specify the credential name to use, ex: RNS_PROOF
    #[clap(long, env, default_value = CREDENTIAL_NAME )]
    credential_name: String,
}

// CreateSchema subcommand args
#[derive(Parser, Debug)]
pub struct CreateSchemaArgs {

    /// Specify the credential name to use, ex: RNS_CREDENTIAL
    #[clap(long, default_value = CREDENTIAL_NAME )]
    credential_name: String,

    /// Specify the schema type to use, ex: jurisdiction, age_over18, age_over21, birth_year, gender
    #[clap(long)]
    schema_type: SchemaType,

    /// Specify the schema name to use, ex: jurisdiction_schema_name
    #[clap(long)]
    schema_name: String,

    /// Specify the schema name to use, ex: jurisdiction
    #[clap(long, default_value = "schema_description" )]
    description: String
}

// CreateAttestation subcommand args
#[derive(Parser, Debug)]
pub struct CreateAttestationArgs {
    
    /// Specify the credential name to use, ex: RNS_PROOFs
    #[clap(long, default_value = CREDENTIAL_NAME )]
    credential_name: String,

    /// Specify the schema name to use, ex: jurisdiction
    #[clap(long)]
    schema_name: String,
    
    /// Specify the recipient to use, ex: A6WcyjnyU4nBD66tKxzg35bYCkeNqF4MCtQr7pwreVAv
    #[clap(long)]
    recipient: String,

    /// Specify the schema type to use, ex: jurisdiction, age_over18, age_over21, birth_year, gender
    #[clap(long)]
    schema_type: SchemaType,
    
}


// CreateAttestation subcommand args
#[derive(Parser, Debug)]
pub struct CreateAttestationInput {
    
    /// Specify the credential name to use, ex: RNS_CREDENTIAL
    #[clap(long, default_value = CREDENTIAL_NAME )]
    credential_name: String,

    /// Specify the schema name to use, ex: jurisdiction
    #[clap(long)]
    schema_name: String,
    
    /// Specify the recipient to use, ex: A6WcyjnyU4nBD66tKxzg35bYCkeNqF4MCtQr7pwreVAv
    #[clap(long)]
    recipient: String,

    /// Specify the attestation_data to use
    #[clap(long)]
    attestation_data: Vec<u8>,

}

#[derive(Debug, Clone, PartialEq)]
pub enum QueryType {
    Credential,
    Schema,
    Attestation,
}

impl FromStr for QueryType {
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

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaType {
    AgeOver18,
    AgeOver21,
    Gender,
    Jurisdiction,
    BirthYear,
    TestData
}

impl FromStr for SchemaType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "age_over18" => Ok(SchemaType::AgeOver18),
            "age_over21" => Ok(SchemaType::AgeOver21),
            "gender" => Ok(SchemaType::Gender),
            "jurisdiction" => Ok(SchemaType::Jurisdiction),
            "birth_year" => Ok(SchemaType::BirthYear),
            "test_data" => Ok(SchemaType::TestData),
            _ => Err(format!("未知的Schema类型: {}", s)),
        }
    }
}

// Query subcommand args
#[derive(Parser, Debug)]
pub struct QueryArgs {
    /// Specify the credential name to use, ex: RNS_CREDENTIAL
    #[clap(long, default_value = CREDENTIAL_NAME )]
    credential_name: String,

    /// Specify the schema name to use, ex: jurisdiction
    #[clap(long, default_value = "jurisdiction" )]
    schema_name: String,

    /// Specify the query type to use, ex: attestation, schema, credential
    #[clap(long, default_value = "attestation")]
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

            let new_args = CreateSchemaArgs {
                credential_name: sub_args.credential_name.clone(),
                schema_type: sub_args.schema_type.clone(),
                schema_name: sub_args.schema_name.clone(),
                description: sub_args.description.clone(),
            };

            let _ = process_create_schema(&args, &new_args);
        }

        Commands::CreateAttestation(sub_args) => {
            
            let recipient = sub_args.recipient.clone(); 
            
            let mut serialized_attestation_data = Vec::new();
            let schema_type = &sub_args.schema_type;
            match schema_type {
                SchemaType::TestData => {
                    let data: TestData = TestData {
                        name: "true".to_string(),
                    };
                    data.serialize(&mut serialized_attestation_data).expect("序列化失败");
                },
                SchemaType::AgeOver18 => {
                    let data = AgeOver18 {
                        age_over18: true
                    };
                    data.serialize(&mut serialized_attestation_data).expect("序列化失败");
                },
                SchemaType::AgeOver21 => {
                    let data = AgeOver21 {
                        age_over21: true
                    };
                    data.serialize(&mut serialized_attestation_data).expect("序列化失败");
                },
                SchemaType::Jurisdiction => {
                    let data = Jurisdiction {
                        jurisdiction: "1".to_string(),
                    };
                    data.serialize(&mut serialized_attestation_data).expect("序列化失败");
                },
                SchemaType::Gender => {
                    let data = Gender {
                        gender: true
                    };
                    data.serialize(&mut serialized_attestation_data).expect("序列化失败");
                },
                SchemaType::BirthYear => {
                    let data = BirthYear{
                        birth_year: 2000
                    };
                    data.serialize(&mut serialized_attestation_data).expect("序列化失败");
                }
            }

            let new_args = CreateAttestationInput {
                credential_name: sub_args.credential_name.clone(),
                schema_name: sub_args.schema_name.clone(),
                attestation_data: serialized_attestation_data.clone(),
                recipient: recipient
            };

            let _ = process_create_attestation(&args, &new_args);

        }
 
        Commands::Query(sub_args) => {
            let _ = process_query(&args, sub_args);
        }
    }

    Ok(())
}