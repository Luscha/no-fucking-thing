#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::package::{ContractInfoResponse, OfferingsResponse, QueryOfferingsResult};
use crate::state::{increment_offerings, Offering, CONTRACT_INFO, offerings, offering_key};
use cosmwasm_std::Pair;
use cosmwasm_std::{
    from_binary, to_binary, Api, Binary, CosmosMsg, Env, DepsMut, Deps,
    Response, MessageInfo, Order, StdResult, WasmMsg, Coin, StdError
};
use cw721::{Cw721ExecuteMsg, Cw721ReceiveMsg};
use cw_asset::{AssetUnchecked, Asset};
use std::str::from_utf8;

use crate::error::ContractError;
use crate::msg::{BuyNft, ExecuteMsg, InstantiateMsg, QueryMsg, SellNft};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let info = ContractInfoResponse { name: msg.name };
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
        ExecuteMsg::Buy { msg } => try_buy(deps, info, msg),
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
    msg: BuyNft,
) -> Result<Response, ContractError> {
    assert_sent_native_token_balance(&msg.payment, &info)?;

    // check if offering exists
    let off = offerings().load(deps.storage, &msg.offering_id)?;

    // check for enough coins
    if msg.payment.amount < off.list_price.amount {
        return Err(ContractError::InsufficientFunds {});
    }

    if msg.payment.denom != off.list_price.denom {
        return Err(ContractError::WrongCoin {});
    }

    // create transfer coin msg
    let asset1 = Asset::native(msg.payment.denom.clone(), msg.payment.amount.clone());
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
    offerings().remove(deps.storage, &msg.offering_id).unwrap();

    let price_string = format!("{} {}", msg.payment.denom, msg.payment.amount);

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

const ACCEPTED_DENOMS: &[&str] = &["uust", "uluna"];

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
        key: offering_key(&info.sender.to_string(), &rcv_msg.token_id),
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
        .add_attribute("key", off.key)
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOfferings {} => to_binary(&query_offerings(deps)?),
    }
}

// ============================== Query Executers ==============================

fn query_offerings(
    deps: Deps,
) -> StdResult<OfferingsResponse> {
    let res: StdResult<Vec<QueryOfferingsResult>> = offerings()
        .range(deps.storage, None, None, Order::Ascending)
        .map(|kv_item| parse_offering(deps.api, kv_item))
        .collect();

    Ok(OfferingsResponse {
        offerings: res?, // Placeholder
    })
}

fn parse_offering(
    api: &dyn Api,
    item: StdResult<Pair<Offering>>,
) -> StdResult<QueryOfferingsResult> {
    item.and_then(|(k, offering)| {
        let id = from_utf8(&k)?;
        Ok(QueryOfferingsResult {
            id: id.to_string(),
            token_id: offering.token_id,
            list_price: offering.list_price,
            contract_addr: api.addr_humanize(&offering.contract_addr)?.into_string(),
            seller: api.addr_humanize(&offering.seller)?.into_string(),
        })
    })
}

