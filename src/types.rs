use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData, Clone, Debug)]
pub struct SVGenesisNFT {
    pub key_image_url: Url,
    pub name: String,
    pub seed_lossy: String,
    pub layers: Vec<String>,
    pub svg_data: String,
}
