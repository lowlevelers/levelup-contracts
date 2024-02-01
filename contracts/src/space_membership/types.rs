use openbrush::{contracts::ownable::OwnableError, traits::AccountId};

// alias types
pub type WalletAddress = AccountId;

#[derive(scale::Decode, scale::Encode)]
#[cfg_attr(
    feature = "std",
    derive(
        Debug,
        PartialEq,
        Eq,
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout
    )
)]
pub struct MembershipData {
    pub exp_points: u64,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum SpaceMembershipError {
    NotSpaceAdmin,
    OwnershipError(OwnableError),
}

impl From<OwnableError> for SpaceMembershipError {
    fn from(value: OwnableError) -> Self {
        return SpaceMembershipError::OwnershipError(value);
    }
}
