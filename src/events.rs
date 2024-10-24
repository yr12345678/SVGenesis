use scrypto::prelude::*;

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct Generation {
    pub key_image_url: Url,
    pub seed_lossy: String,
    pub non_fungible_local_id: NonFungibleLocalId,
}
