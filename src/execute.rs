// WE HAVE HERE THE EXECUTE FUNCTION

use crate::state::{STATE};
use crate::error::ContractError;
use cosmwasm_std::{DepsMut, MessageInfo, Response, BankMsg, SubMsg, Coin};

pub fn increment_by (deps: DepsMut, amount: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += amount;
        Ok(state)
    });

    Ok(Response::new()
    .add_attribute("action", "increment")
    .add_attribute("amount", amount.to_string())) //the amount incremented
    // I'm not going to handle errors in this simple function
}

pub fn decrement_by(deps: DepsMut, amount: i32) -> Result<Response, ContractError>{
    STATE.update(deps.storage, |mut state | -> Result<_, ContractError> {
        state.count -= amount;
        if  state.count-amount < 0{
            return Err(ContractError::InvalidDecrement{}) // can't decrement the count if the amount is <= 0
        }
        Ok(state)
        
    });

    Ok(Response::new()
    .add_attribute("action", "decrementBy")
    .add_attribute("amount", amount.to_string())) //the amount decremented
    // I'm not going to handle other errors in this function
}

pub fn reset(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {     //Only the owner can do the reset action 
            return Err(ContractError::Unauthorazied{})
        }
        state.count = 0; // reset the count
        Ok(state) 
    }); 
    Ok(Response::new()
    .add_attribute("action", "reset state"))   
}

pub fn send_funds(deps: DepsMut, address: String, token: Coin) -> Result<Response, ContractError> {
    let verified_addr = deps.api.addr_validate(&address)?; // first we are validating our recipient
    let send_msg: SubMsg = SubMsg::new(BankMsg::Send{to_address: verified_addr.to_string(), amount: vec![token]}); // we crate the Sub Message

    Ok(Response::new().add_submessage(send_msg))
}