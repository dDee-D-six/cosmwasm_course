use cw_storage_plus::Item;
use cosmwasm_std::Uint64;

pub const DEE_COUNT: Item<Uint64> = Item::new("dee_count");
