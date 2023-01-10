use cosmwasm_std::{ Deps, Env, StdResult, Binary, to_binary };

use crate::{ state::POLLS, msg::GetPollResponse };

pub fn get_poll(deps: Deps, _env: Env, question: String) -> StdResult<Binary> {
    let poll = POLLS.may_load(deps.storage, question)?;
    to_binary(&(GetPollResponse { poll }))
}