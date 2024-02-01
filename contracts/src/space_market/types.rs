use openbrush::{
    contracts::psp34::Id,
    storage::Mapping,
    traits::{Balance, String},
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct SpaceNftData {
    pub last_token_id: u64,
    pub collection_id: u32,
    pub max_supply: u64,
    pub price_per_mint: Balance,
    pub fid_list: Mapping<Id, String>,
    pub sale_list: Mapping<Id, Balance>,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum SpaceNftError {
    BadMintValue,
    CollectionIsFull,
    WithdrawalFailed,
    NotTokenOwner,
    NotForSale,
    OwnToken,
    PriceNotMatch,
    TransferNativeTokenFailed,
}

impl SpaceNftError {
    #[allow(unused)]
    pub fn as_str(&self) -> String {
        match self {
            SpaceNftError::BadMintValue => String::from("BadMintValue"),
            SpaceNftError::CollectionIsFull => String::from("CollectionIsFull"),
            SpaceNftError::WithdrawalFailed => String::from("WithdrawalFailed"),
            SpaceNftError::NotTokenOwner => String::from("NotTokenOwner"),
            SpaceNftError::NotForSale => String::from("NotForSale"),
            SpaceNftError::OwnToken => String::from("OwnToken"),
            SpaceNftError::PriceNotMatch => String::from("PriceNotMatch"),
            SpaceNftError::TransferNativeTokenFailed => String::from("TransferNativeTokenFailed"),
        }
    }
}
