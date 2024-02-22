// WE HAVE HERE ALL THE QUERY FUNCTIONS

use crate::state::{STATE};
use cosmwasm_std::{Deps, StdResult};

pub fn has_reset (deps: Deps) -> StdResult<bool>{
    let state = STATE.load(deps.storage)?;
    Ok(state.count == 0) // check if the count is equal 0. This return TRUE if the state is resetted
}