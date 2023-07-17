use crate::error::ContractError;
use crate::execute_fns::check_sapience_level::check_sapience_level;
use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response};
use universe::species::Traveler;

pub fn step_through_jumpring(
    portal: Addr,
    destination: Addr,
    traveler: Traveler,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    check_sapience_level(&portal, &deps, &info)?;

    if traveler.cyberdized != true {
        return Err(ContractError::NotACyborg {});
    }

    Ok(Response::default())
}
