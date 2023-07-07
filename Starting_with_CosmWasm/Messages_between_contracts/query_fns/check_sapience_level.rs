use crate::error::ContractError;
use cosmwasm_std::{to_binary, Addr, DepsMut, MessageInfo, Response, WasmQuery};
use portal::msg::QueryMsg;

fn check_sapience_level(
    portal: &Addr,
    deps: &DepsMut,
    info: &MessageInfo,
) -> Result<Response, ContractError> {
    let query = WasmQuery::Smart {
        contract_addr: portal.to_string(),
        msg: to_binary(&QueryMsg::MinimumSapience {})?,
    };


    Ok(Response::default())
}
