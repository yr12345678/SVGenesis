use events::Generation;
use scrypto::prelude::*;
use types::SVGenesisNFT;

pub mod events;
pub mod hsl;
pub mod layers;
pub mod nft_generator;
pub mod types;
pub mod utils;

#[blueprint]
#[types(SVGenesisNFT, Vec<u8>, Hash, NonFungibleLocalId)]
#[events(Generation)]
mod svgenesis {
    enable_method_auth! {
        roles {
            admin_role => updatable_by: [OWNER];
        },
        methods {
            mint_nft => PUBLIC;
            seed_used => PUBLIC;
            mint_admin_badge => restrict_to: [OWNER];
        }
    }

    struct SVGenesis {
        svgenesis_manager: ResourceManager,
        next_nft_id: u64,
        used_seeds: KeyValueStore<Vec<u8>, NonFungibleLocalId>,
        existing_hashes: KeyValueStore<Hash, NonFungibleLocalId>,
        owner_badge: ResourceAddress,
        admin_badge: ResourceAddress,
    }

    impl SVGenesis {
        pub fn instantiate() -> (Global<SVGenesis>, FungibleBucket) {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(SVGenesis::blueprint_id());

            // Create owner badge
            let owner_badge = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata! {
                    init {
                        "name" => "SVGenesis owner badge", locked;
                        "symbol" => "SVGOWN", locked;
                        "description" => "The owner badge for the SVGenesis NFT collection.", locked;
                        "icon_url" => Url::of("https://i.ibb.co/gJY74HX/svgenesis.png"), locked;
                    }
                })
                .mint_roles(mint_roles!(
                    minter => rule!(deny_all);
                    minter_updater => rule!(deny_all);
                ))
                .burn_roles(burn_roles!(
                    burner => rule!(allow_all);
                    burner_updater => rule!(deny_all);
                ))
                .mint_initial_supply(1);

