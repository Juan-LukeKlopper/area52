use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response
};
use crate::error::ContractError;
use crate::msg::{InstantiateMsg};
use crate::state::{config, State};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
        planet_name: msg.planet_name,
        planet_sapients: msg.planet_sapients,
        minimum_sapience: msg.minimum_sapience,
    };
    config(deps.storage).save(&state)?;
    
    Ok(Response::new()
        .add_attribute("owner", state.owner)
        .add_attribute("minimum_sapience", state.minimum_sapience.as_str()))
}
