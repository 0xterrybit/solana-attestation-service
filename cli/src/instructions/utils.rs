use solana_program::pubkey::Pubkey;
use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::commitment_config::CommitmentConfig;

use solana_account_decoder_client_types::{UiDataSliceConfig, UiAccountEncoding};
use solana_client::{
    rpc_config::{
        RpcProgramAccountsConfig,
        RpcAccountInfoConfig,
    },
    rpc_client::RpcClient,
    rpc_filter::{
        RpcFilterType,
        Memcmp,
    }
};

pub fn get_credential_pda(
    program_id: &Pubkey,
    authority: &Pubkey,
    credential_name: &str,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"credential",
            &authority.to_bytes(),
            credential_name.as_bytes(),
        ],
        &program_id,
    )
}

pub fn get_schema_pda(
    program_id: &Pubkey,
    credential_pda: &Pubkey,
    schema_name: &str,
) -> (Pubkey, u8) {

    Pubkey::find_program_address(
        &[
            b"schema",
            &credential_pda.to_bytes(),
            schema_name.as_bytes(),
            &[1], // 版本号
        ],
        &program_id,
    )
}

pub fn get_attestation_pda(
    program_id: &Pubkey,
    credential_pda: &Pubkey,
    schema_pda: &Pubkey,
    authority: &Pubkey,
    nonce: &Pubkey,
) -> (Pubkey, u8) {

    Pubkey::find_program_address(
        &[
            b"attestation",
            &credential_pda.to_bytes(),
            &authority.to_bytes(),
            &schema_pda.to_bytes(),
            &nonce.to_bytes(),
        ],
        &program_id,
    )
}


pub fn get_all_attestation_addresses(
    program_id: &Pubkey,
    client: &RpcClient,
    credential_pda: &Pubkey,
    schema_pda: &Pubkey,
) -> Result<Vec<Pubkey>> {
    let filters = vec![
        RpcFilterType::Memcmp(
            Memcmp::new_base58_encoded(
                33, // discriminator (1字节) + nonce (32字节) 后的位置，用于比较 credential
                &credential_pda.to_bytes(),
            ),
        ),
        RpcFilterType::Memcmp(
            Memcmp::new_base58_encoded(
                65, // discriminator (1字节) + nonce (32字节) + credential (32字节) 后的位置，用于比较 schema
                &schema_pda.to_bytes(),
            ),
        ),
    ];
    
    let config = RpcProgramAccountsConfig {
        filters: Some(filters),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            data_slice: Some(UiDataSliceConfig {
                offset: 0,
                length: 5,
            }),
            commitment: Some(CommitmentConfig::processed()),
            min_context_slot: None,
        },
        with_context: Some(false),
        sort_results: Some(true),
    };
    
    let accounts = client.get_program_accounts_with_config(&program_id, config)?;
    
    Ok(accounts.iter().map(|(pubkey, _)| *pubkey).collect())
}



pub fn get_all_credential_addresses(
    program_id: &Pubkey,
    client: &RpcClient,
) -> Result<Vec<Pubkey>> {
    let filters = vec![
        RpcFilterType::Memcmp(
            Memcmp::new_base58_encoded(
                0,      // 从第一个字节开始，即 discriminator
                &[0],   // credential 的 discriminator 值，
            ),
        ),
    ];
    
    let config = RpcProgramAccountsConfig {
        filters: Some(filters),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            data_slice: Some(UiDataSliceConfig {
                offset: 0,
                length: 5,
            }),
            commitment: Some(CommitmentConfig::processed()),
            min_context_slot: None,
        },
        with_context: Some(false),
        sort_results: Some(true),
    };
    
    let accounts = client.get_program_accounts_with_config(&program_id, config)?;
    
    Ok(accounts.iter().map(|(pubkey, _)| *pubkey).collect())
}


pub fn get_all_schema_addresses(
    program_id: &Pubkey,
    client: &RpcClient,
    credential_pda: &Pubkey,
    // schema_pda: &Pubkey,
) -> Result<Vec<Pubkey>> {
    let filters = vec![
        RpcFilterType::Memcmp(
            Memcmp::new_base58_encoded(
                0, // 从第一个字节开始，即 discriminator
                &[1], // Schema 的 discriminator 值，假设为 2，请根据实际情况调整
            ),
        ),
        RpcFilterType::Memcmp(
            Memcmp::new_base58_encoded(
                1, // discriminator 后的位置，Schema 结构中 credential 字段的偏移量
                &credential_pda.to_bytes(),
            ),
        ),
        // RpcFilterType::Memcmp(
        //     Memcmp::new_base58_encoded(
        //         65, // discriminator (1字节) + nonce (32字节) + credential (32字节) 后的位置，用于比较 schema
        //         &schema_pda.to_bytes(),
        //     ),
        // ),
    ];
    
    let config = RpcProgramAccountsConfig {
        filters: Some(filters),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            data_slice: Some(UiDataSliceConfig {
                offset: 0,
                length: 5,
            }),
            commitment: Some(CommitmentConfig::processed()),
            min_context_slot: None,
        },
        with_context: Some(false),
        sort_results: Some(true),
    };
    
    let accounts = client.get_program_accounts_with_config(&program_id, config)?;
    
    Ok(accounts.iter().map(|(pubkey, _)| *pubkey).collect())
}
