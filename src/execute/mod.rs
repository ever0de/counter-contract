use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response};

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
        check_owner(&info.sender, &state.owner)?;
        state.count = count;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "reset"))
}

fn check_owner(lhs: &Addr, hrs: &Addr) -> Result<()> {
    if lhs != hrs {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(())
    }
}
