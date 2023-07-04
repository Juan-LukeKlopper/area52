use cosmwasm_std::{to_binary, Binary, Deps, StdResult};
use crate::state::config_read;
use crate::msg::JumpRingCheckResponse;
use universe::species::{Traveler};

pub fn minimum_sapience(deps: Deps) -> StdResult<Binary> {
    let state = config_read(deps.storage).load()?;
    let out = to_binary(&SapienceResponse {
        level: state.minimum_sapience,
    })?;
    Ok(out)
}

pub fn jumpring_check(traveler: Traveler) -> StdResult<Binary> {
    let out = to_binary(&JumpRingCheckResponse {
        valid: traveler.cyberdized,
    })?;
}
