/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  addDecoderSizePrefix,
  addEncoderSizePrefix,
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU32Decoder,
  getU32Encoder,
  getU8Decoder,
  getU8Encoder,
  getUtf8Decoder,
  getUtf8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
  type WritableSignerAccount,
} from '@solana/kit';
import { SOLANA_ATTESTATION_SERVICE_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const CHANGE_SCHEMA_DESCRIPTION_DISCRIMINATOR = 4;

export function getChangeSchemaDescriptionDiscriminatorBytes() {
  return getU8Encoder().encode(CHANGE_SCHEMA_DESCRIPTION_DISCRIMINATOR);
}

export type ChangeSchemaDescriptionInstruction<
  TProgram extends string = typeof SOLANA_ATTESTATION_SERVICE_PROGRAM_ADDRESS,
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountAuthority extends string | IAccountMeta<string> = string,
  TAccountCredential extends string | IAccountMeta<string> = string,
  TAccountSchema extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountPayer extends string
        ? WritableSignerAccount<TAccountPayer> &
            IAccountSignerMeta<TAccountPayer>
        : TAccountPayer,
      TAccountAuthority extends string
        ? ReadonlySignerAccount<TAccountAuthority> &
            IAccountSignerMeta<TAccountAuthority>
        : TAccountAuthority,
      TAccountCredential extends string
        ? ReadonlyAccount<TAccountCredential>
        : TAccountCredential,
      TAccountSchema extends string
        ? WritableAccount<TAccountSchema>
        : TAccountSchema,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type ChangeSchemaDescriptionInstructionData = {
  discriminator: number;
  description: string;
};

export type ChangeSchemaDescriptionInstructionDataArgs = {
  description: string;
};

export function getChangeSchemaDescriptionInstructionDataEncoder(): Encoder<ChangeSchemaDescriptionInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['description', addEncoderSizePrefix(getUtf8Encoder(), getU32Encoder())],
    ]),
    (value) => ({
      ...value,
      discriminator: CHANGE_SCHEMA_DESCRIPTION_DISCRIMINATOR,
    })
  );
}

export function getChangeSchemaDescriptionInstructionDataDecoder(): Decoder<ChangeSchemaDescriptionInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['description', addDecoderSizePrefix(getUtf8Decoder(), getU32Decoder())],
  ]);
}

export function getChangeSchemaDescriptionInstructionDataCodec(): Codec<
  ChangeSchemaDescriptionInstructionDataArgs,
  ChangeSchemaDescriptionInstructionData
> {
  return combineCodec(
    getChangeSchemaDescriptionInstructionDataEncoder(),
    getChangeSchemaDescriptionInstructionDataDecoder()
  );
}

export type ChangeSchemaDescriptionInput<
  TAccountPayer extends string = string,
  TAccountAuthority extends string = string,
  TAccountCredential extends string = string,
  TAccountSchema extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  payer: TransactionSigner<TAccountPayer>;
  authority: TransactionSigner<TAccountAuthority>;
  /** Credential the Schema is associated with */
  credential: Address<TAccountCredential>;
  /** Credential the Schema is associated with */
  schema: Address<TAccountSchema>;
  systemProgram?: Address<TAccountSystemProgram>;
  description: ChangeSchemaDescriptionInstructionDataArgs['description'];
};

export function getChangeSchemaDescriptionInstruction<
  TAccountPayer extends string,
  TAccountAuthority extends string,
  TAccountCredential extends string,
  TAccountSchema extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends
    Address = typeof SOLANA_ATTESTATION_SERVICE_PROGRAM_ADDRESS,
>(
  input: ChangeSchemaDescriptionInput<
    TAccountPayer,
    TAccountAuthority,
    TAccountCredential,
    TAccountSchema,
    TAccountSystemProgram
  >,
  config?: { programAddress?: TProgramAddress }
): ChangeSchemaDescriptionInstruction<
  TProgramAddress,
  TAccountPayer,
  TAccountAuthority,
  TAccountCredential,
  TAccountSchema,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? SOLANA_ATTESTATION_SERVICE_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    payer: { value: input.payer ?? null, isWritable: true },
    authority: { value: input.authority ?? null, isWritable: false },
    credential: { value: input.credential ?? null, isWritable: false },
    schema: { value: input.schema ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.authority),
      getAccountMeta(accounts.credential),
      getAccountMeta(accounts.schema),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getChangeSchemaDescriptionInstructionDataEncoder().encode(
      args as ChangeSchemaDescriptionInstructionDataArgs
    ),
  } as ChangeSchemaDescriptionInstruction<
    TProgramAddress,
    TAccountPayer,
    TAccountAuthority,
    TAccountCredential,
    TAccountSchema,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedChangeSchemaDescriptionInstruction<
  TProgram extends string = typeof SOLANA_ATTESTATION_SERVICE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    payer: TAccountMetas[0];
    authority: TAccountMetas[1];
    /** Credential the Schema is associated with */
    credential: TAccountMetas[2];
    /** Credential the Schema is associated with */
    schema: TAccountMetas[3];
    systemProgram: TAccountMetas[4];
  };
  data: ChangeSchemaDescriptionInstructionData;
};

export function parseChangeSchemaDescriptionInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedChangeSchemaDescriptionInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 5) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      payer: getNextAccount(),
      authority: getNextAccount(),
      credential: getNextAccount(),
      schema: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getChangeSchemaDescriptionInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
