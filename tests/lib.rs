use radix_common::network::NetworkDefinition;
use rand::prelude::*;
use scrypto_test::prelude::*;
use scrypto_test::utils::dump_manifest_to_file_system;
use std::fs;
use svgenesis::{svgenesis_test::*, types::SVGenesisNFT};
use resvg;

#[test]
fn can_mint_nft() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address =
        PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;

    let (mut svgenesis, _) = SVGenesis::instantiate(package_address, &mut env)?;

    let mut data = [0u8; 128];
    rand::thread_rng().fill_bytes(&mut data);

    // Act
    let first_mint = svgenesis.mint_nft(data.to_vec(), &mut env);

    // Assert
    assert!(first_mint.is_ok());

    Ok(())
}

#[test]
fn cannot_mint_with_same_seed() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address =
        PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;

    let (mut svgenesis, _) = SVGenesis::instantiate(package_address, &mut env)?;

    env.disable_auth_module();

    let mut data = [0u8; 128];
    rand::thread_rng().fill_bytes(&mut data);

    // Act
    let _first_mint = svgenesis.mint_nft(data.to_vec(), &mut env)?;
    let second_mint = svgenesis.mint_nft(data.to_vec(), &mut env);

    // Assert
    assert!(second_mint.is_err());

    Ok(())
}

#[test]
fn cannot_mint_with_wrong_seed_length() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address =
        PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;

    let (mut svgenesis, _) = SVGenesis::instantiate(package_address, &mut env)?;

    env.disable_auth_module();

    let mut data = [0u8; 3];
    rand::thread_rng().fill_bytes(&mut data);

    // Act
    let result = svgenesis.mint_nft(data.to_vec(), &mut env);

    // Assert
    assert!(result.is_err());

    Ok(())
}

#[test]
fn owner_can_mint_admin_badge() {
    // Setup the environment
    let mut ledger = LedgerSimulatorBuilder::new().without_kernel_trace().build();

    // Create an account
    let (public_key, _private_key, account) = ledger.new_allocated_account();

    // Publish the package
    let package_address = ledger.compile_and_publish(this_package!());

    // Instantiate the component
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address,
            "SVGenesis",
            "instantiate",
            manifest_args!(),
        )
        .deposit_batch(account)
        .build();

    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    // Get the component and resource addresses
    let component = receipt.expect_commit_success().new_component_addresses()[0];
    let owner_badge = receipt.expect_commit_success().new_resource_addresses()[0];

    // Mint admin badge
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            account,
            "create_proof_of_amount",
            manifest_args!(owner_badge, dec!(1)),
        )
        .call_method(component, "mint_admin_badge", manifest_args!())
        .deposit_batch(account)
        .build();

    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    // Assert
    receipt.expect_commit_success();
}

// See if we run into any limits when minting a ton of these NFTs
// #[test]
fn limits_test() {
    // Setup the environment
    let mut ledger = LedgerSimulatorBuilder::new().without_kernel_trace().build();

    // Create an account
    let (public_key, _private_key, account) = ledger.new_allocated_account();

    // Publish the package
    let package_address = ledger.compile_and_publish(this_package!());

    // Instantiate the component
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            package_address,
            "SVGenesis",
            "instantiate",
            manifest_args!(),
        )
        .deposit_batch(account)
        .build();

    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    // Get the component address
    let component = receipt.expect_commit_success().new_component_addresses()[0];

    // Perform a mint X times
    for i in 0..500000 {
        // Generate a seed
        let mut seed = [0u8; 128];
        rand::thread_rng().fill_bytes(&mut seed);

        // Mint NFT
        let manifest = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .call_method(component, "mint_nft", manifest_args!(seed.to_vec()))
            .deposit_batch(account)
            .build();

        let receipt = ledger.execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&public_key)],
        );

        // Print fee cost
        println!(
            "#{} execution cost: {} XRD",
            i + 1,
            receipt.fee_summary.total_cost()
        );

        // Assert
        receipt.expect_commit_success();
    }
}

// Mint a load of NFTs for test/review purposes
#[test]
fn mint_nft_batch() -> Result<(), RuntimeError> {
    // Instantiate the component
    let mut env = TestEnvironment::new();
    let package_address =
        PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;

    let (mut svgenesis, _) = SVGenesis::instantiate(package_address, &mut env)?;

    // Disable auth and limits so we don't run into unnecessary issues here
    env.disable_auth_module();
    env.disable_limits_module();

    // Create images directory if necessary
    let _ = fs::create_dir_all("test_images");

    // Mint X NFTs and write them to disk
    for i in 1..1001 {
        // Generate a seed
        let mut data = [0u8; 128];
        rand::thread_rng().fill_bytes(&mut data);

        // Mint the NFT
        let nft_bucket = svgenesis.mint_nft(data.to_vec(), &mut env)?;

        // Get the NFT data
        let resource_manager = ResourceManager(nft_bucket.resource_address(&mut env)?);
        let nft_data = resource_manager.get_non_fungible_data::<_, _, SVGenesisNFT>(
            nft_bucket
                .non_fungible_local_ids(&mut env)?
                .first()
                .unwrap()
                .clone(),
            &mut env,
        )?;

        println!("{:?}: {:#?}", nft_data.name, nft_data.layers);

        let svg_data = hex::decode(nft_data.svg_data).unwrap();

        // Write to disk
        fs::write(
            format!("test_images/{i}.svg"),
            svg_data,
        )
        .expect("Failed to write SVG file.");
    }

    Ok(())
}

// Build a manifest that mints 10 NFTs for testing on Stonknet
#[test]
fn build_mint_manifest() -> Result<(), RuntimeError> {
    let mut manifest = ManifestBuilder::new();

    // Generate the mint instructions
    for _ in 0..10 {
        // Generate a seed to use
        let mut data = [0u8; 128];
        rand::thread_rng().fill_bytes(&mut data);

        let component_address = GlobalAddress::try_from_bech32(
            &AddressBech32Decoder::new(&NetworkDefinition::stokenet()),
            "component_tdx_2_1cz7tlcgchcmruknrwq5t42atcvkdgyfjm7w40vn24xn22j2e2w7sz7",
        )
        .unwrap();

        manifest =
            manifest.call_method(component_address, "mint_nft", manifest_args!(data.to_vec()));
    }

    // Deposit the minted NFTs
    manifest = manifest.deposit_batch(
        GlobalAddress::try_from_bech32(
            &AddressBech32Decoder::new(&NetworkDefinition::stokenet()),
            "account_tdx_2_1292jyxrlexx6m877v038jmyjs0cna83l3suppctuy257x5a4unjqds",
        )
        .unwrap(),
    );

    // Write the manifest to disk
    let _ = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./manifests",
        Some("mint_manifest"),
        &NetworkDefinition::stokenet(),
    );

    Ok(())
}
