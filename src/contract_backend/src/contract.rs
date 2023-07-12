use std::borrow::Cow;
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{BoundedStorable, Storable};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Contract
{
    pub version: u128,
    pub date_issued: String,
    pub date_updated: String,
    pub contract_text: String,
}

// The `Storable` trait is already implemented for several common types (e.g. u64),
// so you can use those directly without implementing the `Storable` trait for them.
impl Storable for Contract
{
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Contract
{
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}