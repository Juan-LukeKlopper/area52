use crate::error::ContractError;
use crate::state::{config};
use cosmwasm_std::{DepsMut, MessageInfo, Response};
use universe::species::Species;

pub fn imbibe_potion(
    name: String,
    species: Species,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // imbibe_potion code needs to added here
}
