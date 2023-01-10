use cosmwasm_std::Addr;
use cw_storage_plus::{ Item, Map };
use schemars::JsonSchema;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_address: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub question: String,
    pub yes_votes: u64,
    pub no_votes: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");

// poll id to poll
pub const POLLS: Map<String, Poll> = Map::new("polls");
pub const USER_VOTED: Map<(String, Addr), bool> = Map::new("user_voted");