// ============================== Test ==============================

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockStorage};
    use cosmwasm_std::{coins, from_binary, Uint128};
    use std::borrow::BorrowMut;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            name: String::from("test market"),
        };
        let info = mock_info("creator", &[]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn sell_offering_happy_path() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            name: String::from("test market"),
        };
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &[]);

        let sell_msg = SellNft {
            list_price: Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(5),
            },
        };

        let msg = ExecuteMsg::ReceiveNft(Cw721ReceiveMsg {
            sender: String::from("seller"),
            token_id: String::from("SellableNFT"),
            msg: to_binary(&sell_msg).unwrap(),
        });
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Offering should be listed
        let _res = query(deps.as_ref(), mock_env(), QueryMsg::GetOfferings {}).unwrap();
        let value: OfferingsResponse = from_binary(&_res).unwrap();
        assert_eq!(1, value.offerings.len());

        let msg2 = ExecuteMsg::Buy {msg: BuyNft {
            offering_id: value.offerings[0].id.clone(),
            payment: Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(5),
            }
        }};

        let info_buy = mock_info("buyer", &[Coin {
            denom: "uluna".to_string(),
            amount: Uint128::from(5u128),
        }]);

        let _res = execute(deps.as_mut(), mock_env(), info_buy, msg2).unwrap();

        // check offerings again. Should be 0
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::GetOfferings {}).unwrap();
        let value2: OfferingsResponse = from_binary(&res2).unwrap();
        assert_eq!(0, value2.offerings.len());
    }

    #[test]
    fn withdraw_offering_happy_path() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            name: String::from("test market"),
        };
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &[]);

        let sell_msg = SellNft {
            list_price: Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(5),
            },
        };

        let msg = ExecuteMsg::ReceiveNft(Cw721ReceiveMsg {
            sender: String::from("seller"),
            token_id: String::from("SellableNFT"),
            msg: to_binary(&sell_msg).unwrap(),
        });
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Offering should be listed
        let _res = query(deps.as_ref(), mock_env(), QueryMsg::GetOfferings {}).unwrap();
        let value: OfferingsResponse = from_binary(&_res).unwrap();
        assert_eq!(1, value.offerings.len());

        // withdraw offering
        let withdraw_info = mock_info("seller", &[]);
        let withdraw_msg = ExecuteMsg::WithdrawNft {
            offering_id: value.offerings[0].id.clone(),
        };
        let _res = execute(deps.as_mut(), mock_env(), withdraw_info, withdraw_msg).unwrap();

        // Offering should be removed
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::GetOfferings {}).unwrap();
        let value2: OfferingsResponse = from_binary(&res2).unwrap();
        assert_eq!(0, value2.offerings.len());
    }

    #[test]
    fn multiple_offerings_one_owner() {
        let mut store = MockStorage::new();
        let deps = mock_dependencies(&[]);

        let token1 = "TOKEN1".to_string();
        let token2 = "TOKEN2".to_string();
        let token3 = "TOKEN3".to_string();
        let contract = "addr".to_string();
        let seller1 = "seller1".to_string();
        let seller_addr1 = deps.api.addr_canonicalize(seller1.as_str()).unwrap();
        let seller2 = "seller2".to_string();
        let seller_addr2 = deps.api.addr_canonicalize(seller2.as_str()).unwrap();
        let price: Coin = Coin {
            denom: "uluna".to_string(),
            amount: Uint128::new(5),
        };

        let off = Offering {
            key: offering_key(&contract, &token1),
            contract_addr: deps.api.addr_canonicalize(contract.as_str()).unwrap(),
            token_id: token1.clone(),
            seller: seller_addr1.clone(),
            list_price: price.clone(),
        };
    
        offerings().save(store.borrow_mut(), &off.key, &off).unwrap();

        let list: Vec<_> = offerings()
            .idx.seller
            .prefix(seller_addr1.as_slice().to_vec())
            .range(&store, None, None, Order::Ascending)
            .collect::<StdResult<_>>().unwrap();

        assert_eq!(1, list.len());

        let off = Offering {
            key: offering_key(&contract, &token2),
            contract_addr: deps.api.addr_canonicalize(contract.as_str()).unwrap(),
            token_id: token2.clone(),
            seller: seller_addr1.clone(),
            list_price: price.clone(),
        };
    
        offerings().save(store.borrow_mut(), &token2, &off).unwrap();

        let list: Vec<_> = offerings()
            .idx.seller
            .prefix(seller_addr1.as_slice().to_vec())
            .range(&store, None, None, Order::Ascending)
            .collect::<StdResult<_>>().unwrap();

        assert_eq!(2, list.len());

        let off = Offering {
            key: offering_key(&contract, &token3),
            contract_addr: deps.api.addr_canonicalize(contract.as_str()).unwrap(),
            token_id: token3.clone(),
            seller: seller_addr2.clone(),
            list_price: price.clone(),
        };
    
        offerings().save(store.borrow_mut(), &token3, &off).unwrap();

        let list: Vec<_> = offerings()
            .idx.seller
            .prefix(seller_addr1.as_slice().to_vec())
            .range(&store, None, None, Order::Ascending)
            .collect::<StdResult<_>>().unwrap();

        assert_eq!(2, list.len());

        let list: Vec<_> = offerings()
            .idx.seller
            .prefix(seller_addr2.as_slice().to_vec())
            .range(&store, None, None, Order::Ascending)
            .collect::<StdResult<_>>().unwrap();

        assert_eq!(1, list.len());
    }

    #[test]
    fn wrong_provided_buy_funds() {
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            name: String::from("test market"),
        };
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &[]);

        let sell_msg = SellNft {
            list_price: Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(5),
            },
        };

        let msg = ExecuteMsg::ReceiveNft(Cw721ReceiveMsg {
            sender: String::from("seller"),
            token_id: String::from("SellableNFT"),
            msg: to_binary(&sell_msg).unwrap(),
        });
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Offering should be listed
        let _res = query(deps.as_ref(), mock_env(), QueryMsg::GetOfferings {}).unwrap();
        let value: OfferingsResponse = from_binary(&_res).unwrap();
        assert_eq!(1, value.offerings.len());

        let msg2 = ExecuteMsg::Buy {msg: BuyNft {
            offering_id: value.offerings[0].id.clone(),
            payment: Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(5),
            }
        }};

        // msg, funds amount missmatch
        let info_buy = mock_info("buyer", &coins(1u128, "uluna"));

        let _res = execute(deps.as_mut(), mock_env(), info_buy, msg2.clone()).unwrap_err();
        match _res {
            ContractError::Std(StdError::GenericErr { msg, .. }) => assert_eq!(
                msg,
                "Native token balance mismatch between the argument and the transferred".to_string()
            ),
            _ => panic!("Must return generic error"),
        }

        // msg, funds denom missmatch
        let info_buy = mock_info("buyer", &coins(1u128, "uust"));

        let _res = execute(deps.as_mut(), mock_env(), info_buy, msg2.clone()).unwrap_err();
        match _res {
            ContractError::Std(StdError::GenericErr { msg, .. }) => assert_eq!(
                msg,
                "Native token denom mismatch between the argument and the transferred".to_string()
            ),
            _ => panic!("Must return generic error"),
        }

        // not enough funds
        let msg2 = ExecuteMsg::Buy {msg: BuyNft {
            offering_id: value.offerings[0].id.clone(),
            payment: Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(3),
            }
        }};
        let info_buy = mock_info("buyer", &coins(3u128, "uluna"));
        let _res = execute(deps.as_mut(), mock_env(), info_buy, msg2.clone()).unwrap_err();
        match _res {
            ContractError::InsufficientFunds{} => (),
            _ => panic!("Must return InsufficientFunds error"),
        }

        // wrong denom
        let msg2 = ExecuteMsg::Buy {msg: BuyNft {
            offering_id: value.offerings[0].id.clone(),
            payment: Coin {
                denom: "uust".to_string(),
                amount: Uint128::new(5),
            }
        }};
        let info_buy = mock_info("buyer", &coins(5u128, "uust"));
        let _res = execute(deps.as_mut(), mock_env(), info_buy, msg2.clone()).unwrap_err();
        match _res {
            ContractError::WrongCoin{} => (),
            _ => panic!("Must return WrongCoin error"),
        }
    }
    
    #[test]
    fn unsupported_nft_offeting_denom() {
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {
            name: String::from("test market"),
        };
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &[]);

        let sell_msg = SellNft {
            list_price: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::new(5),
            },
        };

        let msg = ExecuteMsg::ReceiveNft(Cw721ReceiveMsg {
            sender: String::from("seller"),
            token_id: String::from("SellableNFT"),
            msg: to_binary(&sell_msg).unwrap(),
        });

        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        match _res {
            ContractError::WrongCoin{} => (),
            _ => panic!("Must return WrongCoin error"),
        }

        // Offering should not be listed
        let _res = query(deps.as_ref(), mock_env(), QueryMsg::GetOfferings {}).unwrap();
        let value: OfferingsResponse = from_binary(&_res).unwrap();
        assert_eq!(0, value.offerings.len());
    }
}
