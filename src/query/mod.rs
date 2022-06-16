use cosmwasm_std::{Deps, StdResult};

use crate::{msg::CountResponse, state::STATE};

pub(crate) fn count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(CountResponse { count: state.count })
}