            // Create admin badge
            let admin_badge_manager = ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(
                require(owner_badge.resource_address())
            )))
            .divisibility(DIVISIBILITY_NONE)
            .metadata(metadata! {
                init {
                    "name" => "SVGenesis admin badge", locked;
                    "symbol" => "SVGADM", locked;
                    "description" => "An admin badge for the SVGenesis NFT collection.", locked;
                    "icon_url" => Url::of("https://i.ibb.co/gJY74HX/svgenesis.png"), locked;
                }
            })
            .mint_roles(mint_roles!(
                minter => rule!(require(global_caller(component_address)));
                minter_updater => rule!(deny_all);
            ))
            .burn_roles(burn_roles!(
                burner => rule!(allow_all);
                burner_updater => OWNER;
            ))
            .recall_roles(recall_roles!(
                recaller => OWNER;
                recaller_updater => OWNER;
            ))
            .create_with_no_initial_supply();

            // Metadata setter rule, we allow the admin to update metadata too to update the dApp definition
            let metadata_setter_rule = rule!(
                require(owner_badge.resource_address()) || require(admin_badge_manager.address())
            );

            // Create the NFT manager
            let svgenesis_manager = ResourceBuilder::new_integer_non_fungible_with_registered_type::<SVGenesisNFT>(OwnerRole::Updatable(rule!(require(owner_badge.resource_address()))))
                .mint_roles(mint_roles!(
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => OWNER;
                ))
                .burn_roles(burn_roles!(
                    burner => rule!(deny_all);
                    burner_updater => rule!(deny_all);
                ))
                .metadata(metadata! {
                    roles {
                        metadata_setter => metadata_setter_rule.clone();
                        metadata_setter_updater => OWNER;
                        metadata_locker => OWNER;
                        metadata_locker_updater => rule!(deny_all);
                    },
                    init {
                        "name" => "SVGenesis", locked;
                        "description" => "SVGenesis is an experimental NFT collection that's generated and hosted completely on-ledger. It's free and unlimited.", updatable;
                        "icon_url" => Url::of("https://i.ibb.co/gJY74HX/svgenesis.png"), updatable;
                        "tags" => vec!["nft", "collection", "svg"], updatable;
                    }
                })
                .create_with_no_initial_supply();

            // Instantiate the component
            let svgenesis_component = Self {
                svgenesis_manager,
                next_nft_id: 1,
                used_seeds: KeyValueStore::<Vec<u8>, NonFungibleLocalId>::new_with_registered_type(
                ),
                existing_hashes:
                    KeyValueStore::<Hash, NonFungibleLocalId>::new_with_registered_type(),
                owner_badge: owner_badge.resource_address(),
                admin_badge: admin_badge_manager.address(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Updatable(rule!(require(
                owner_badge.resource_address()
            ))))
            .roles(roles! {
                admin_role => rule!(require(admin_badge_manager.address()));
            })
            .metadata(metadata! (
                roles {
                    metadata_setter => metadata_setter_rule.clone();
                    metadata_setter_updater => OWNER;
                    metadata_locker => OWNER;
                    metadata_locker_updater => rule!(deny_all);
                },
                init{
                    "name" => "SVGenesis minter", updatable;
                    "description" => "The component that mints SVGenesis NFTs.", updatable;
                    "tags" => vec!["nft", "collection"], updatable;
                }
            ))
            .with_address(address_reservation)
            .globalize();

            (svgenesis_component, owner_badge)
        }

        /// Mints an SVGenesis NFT using the provided seed.
        ///
        /// Returns a Bucket containing the minted NFT.
        ///
        /// Panics if:
        /// * The seed is not a multiple of 4
        /// * The seed is already used
        /// * The resulting SVG data already exists
        pub fn mint_nft(&mut self, seed: Vec<u8>) -> Bucket {
            // Make sure seed length is multiple of 4
            assert!(seed.len() % 4 == 0, "Seed length must be a multiple of 4!");

            // Make sure we can't reuse seeds
            assert!(
                self.used_seeds.get(&seed).is_none(),
                "Seed was already used! Try another one :)"
            );

            // Generate our SVG data
            let (nft_image_data, layers) = nft_generator::generate_nft_image_data(&seed);
            let url_encoded_nft_image_data = urlencoding::encode(&nft_image_data).into_owned();
            let svg_data_uri = format!("data:image/svg+xml,{url_encoded_nft_image_data}");
            let svg_data_hash = hash(nft_image_data.clone());

            // Make sure hash does not yet exist
            assert!(
                self.existing_hashes.get(&svg_data_hash).is_none(),
                "This image already exsists!"
            );

            // Mint the NFT
            let nft_id = NonFungibleLocalId::integer(self.next_nft_id);
            let nft_bucket = self.svgenesis_manager.mint_non_fungible::<SVGenesisNFT>(
                &nft_id,
                SVGenesisNFT {
                    key_image_url: Url::of(svg_data_uri.clone()),
                    name: format!("SVGenesis #{}", self.next_nft_id),
                    // Can't guarantee that all characters will be valid UTF-8, so this is basically best-effort and for fun if someone wants to use their own vanity seed
                    seed_lossy: String::from_utf8_lossy(&seed).into_owned(),
                    layers,
                    svg_data: hex::encode(nft_image_data),
                },
            );

            // Generate mint event
            Runtime::emit_event(Generation {
                key_image_url: Url::of(svg_data_uri.clone()),
                seed_lossy: String::from_utf8_lossy(&seed).into_owned(),
                non_fungible_local_id: NonFungibleLocalId::from(self.next_nft_id),
            });

            // Add the hash, seed and NonFungibleLocalId to the used_seeds and existing_hashes KeyValueStores
            self.used_seeds.insert(seed, nft_id.clone());
            self.existing_hashes.insert(svg_data_hash, nft_id.clone());

            // Increment our NFT id counter for the next mint
            self.next_nft_id += 1;

            nft_bucket
        }

        /// Checks if a seed was used.
        ///
        /// Returns a tuple with a bool and optionally a NonFungibleLocalId for the NFT that was minted with this seed.
        pub fn seed_used(&self, seed: Vec<u8>) -> (bool, Option<NonFungibleLocalId>) {
            match self.used_seeds.get(&seed) {
                Some(nflid) => (true, Some(nflid.to_owned())),
                None => (false, None),
            }
        }

        /// Mints an admin badge
        ///
        /// Returns a Bucket with the admin badge
        pub fn mint_admin_badge(&mut self) -> Bucket {
            ResourceManager::from_address(self.admin_badge).mint(1)
        }
    }
}
