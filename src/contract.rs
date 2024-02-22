#[cfg(not(feature = "library"))] 
use cosmwasm_std::entry_point; 
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
//use cosmwasm_std::Addr;
//use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use crate::execute::{*};
use crate::query::{*};



//const CONTRACT_NAME: &str = "crates.io:test1";
//const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };

    STATE.save(deps.storage,&state);

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
    
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => increment_by(deps,1),
        ExecuteMsg::IncrementBy {amount} => increment_by(deps, amount),
        ExecuteMsg::DecrementBy {amount} => decrement_by(deps, amount), 
        ExecuteMsg::Reset {} => reset(deps, info),
        ExecuteMsg::SendFunds{address, token} => send_funds(deps, address, token),        
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::HasReset {} => to_binary(&has_reset(deps)?),
    }
}


#[cfg(test)]
mod tests {}
 