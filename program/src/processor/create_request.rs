use core::marker::PhantomData;

use pinocchio::{
    account_info::AccountInfo,
    instruction::Seed,
    program_error::ProgramError,
    pubkey::{
        find_program_address,
        Pubkey,
    },
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::Transfer;
use crate::{
    acc_info_as_str, 
    constants::REQUEST_SEED, 
    error::AttestationServiceError, 
    state::{
        discriminator::AccountSerialize, 
        Request, 
        Credential, 
        Schema
    } 
};

use super::{
    create_pda_account, 
    verify_owner_mutability, 
    verify_signer, 
    verify_system_program
};

#[inline(always)]
pub fn process_create_request(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    

    let [
        payer_info, 
        authority_info,         // inssuer authority_info
        credential_info,        // credential_pda
        schema_info,            // schema_pda
        request_info,           // request_pda
        system_program
    ] = accounts
    else {
        return Err(AttestationServiceError::InvalidRequestError.into());
    };
    
    // let amount = 1_000_000;
    // Transfer {
    //     from: payer_info,
    //     to: authority_info,
    //     lamports: amount,
    // }
    // .invoke()?;

    // // Validate system program
    // verify_system_program(system_program)?;

    // verify_owner_mutability(credential_info, program_id, false)?;
    // verify_owner_mutability(schema_info, program_id, false)?;

    // let credential_data = credential_info.try_borrow_data()?;
    // let credential = Credential::try_from_bytes(&credential_data)?;
    // Validate Credential PDA
    // credential.verify_pda(credential_info, program_id)?;

    // let schema_data = schema_info.try_borrow_data()?;
    // let schema = Schema::try_from_bytes(&schema_data)?;

    // Validate Schema PDA
    // schema.verify_pda(schema_info, program_id)?;

    // Validate Schema is owned by Credential
    // if schema.credential.ne(credential_info.key()) {
    //     return Err(AttestationServiceError::InvalidCredential.into());
    // }
    log!("process_create_request enter CreateRequestArgs");

    let args = CreateRequestArgs::try_from_bytes(instruction_data)?;
    let data: &[u8] = args.data()?;
    let nonce = args.nonce()?;
    let expiry = args.expiry()?;

    
    let (request_pda, reuest_bump) = find_program_address(
        &[
            REQUEST_SEED,
            credential_info.key(),
            schema_info.key(),
            nonce,
        ],
        &program_id
    );

    // Validate request PDA is correct
    if request_info.key().ne(&request_pda) {
        return Err(AttestationServiceError::InvalidRequestData.into());
    }
    log!("process_create_request enter CreateRequestArgs222");
    

    // Create Attestation account
    // Account layout
    // discriminator - 1
    // nonce - 32
    // Credential - 32
    // Schema - 32
    // data - 4 + len
    // signer - 32
    // expiry - 8
    let space = 1 + 32 + 32 + 32 + (4 + data.len()) + 32 + 8;

    let bump_seed = [reuest_bump];
    let signer_seeds = [
        Seed::from(REQUEST_SEED),
        Seed::from(credential_info.key()),
        Seed::from(schema_info.key()),
        Seed::from(nonce),
        Seed::from(&bump_seed),
    ];

    log!("process_create_request enter CreateRequestArgs3333");
    let rent = Rent::get()?;

    log!("process_create_request enter CreateRequestArgs44");
    create_pda_account(
        payer_info,
        &rent,
        space,
        program_id,
        request_info,
        signer_seeds,
        None,
    )?;

    log!("process_create_request enter CreateRequestArgs55");
    let request = Request {
        credential: *credential_info.key(),
        schema: *schema_info.key(),
        signer: *payer_info.key(),
        nonce: *nonce,
        data: data.to_vec(),
        expiry: expiry,
    };

    log!("process_create_request enter CreateRequestArgs6");
    let mut request_data = request_info.try_borrow_mut_data()?;

    log!("process_create_request enter CreateRequestArgs7");
    request_data.copy_from_slice(&request.to_bytes());


    log!("process_create_request enter CreateRequestArgs8");

    Ok(())
}

pub struct CreateRequestArgs<'a> {
    raw: *const u8,
    _data: PhantomData<&'a [u8]>,
}

impl CreateRequestArgs<'_> {
    #[inline]
    pub fn try_from_bytes(bytes: &[u8]) -> Result<CreateRequestArgs, ProgramError> {
        // The minimum expected size of the instruction data.
        // - nonce (32 bytes)
        // - data (5 bytes. 4 len, 1 byte)
        // - expiry (8 bytes)
        // if bytes.len() < 45 {
        //     return Err(ProgramError::InvalidInstructionData);
        // }

        Ok(CreateRequestArgs {
            raw: bytes.as_ptr(),
            _data: PhantomData,
        })
    }

    #[inline]
    pub fn nonce(&self) -> Result<&Pubkey, ProgramError> {
        // SAFETY: the `bytes` length was validated in `try_from_bytes`.
        unsafe {
            let nonce = &*(self.raw as *const Pubkey);
            Ok(nonce)
        }
    }

    pub fn _data_len(&self) -> usize {
        unsafe { *(self.raw.add(32) as *const u32) as usize }
    }

    #[inline]
    pub fn data(&self) -> Result<&[u8], ProgramError> {
        // SAFETY: the `bytes` length was validated in `try_from_bytes`.
        unsafe {
            let len = self._data_len();
            let data_bytes = core::slice::from_raw_parts(self.raw.add(36), len as usize);
            Ok(data_bytes)
        }
    }

    // #[inline]
    pub fn expiry(&self) -> Result<i64, ProgramError> {
        // SAFETY: the `bytes` length was validated in `try_from_bytes`.
        unsafe {
            let data_len = self._data_len();
            let offset = 32 + 4 + data_len;
            let expiry = *(self.raw.add(offset) as *const i64);
            Ok(expiry)
        }
    }
}
