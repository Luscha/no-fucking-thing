
#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockStorage};
    use cosmwasm_std::{coins, from_binary, to_binary, Api, 
        Uint128, StdResult, Coin, StdError, Order};
    use std::borrow::BorrowMut;

    use cw721::Cw721ReceiveMsg;

    use crate::state::{increment_offerings, offerings, Offering};

    use crate::query::query;
    use crate::execute::{execute, instantiate};
    use crate::package::{OfferingsResponse, OfferingResponse};
    use crate::msg::{ExecuteMsg, InstantiateMsg, SellNft, QueryMsg};
    use crate::error::ContractError;

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
        let _res = query(deps.as_ref(), mock_env(), QueryMsg::Offerings {limit: None, start_after: None}).unwrap();
        let value: OfferingsResponse = from_binary(&_res).unwrap();
        assert_eq!(1, value.offerings.len());

        let msg2 = ExecuteMsg::Buy {
            offering_id: value.offerings[0].id.clone(),
            payment: Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(5),
            }
        };

        let info_buy = mock_info("buyer", &[Coin {
            denom: "uluna".to_string(),
            amount: Uint128::from(5u128),
        }]);

        let _res = execute(deps.as_mut(), mock_env(), info_buy, msg2).unwrap();

        // check offerings again. Should be 0
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::Offerings {limit: None, start_after: None}).unwrap();
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
        let _res = query(deps.as_ref(), mock_env(), QueryMsg::Offerings {limit: None, start_after: None}).unwrap();
        let value: OfferingsResponse = from_binary(&_res).unwrap();
        assert_eq!(1, value.offerings.len());

        // withdraw offering
        let withdraw_info = mock_info("seller", &[]);
        let withdraw_msg = ExecuteMsg::WithdrawNft {
            offering_id: value.offerings[0].id.clone(),
        };
        let _res = execute(deps.as_mut(), mock_env(), withdraw_info, withdraw_msg).unwrap();

        // Offering should be removed
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::Offerings {limit: None, start_after: None}).unwrap();
        let value2: OfferingsResponse = from_binary(&res2).unwrap();
        assert_eq!(0, value2.offerings.len());
    }

    #[test]
    fn offerings_list_query() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            name: String::from("test market"),
        };
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("contract_1", &[]);

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

        let info = mock_info("contract_2", &[]);

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

        let info = mock_info("contract_2", &[]);

        let sell_msg = SellNft {
            list_price: Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(5),
            },
        };

        let msg = ExecuteMsg::ReceiveNft(Cw721ReceiveMsg {
            sender: String::from("seller2"),
            token_id: String::from("SellableNFT2"),
            msg: to_binary(&sell_msg).unwrap(),
        });
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // check offerings
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::Offerings {limit: None, start_after: None}).unwrap();
        let value2: OfferingsResponse = from_binary(&res2).unwrap();
        assert_eq!(3, value2.offerings.len());

        // check pagination
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::Offerings {limit: Some(2), start_after: None}).unwrap();
        let value2: OfferingsResponse = from_binary(&res2).unwrap();
        assert_eq!(2, value2.offerings.len());

        // check pagination
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::Offerings {limit: Some(2), start_after: Some("2".to_string())}).unwrap();
        let value2: OfferingsResponse = from_binary(&res2).unwrap();
        assert_eq!(1, value2.offerings.len());

        // check offerings owner
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::OfferingsByOwner {owner: "seller".to_string(), limit: None, start_after: None}).unwrap();
        let value2: OfferingsResponse = from_binary(&res2).unwrap();
        assert_eq!(2, value2.offerings.len());

        // check pagination owner
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::OfferingsByOwner {owner: "seller".to_string(), limit: Some(1), start_after: None}).unwrap();
        let value2: OfferingsResponse = from_binary(&res2).unwrap();
        assert_eq!(1, value2.offerings.len());

        // check pagination owner
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::OfferingsByOwner {owner: "seller".to_string(), limit: Some(2), start_after: Some("2".to_string())}).unwrap();
        let value2: OfferingsResponse = from_binary(&res2).unwrap();
        assert_eq!(0, value2.offerings.len());

        // check offerings contract
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::OfferingsByCollection {collection_contract: "contract_2".to_string(), limit: None, start_after: None}).unwrap();
        let value2: OfferingsResponse = from_binary(&res2).unwrap();
        assert_eq!(2, value2.offerings.len());

        // check pagination contract
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::OfferingsByCollection {collection_contract: "contract_2".to_string(), limit: Some(1), start_after: None}).unwrap();
        let value2: OfferingsResponse = from_binary(&res2).unwrap();
        assert_eq!(1, value2.offerings.len());

        // check pagination contract
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::OfferingsByCollection {collection_contract: "contract_2".to_string(), limit: Some(2), start_after: Some("2".to_string())}).unwrap();
        let value2: OfferingsResponse = from_binary(&res2).unwrap();
        assert_eq!(1, value2.offerings.len());

        // check search
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::OfferingByNft {collection_contract: "contract_2".to_string(), token_id: "SellableNFT2".to_string()}).unwrap();
        let value2: OfferingResponse = from_binary(&res2).unwrap();
        assert_eq!("SellableNFT2", value2.offering.token_id);

        // check search fail
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::OfferingByNft {collection_contract: "contract_3".to_string(), token_id: "SellableNFT2".to_string()}).unwrap_err();
        match res2 {
            StdError::NotFound { .. } => (),
            _ => panic!("Must return not_found error"),
        }
    }

    #[test]
    fn store_multiple_offerings_one_owner() {
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

        let mut id = increment_offerings(store.borrow_mut()).unwrap().to_string();

        let off = Offering {
            contract_addr: deps.api.addr_canonicalize(contract.as_str()).unwrap(),
            token_id: token1.clone(),
            seller: seller_addr1.clone(),
            list_price: price.clone(),
        };
    
        offerings().save(store.borrow_mut(), &id, &off).unwrap();

        let list: Vec<_> = offerings()
            .idx.seller
            .prefix(seller_addr1.as_slice().to_vec())
            .range(&store, None, None, Order::Ascending)
            .collect::<StdResult<_>>().unwrap();

        assert_eq!(1, list.len());

        id = increment_offerings(store.borrow_mut()).unwrap().to_string();
        let off = Offering {
            contract_addr: deps.api.addr_canonicalize(contract.as_str()).unwrap(),
            token_id: token2.clone(),
            seller: seller_addr1.clone(),
            list_price: price.clone(),
        };
    
        offerings().save(store.borrow_mut(), &id, &off).unwrap();

        let list: Vec<_> = offerings()
            .idx.seller
            .prefix(seller_addr1.as_slice().to_vec())
            .range(&store, None, None, Order::Ascending)
            .collect::<StdResult<_>>().unwrap();

        assert_eq!(2, list.len());

        id = increment_offerings(store.borrow_mut()).unwrap().to_string();
        let off = Offering {
            contract_addr: deps.api.addr_canonicalize(contract.as_str()).unwrap(),
            token_id: token3.clone(),
            seller: seller_addr2.clone(),
            list_price: price.clone(),
        };
    
        offerings().save(store.borrow_mut(), &id, &off).unwrap();

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
    fn store_multiple_offerings_one_contract() {
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

        let mut id = increment_offerings(store.borrow_mut()).unwrap().to_string();

        let off = Offering {
            contract_addr: deps.api.addr_canonicalize(contract.as_str()).unwrap(),
            token_id: token1.clone(),
            seller: seller_addr1.clone(),
            list_price: price.clone(),
        };
    
        offerings().save(store.borrow_mut(), &id, &off).unwrap();

        let list: Vec<_> = offerings()
            .idx.contract
            .prefix(deps.api.addr_canonicalize(contract.as_str()).unwrap().as_slice().to_vec())
            .range(&store, None, None, Order::Ascending)
            .collect::<StdResult<_>>().unwrap();

        assert_eq!(1, list.len());

        id = increment_offerings(store.borrow_mut()).unwrap().to_string();
        let off = Offering {
            contract_addr: deps.api.addr_canonicalize(contract.as_str()).unwrap(),
            token_id: token2.clone(),
            seller: seller_addr1.clone(),
            list_price: price.clone(),
        };
    
        offerings().save(store.borrow_mut(), &id, &off).unwrap();

        let list: Vec<_> = offerings()
            .idx.contract
            .prefix(deps.api.addr_canonicalize(contract.as_str()).unwrap().as_slice().to_vec())
            .range(&store, None, None, Order::Ascending)
            .collect::<StdResult<_>>().unwrap();

        assert_eq!(2, list.len());

        id = increment_offerings(store.borrow_mut()).unwrap().to_string();
        let off = Offering {
            contract_addr: deps.api.addr_canonicalize(contract.as_str()).unwrap(),
            token_id: token3.clone(),
            seller: seller_addr2.clone(),
            list_price: price.clone(),
        };
    
        offerings().save(store.borrow_mut(), &id, &off).unwrap();

        let list: Vec<_> = offerings()
            .idx.contract
            .prefix(deps.api.addr_canonicalize(contract.as_str()).unwrap().as_slice().to_vec())
            .range(&store, None, None, Order::Ascending)
            .collect::<StdResult<_>>().unwrap();

        assert_eq!(3, list.len());

        let list: Vec<_> = offerings()
            .idx.contract
            .prefix(deps.api.addr_canonicalize(contract.as_str()).unwrap().as_slice().to_vec())
            .range(&store, None, None, Order::Ascending)
            .filter(|o| {
                o.as_ref().unwrap().1.token_id == token1
            })
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
        let _res = query(deps.as_ref(), mock_env(), QueryMsg::Offerings {limit: None, start_after: None}).unwrap();
        let value: OfferingsResponse = from_binary(&_res).unwrap();
        assert_eq!(1, value.offerings.len());

        let msg2 = ExecuteMsg::Buy {
            offering_id: value.offerings[0].id.clone(),
            payment: Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(5),
            }
        };

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
        let info_buy = mock_info("buyer", &coins(1u128, "uusd"));

        let _res = execute(deps.as_mut(), mock_env(), info_buy, msg2.clone()).unwrap_err();
        match _res {
            ContractError::Std(StdError::GenericErr { msg, .. }) => assert_eq!(
                msg,
                "Native token denom mismatch between the argument and the transferred".to_string()
            ),
            _ => panic!("Must return generic error"),
        }

        // not enough funds
        let msg2 = ExecuteMsg::Buy {
            offering_id: value.offerings[0].id.clone(),
            payment: Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(3),
            }
        };
        let info_buy = mock_info("buyer", &coins(3u128, "uluna"));
        let _res = execute(deps.as_mut(), mock_env(), info_buy, msg2.clone()).unwrap_err();
        match _res {
            ContractError::InsufficientFunds{} => (),
            _ => panic!("Must return InsufficientFunds error"),
        }

        // wrong denom
        let msg2 = ExecuteMsg::Buy {
            offering_id: value.offerings[0].id.clone(),
            payment: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::new(5),
            }
        };
        let info_buy = mock_info("buyer", &coins(5u128, "uusd"));
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
                denom: "uust".to_string(),
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
        let _res = query(deps.as_ref(), mock_env(), QueryMsg::Offerings {limit: None, start_after: None}).unwrap();
        let value: OfferingsResponse = from_binary(&_res).unwrap();
        assert_eq!(0, value.offerings.len());
    }
}
