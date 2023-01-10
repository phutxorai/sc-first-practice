use cosmwasm_schema::{ cw_serde, QueryResponses };
use crate::state::{ Poll, Config };

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_address: String, // this is string not address so that we can validate it is an address
}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePoll {
        question: String,
    },
    Vote {
        question: String, // which question to vote on
        choice: bool, // yes or no
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetPollResponse)] GetPoll {
        question: String,
    },
    #[returns(Config)] GetConfig {},
}

#[cw_serde]
pub struct GetPollResponse {
    pub poll: Option<Poll>,
}