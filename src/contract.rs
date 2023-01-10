#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary };
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg };
use crate::queries::get_poll;
use crate::execute::{ create_poll, vote };
use crate::state::{ Config, CONFIG };

// version info for migration info
const CONTRACT_NAME: &str = "polls";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION"); // takes env from cargo.toml

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // this will error if the user gives an invalid address as we placed a ? for unwrapping at the end
    let validated_admin_address = deps.api.addr_validate(&msg.admin_address)?;
    let config = Config {
        admin_address: validated_admin_address,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll { question } => create_poll(deps, env, info, question),
        ExecuteMsg::Vote { question, choice } => vote(deps, env, info, question, choice),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPoll { question } => get_poll(deps, env, question),
        QueryMsg::GetConfig {} => to_binary(&CONFIG.load(deps.storage)?),
    }
}