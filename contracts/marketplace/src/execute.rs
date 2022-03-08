#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::package::{ContractInfoResponse};
use crate::state::{increment_offerings, Offering, CONTRACT_INFO, offerings};
use cosmwasm_std::{
    from_binary, to_binary, Api, CosmosMsg, Env, DepsMut,
    Response, MessageInfo, StdResult, WasmMsg, Coin, StdError
};
use cw721::{Cw721ExecuteMsg, Cw721ReceiveMsg};
use cw_asset::{AssetUnchecked, Asset};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, SellNft};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let info = ContractInfoResponse { 
        name: msg.name,
    };
    CONTRACT_INFO.save(deps.storage, &info)?;
    Ok(Response::default())
}

// And declare a custom Error variant for the ones where you will want to make use of it
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::WithdrawNft { offering_id } => try_withdraw(deps, info, offering_id),
        ExecuteMsg::Buy { offering_id, payment } => try_buy(deps, info, offering_id, payment),
        ExecuteMsg::ReceiveNft(msg) => try_receive_nft(deps, info, msg),
    }
}

pub fn assert_sent_native_token_balance(payment: &Coin, message_info: &MessageInfo) -> StdResult<()> {
    match message_info.funds.iter().find(|x| x.denom == *payment.denom) {
        Some(coin) => {
            if payment.amount == coin.amount {
                Ok(())
            } else {
                Err(StdError::generic_err("Native token balance mismatch between the argument and the transferred"))
            }
        }
        None => Err(StdError::generic_err("Native token denom mismatch between the argument and the transferred"))
    }
}

// ============================== Message Executers ==============================

pub fn try_buy(
    deps: DepsMut,
    info: MessageInfo,
    offering_id: String, 
    payment: Coin,
) -> Result<Response, ContractError> {
    assert_sent_native_token_balance(&payment, &info)?;

    // check if offering exists
    let off = offerings().load(deps.storage, &offering_id)?;

    // check for enough coins
    if payment.amount < off.list_price.amount {
        return Err(ContractError::InsufficientFunds {});
    }

    if payment.denom != off.list_price.denom {
        return Err(ContractError::WrongCoin {});
    }

    // create transfer coin msg
    let asset1 = Asset::native(payment.denom.clone(), payment.amount.clone());
    let coin_transfer_cosmos_msg = asset1.transfer_msg(deps.api.addr_humanize(&off.seller)?.into_string())?;

    // create transfer cw721 msg
    let transfer_cw721_msg = Cw721ExecuteMsg::TransferNft {
        recipient: info.sender.to_string(),
        token_id: off.token_id.clone(),
    };
    let exec_cw721_transfer = WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&off.contract_addr)?.into_string(),
        msg: to_binary(&transfer_cw721_msg)?,
        funds: vec![],
    };

    // if everything is fine transfer cw20 to seller
    // transfer nft to buyer
    let cw721_transfer_cosmos_msg: CosmosMsg = exec_cw721_transfer.into();

    //delete offering
    offerings().remove(deps.storage, &offering_id).unwrap();

    let price_string = format!("{} {}", payment.denom, payment.amount);

    Ok(Response::new()
        .add_attribute("action", "buy_nft")
        .add_attribute("buyer", info.sender.to_string())
        .add_attribute("seller", off.seller.to_string())
        .add_attribute("paid_price", price_string)
        .add_attribute("token_id", off.token_id)
        .add_attribute("contract_addr", off.contract_addr.to_string())
        .add_message(coin_transfer_cosmos_msg)
        .add_message(cw721_transfer_cosmos_msg)
    )
}

const ACCEPTED_DENOMS: &[&str] = &["uusd", "uluna"];

fn validate_deposit(api: &dyn Api, asset_unchecked: AssetUnchecked) -> StdResult<()> {
    asset_unchecked.check(api, Some(ACCEPTED_DENOMS))?;
    Ok(())
}

pub fn try_receive_nft(
    deps: DepsMut,
    info: MessageInfo,
    rcv_msg: Cw721ReceiveMsg,
) -> Result<Response, ContractError> {
    let msg: SellNft = from_binary(&rcv_msg.msg)?;

    // validate denom
    let asset_unchecked = AssetUnchecked::from(Asset::native(msg.list_price.denom.clone(), msg.list_price.amount.clone()));
    match validate_deposit(deps.api, asset_unchecked) {
        Err(_) => return Err(ContractError::WrongCoin{}),
        _ => ()
    }

    // check if same token Id form same original contract is already on sale
    // get OFFERING_COUNT
    let id = increment_offerings(deps.storage)?.to_string();

    // save Offering
    let off = Offering {
        contract_addr: deps.api.addr_canonicalize(&info.sender.as_str())?,
        token_id: rcv_msg.token_id,
        seller: deps.api.addr_canonicalize(&deps.api.addr_validate(&rcv_msg.sender)?.as_str())?,
        list_price: msg.list_price.clone(),
    };

    offerings().save(deps.storage, &id, &off)?;

    let price_string = format!("{} {}", msg.list_price.amount, msg.list_price.denom);

    Ok(Response::new()
        .add_attribute("action", "sell_nft")
        .add_attribute("original_contract", info.sender.to_string())
        .add_attribute("seller", deps.api.addr_humanize(&off.seller)?.as_str())
        .add_attribute("list_price", price_string)
        .add_attribute("token_id", off.token_id)
        .add_attribute("offer_id", id)
    )
}

pub fn try_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    offering_id: String,
) -> Result<Response, ContractError> {
    // check if token_id is currently sold by the requesting address
    let off = offerings().load(deps.storage, &offering_id)?;
    if deps.api.addr_humanize(&off.seller)? == info.sender {
        // transfer token back to original owner
        let transfer_cw721_msg = Cw721ExecuteMsg::TransferNft {
            recipient: deps.api.addr_humanize(&off.seller)?.into_string(),
            token_id: off.token_id.clone(),
        };

        let exec_cw721_transfer = WasmMsg::Execute {
            contract_addr: deps.api.addr_humanize(&off.contract_addr)?.into_string(),
            msg: to_binary(&transfer_cw721_msg)?,
            funds: vec![],
        };

        // remove offering
        offerings().remove(deps.storage, &offering_id).unwrap();

        return Ok(Response::new()
            .add_attribute("action", "withdraw_nft")
            .add_attribute("seller", info.sender.to_string())
            .add_attribute("offering_id", offering_id)
            .add_message(exec_cw721_transfer)
        );
    }
    Err(ContractError::Unauthorized {})
}
