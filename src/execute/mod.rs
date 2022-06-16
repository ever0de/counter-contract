use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::{result::Result, state::STATE, ContractError};

pub fn try_increment(deps: DepsMut) -> Result<Response> {
    STATE.update(deps.storage, |mut state| -> Result<_> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response> {
    STATE.update(deps.storage, |mut state| -> Result<_> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "reset"))
}
