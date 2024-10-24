This repository contains an experimental setup for generating and hosting SVG-based NFTs completely on the Radix DLT (https://www.radixdlt.com). It's meant as an inspiration for others to use in their own projects!

To make this work, this repository uses:
* Mleekko's .Random for pseudo-randomness: https://github.com/dot-random/dot-random
* A modified version of an SVG generator (https://github.com/bodoni/svg/) from which any references to floats are stripped (as they're not allowed), which can be found at https://github.com/yr12345678/svg

This NFT project utilises the fact that the `key_image_url` field supports SVG data URIs. Browsers render this natively. The official wallets support rendering the SVGs through the image server they use, because of lacking SVG renderer libraries for (mostly) Swift. This means that we don't have to store images externally and can generate and store the NFTs on-ledger completely.

# Setup
The project has a simple NFT generation system that allows you to define layers and exclusions. It then picks layers taking into account these exclusions, but also any weights you've assigned to the layers.

## Layer trait
Each layer implements the Layer trait, which defines the following behavior:
1. `generate`: generates the SVG code for this layer
2. `exclusions`: returns a vector of TypeIds to exclude when this layer is picked
3. `layer_type`: returns the TypeId for this layer
4. `layer_name`: returns the layer struct's name to be used in the NFT's metadata

## Layer categories
The project structures layers in categories (background, frame, big element, small element) that each have their own folder. The `mod.rs` in this folder exports all the individual layers and contains a method to return a random layer, which is called by the NFT generator. This `random_...` method lists all available layers in the category and their respective weights. It also takes any exclusions that it has to take into account and thus filters out any layers that are not allowed.

## Layers
Individual layers have their own files in the layer category folders. They contain the code required to generate the SVG code for that specific layer. It returns a vector of `Element`s which are later compiled into an SVG document.

## NFT generator
The NFT generator is responsible for tying it all together. It sets up the structure of the NFT with the order of layer categories and calls the methods to randomly pick layers within those categories, taking into account any exlcusions. Once it has all the layers, it generates the SVG data by calling the `generate()` method on each layer.

## HSL
A custom HSL implementation to generate random colors or colors based off another color. It's probably not the greatest implementation, but it prevented adding another dependency from which references to floats had to be stripped.

The following methods are available:
* `new`
* `new_random`
* `triadic_colors` (also has an `as_strings` variant)
* `analogous_colors` (also has an `as_strings` variant)
* `complementary_colors` (also has an `as_strings` variant)
* `monochromatic_colors` (also has an `as_strings` variant)
* `split_complementary_colors` (also has an `as_strings` variant)
* `derive_similar_color`
* `as_string`

## Utils
Contains some handy methods for repetitive actions, such as:
* Generating SVG gradients
* Picking a random layer
* Excluding layers

## Blueprint
The Scrypto part of this project is actually pretty straight-forward. It just generates the NFT SVG data, turns that into a data URI for the `key_image_url` and mints an NFT with it.

## Randomness
This project uses pseudo-randomness via .Random to generate NFTs. The collection this project generates is unlimited and free and there's not really a concept of rarity, so users are able to provide their own seeds for this. 

If you plan on generating a collection that is limited, costs money and/or has the concept of rarity, you **should not allow anyone to provide a seed themselves**. Instead, you should be using the RandomComponent described in the .Random documentation: https://github.com/dot-random/dot-random.

# Components
If you want to integrate SVGenesis mints in your product, use the components below.

## Stokenet
`component_tdx_2_1cqs9mhe56cpj4fakklcup3ax6jr00hvyrheg2ju6ua4xhh3h5r536q`

## Mainnet
`component_rdx1cpjnvp4q44mjdngrgyhvyhvwva4mhac6wgrj8msjx00tmszkz850ty`

# Interface
The component instantiated from this blueprint has two methods:
* `seed_used(seed: Vec<u8>)`: returns `(bool, Option<NonFungibleLocalId>)` indicating whether a seed was already used, and if so, which NFT was generated with it.
* `mint_nft(seed: Vec<u8>)`: returns a `Bucket` with the generated NFT, as long as the seed was not already used and the generated SVG code does not already exist. 

Seeds are provided hex-encoded. You can use something like https://www.browserling.com/tools/random-hex to generate a random seed. **Seed length must be a multiple of 4** (this is a requirement of .Random).

## Manifests
### Mint NFT
```
CALL_METHOD
    Address("SVGENESIS_COMPONENT_ADDRESS") # Put the SVGenesis component address here
    "mint_nft"
    Bytes("HEX_ENCODED_SEED") # Put a hex-encoded seed here
;
CALL_METHOD
    Address("YOUR_ACCOUNT") # Put an account address here
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```
### Check if seed was used
```
CALL_METHOD
    Address("YOUR_ACCOUNT") # Put an account address here
    "seed_used"
    Bytes("HEX_ENCODED_SEED")
;
```

## Events
### Generation
The component emits an event upon NFT mint called `Generation` with the following fields:
* `key_image_url`: the `key_image_url` as included in the NFT data. This contains the SVG data URI.
* `seed_lossy`: the (lossy) seed used to mint this NFT.
* `non_fungible_local_id`: the NonFungibleLocalId for this NFT.

# The NFTs
This project generates NFTs with shapes that have different sizes and colors. Most of these have an equal chance of occurring, but for esthetical reasons, some things are less likely to occur, such as gradients vs. solid colors. Sometimes a base color is generated, from which all subsequent colors are derived (see `derive_similar_color` under HSL). Also, some layers exclude other layers, because they simply don't work well together.

While the collection is random, has a ton of possible variants, excludes used seeds and stores hashes of already used SVG code, it is probably possible to still generate an NFT that looks the same as another, because the SVG code might be different, but the visual result the same. Chances for this should be small, but probably not zero.
