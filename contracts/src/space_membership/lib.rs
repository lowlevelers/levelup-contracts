#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod types;

#[openbrush::implementation(Ownable)]
#[openbrush::contract]
mod space_membership {
    use crate::types::{MembershipData, SpaceMembershipError, WalletAddress};
    use openbrush::{contracts::ownable, modifiers, storage::Mapping, traits::Storage};

    #[ink(storage)]
    #[derive(Storage)]
    pub struct SpaceMembership {
        #[storage_field]
        ownable: ownable::Data,
        // records of subscribed members
        membership: Mapping<WalletAddress, MembershipData>,
        // address of the governance token
        governance_token: AccountId,
    }

    impl SpaceMembership {
        pub fn partial_new(governance_token: AccountId) -> Self {
            Self {
                governance_token,
                membership: Default::default(),
                ownable: Default::default(),
            }
        }

        #[ink(constructor)]
        pub fn new(governance_token: AccountId) -> Self {
            let mut instance = Self::partial_new(governance_token);
            let caller = instance.env().caller();
            ownable::InternalImpl::_init_with_owner(&mut instance, caller);
            return instance;
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn inc_experience(
            &mut self,
            member_id: AccountId,
            exp: u64,
        ) -> Result<(), SpaceMembershipError> {
            if let Some(membership) = self.membership.get(&member_id) {
                self.membership.insert(
                    &member_id,
                    &MembershipData {
                        exp_points: membership.exp_points + exp,
                        ..membership
                    },
                );
                return Ok(());
            }
            Ok(())
        }
    }
}
