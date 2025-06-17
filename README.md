# Solana Attestation Service

Built by [Exo Technologies](https://exotechnologies.xyz) with support from Solana Foundation

## Running Tests

Run integration tests with the following script

```
cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo test
```

## Generating IDL

This repository uses Shank for IDL generation.

Install the Shank CLI

```
cargo install shank-cli
```

Generate IDL

```
shank idl -r program -o idl
// OR
pnpm run generate-idl
```

## Generating Clients

_Ensure the IDL has been created or updated using the above IDL generation steps._

Install dependencies

```
pnpm install
```

Run client generation script

```
pnpm run generate-clients
```


pnpm run generate-idl && pnpm run generate-clients 

solana program set-upgrade-authority <PROGRAM_ID> --new-upgrade-authority 8Ez6wwXdS58PyBGxGSB3MQuMRuJsAXaxhMwE79ngVxzU --keypair /Users/gaomin/.config/solana/attestation_admin_test.json --url https://api.devnet.solana.com

## 
terry's owner: 
    CJmTPWkPX9GepGNkATHZf2vnEZUr9UVt5MqiDhKPLGVs

solana program set-upgrade-authority D5PPvGRyK6ii3zjUn1zsuetjZbt6tzqL7VkhNiV4UDsJ \
    --new-upgrade-authority 8Ez6wwXdS58PyBGxGSB3MQuMRuJsAXaxhMwE79ngVxzU \
    --url https://api.devnet.solana.com

solana program set-upgrade-authority 3WaA2C9VRHczjqcdVgWw8Ug2VfoCVbCzEp9bwPPG6Qj6
    --new-upgrade-authority ~/.config/solana/rnspay_admin.json \

solana program \
    set-upgrade-authority 3WaA2C9VRHczjqcdVgWw8Ug2VfoCVbCzEp9bwPPG6Qj6 \
    --new-upgrade-authority ~/.config/solana/rnspay_admin.json \ 
    -k ~/.config/solana/attestation_admin_test.json  \ 
    --url https://api.devnet.solana.com
 




request program_id: D5PPvGRyK6ii3zjUn1zsuetjZbt6tzqL7VkhNiV4UDsJ
request authority: A6WcyjnyU4nBD66tKxzg35bYCkeNqF4MCtQr7pwreVAv
request credential_name: rns_credential_1
request schema_name: jurisdiction_3
request query_type: Attestation


request program_id: D5PPvGRyK6ii3zjUn1zsuetjZbt6tzqL7VkhNiV4UDsJ
request authority: A6WcyjnyU4nBD66tKxzg35bYCkeNqF4MCtQr7pwreVAv
request credential_name: rns_credential_1
request schema_name: jurisdiction_3


Credential PDA: E4ccYoQdJZhypRPVf1YD3mrLH4NKN9iJqYanf6jKtcvD
Schema PDA: Bi6dbk7yAQYtumf7jDzZkHYa8nJFTLZffSSWcWaJdDqo


Credential PDA: E4ccYoQdJZhypRPVf1YD3mrLH4NKN9iJqYanf6jKtcvD
Schema PDA: Bi6dbk7yAQYtumf7jDzZkHYa8nJFTLZffSSWcWaJdDqo




cargo run --bin cli create-credential

cargo run --bin cli create-schema --credential-name RNS_PROOF --schema-type gender --schema-name gender
cargo run --bin cli create-schema --credential-name RNS_PROOF --schema-type birth_year --schema-name birth_year
cargo run --bin cli create-schema --credential-name RNS_PROOF --schema-type age_over21 --schema-name age_over21
cargo run --bin cli create-schema --credential-name RNS_PROOF --schema-type age_over18 --schema-name age_over18
    J6RGBiAd3PQHgVdasfJx19NU1aa5GjxKLJmtMRtzbDf2
cargo run --bin cli create-schema --credential-name RNS_PROOF --schema-type jurisdiction --schema-name jurisdiction
    Had9DqEanDdiYq1D3kMMBa3rNcMstLjP8J1eH8DthCoM



22zoJMtdu4tQc2PzL74ZUT7FrwgB1Udec8DdW4yw4BdG
cargo run --bin cli create-schema \
    --credential-name RNS_PROOF \
    --schema-name jurisdiction \
    --schema-type jurisdiction


    