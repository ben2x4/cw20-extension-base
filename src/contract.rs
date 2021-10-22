#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Empty};

use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::ContractError;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<Empty>, ContractError> {
    Ok(cw20_base::contract::instantiate(deps, _env, _info, msg)?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<Empty>, ContractError> {
    Ok(match msg {
        ExecuteMsg::Transfer { recipient, amount } => {
            cw20_base::contract::execute_transfer(deps, _env, info, recipient, amount)
        }
        ExecuteMsg::Burn { amount } => cw20_base::contract::execute_burn(deps, _env, info, amount),
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => cw20_base::contract::execute_send(deps, _env, info, contract, amount, msg),
        ExecuteMsg::Mint { recipient, amount } => cw20_base::contract::execute_mint(deps, _env, info, recipient, amount),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => cw20_base::allowances::execute_increase_allowance(deps, _env, info, spender, amount, expires),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => cw20_base::allowances::execute_decrease_allowance(deps, _env, info, spender, amount, expires),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => cw20_base::allowances::execute_transfer_from(deps, _env, info, owner, recipient, amount),
        ExecuteMsg::BurnFrom { owner, amount } => cw20_base::allowances::execute_burn_from(deps, _env, info, owner, amount),
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => cw20_base::allowances::execute_send_from(deps, _env, info, owner, contract, amount, msg),
        ExecuteMsg::UpdateMarketing {
            project,
            description,
            marketing,
        } => cw20_base::contract::execute_update_marketing(deps, _env, info, project, description, marketing),
        ExecuteMsg::UploadLogo(logo) => cw20_base::contract::execute_upload_logo(deps, _env, info, logo),
    }?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Inherited from cw20_base
        QueryMsg::Balance { address } => to_binary(&cw20_base::contract::query_balance(deps, address)?),
        QueryMsg::TokenInfo {} => to_binary(&cw20_base::contract::query_token_info(deps)?),
        QueryMsg::Minter {} => to_binary(&cw20_base::contract::query_minter(deps)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&cw20_base::allowances::query_allowance(deps, owner, spender)?)
        }
        QueryMsg::AllAllowances {
            owner,
            start_after,
            limit,
        } => to_binary(&cw20_base::enumerable::query_all_allowances(deps, owner, start_after, limit)?),
        QueryMsg::AllAccounts { start_after, limit } => {
            to_binary(&cw20_base::enumerable::query_all_accounts(deps, start_after, limit)?)
        }
        QueryMsg::MarketingInfo {} => to_binary(&cw20_base::contract::query_marketing_info(deps)?),
        QueryMsg::DownloadLogo {} => to_binary(&cw20_base::contract::query_download_logo(deps)?),
    }
}

#[cfg(test)]
mod tests {
}
