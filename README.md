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

sh -c "$(curl -sSfL https://release.anza.xyz/v2.1.13/install)"

cargo run --bin cli create-attestation --schema-type gender --recipient 

cargo run --bin cli create-schema --schema-type test_data --schema-name test_data
cargo run --bin cli create-attestation --recipient  \
        --schema-type test_data \
         --schema-name test_data

cargo run --bin cli create-attestation \
        --schema-type gender \
        --schema-name gender \
        --recipient 
2XSC9iEMRyRgZaaQH8BsK1N6Rc7PHyFc9GfoLm4aHmCyZ51aV7LxF5gep7nuGfyEdUAEPTf4wWhvg38Z53SKSsuD

cargo run --bin cli create-attestation \
        --schema-type birth_year \
        --schema-name birth_year \
        --recipient 
4cnHzd3qSw62QCJbBiWJaStF3ahCpFrxW5hZo9PLVZMc17m4pdhbGWx3Td5tisK2HesocgyFrYRXZJFAdJ5kL4UG

cargo run --bin cli create-attestation \
        --schema-type age_over18 \
        --schema-name age_over18 \
        --recipient 
49DgVbPuqjz8vwHBEBg23jEEbtiyrxfsV5DRaSEpBs2adHHkFKCEVkq8w9gP94KETkrLnBwoV83C9eKhfHWTPt43

cargo run --bin cli create-attestation \
        --schema-type age_over21 \
        --schema-name age_over21 \
        --recipient 
2egE3bNLxMjTYZeie1Np6FTBVruYKjNucQaeam8zL4CnBqqP4tLTWtcLohyWyapKnPpVcvcBd2gFuhtPUsAYwgaP

cargo run --bin cli create-attestation \
        --schema-type jurisdiction \
        --schema-name jurisdiction \
        --recipient 

4Lfs7Z9QEc1cy49T7o2jZLFM4ni8BrZVZQoE9UWDofJJzS3n17z4ujkDvXiE5QC1riGH4JivV5XRSr3Z3ndgpZ9w