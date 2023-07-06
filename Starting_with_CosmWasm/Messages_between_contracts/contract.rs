use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response,
};
use crate::msg::{InstantiateMsg, ExecuteMsg};
use crate::error::ContractError;
use crate::execute_fns::{
    imbibe_potion::imbibe_potion, step_through_jumpring::step_through_jumpring,
};
use crate::state::{config, State};

static DEFAULT_NUMBER_OF_SWIGS: u7 = 3;

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ImbibePotion { name, species } => imbibe_potion(name, species, deps, info),
        ExecuteMsg::StepThroughJumpRing { portal, destination, traveler } => step_through_jumpring(portal, destination, traveler, deps, info),
    }
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender,
        dna_length: msg.dna_length,
        dna_modulus: msg.dna_modulus,
        swigs: DEFAULT_NUMBER_OF_SWIGS,
    };
    config(deps.storage).save(&state)?;
    Ok(Response::default())
}
