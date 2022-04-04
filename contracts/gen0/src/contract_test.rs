#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{
        from_binary, CosmosMsg, WasmMsg, DepsMut, Response,
        to_binary, StdResult, Coin, Uint128, coins, StdError
    };
    use crate::query::{query};
    use crate::contract::{execute, instantiate};

    use crate::msg::{
        ExecuteMsg, InstantiateMsg, MintMsg, QueryMsg, 
        ContractInfoResponse, NftInfoResponse,
    };

    use cw721::{
        ApprovedForAllResponse, Expiration, OwnerOfResponse,
        Cw721ReceiveMsg, NumTokensResponse, TokensResponse
    };
    use crate::error::ContractError;

    const CREATOR: &str = "creator";
    const MINTER: &str = "merlin";
    const TRASURY: &str = "treasury";
    const PRICE_DENOM: &str = "uusd";
    const PRICE_AMOUNT: u64 = 100000;
    const CONTRACT_NAME: &str = "Magic Power";
    const SYMBOL: &str = "MGK";
    const TOKEN_URI: &str = "ipfs://test";


    fn setup_contract(deps: DepsMut) {
        let msg = InstantiateMsg {
            name: CONTRACT_NAME.to_string(),
            treasury: TRASURY.to_string(),
            minting_price_denom: PRICE_DENOM.to_string(),
            minting_price_amount: PRICE_AMOUNT,
            symbol: SYMBOL.to_string(),
            max_tokens: 100,
            token_uri: TOKEN_URI.to_string(),
        };
        let info = mock_info(CREATOR, &[]);
        let res = instantiate(deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    fn get_token_id_from_response(resp: &Response) -> StdResult<String> {
        Ok(resp
            .attributes
            .iter()
            .find(|a| a.key == "token_id")
            .unwrap()
            .value
            .clone())
    }

    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            name: CONTRACT_NAME.to_string(),
            treasury: TRASURY.to_string(),
            minting_price_denom: PRICE_DENOM.to_string(),
            minting_price_amount: PRICE_AMOUNT,
            symbol: SYMBOL.to_string(),
            max_tokens: 100,
            token_uri: TOKEN_URI.to_string(),
        };
        let info = mock_info(MINTER, &[]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res: ContractInfoResponse = 
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::ContractInfo{}).unwrap()).unwrap();
        assert_eq!(
            res,
            ContractInfoResponse {
                minter: MINTER.to_string(),
                treasury: TRASURY.to_string(),
                minting_price_denom: PRICE_DENOM.to_string(),
                minting_price_amount: PRICE_AMOUNT,
                name: CONTRACT_NAME.to_string(),
                symbol: SYMBOL.to_string(),
                max_tokens: 100,
                token_uri: TOKEN_URI.to_string(),
            }
        );

        let count: NumTokensResponse = 
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::NumTokens{}).unwrap()).unwrap();
        assert_eq!(0, count.count);

        // list the token_ids
        let tokens: TokensResponse = 
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::AllTokens{
                start_after: None, 
                limit: None}).unwrap()).unwrap();
        assert_eq!(0, tokens.tokens.len());
    }

    #[test]
    fn sold_out() {
        let payment: Coin = Coin {
            denom: PRICE_DENOM.to_string(),
            amount: Uint128::new(PRICE_AMOUNT.into()),
        };

        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            name: CONTRACT_NAME.to_string(),
            treasury: TRASURY.to_string(),
            minting_price_denom: PRICE_DENOM.to_string(),
            minting_price_amount: PRICE_AMOUNT,
            symbol: SYMBOL.to_string(),
            max_tokens: 1,
            token_uri: TOKEN_URI.to_string(),
        };
        let info = mock_info(MINTER, &[payment.clone()]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(MINTER),
            payment: Some(payment.clone()),
        });

        // first ok
        execute(deps.as_mut(), mock_env(), mock_info(MINTER, &[payment.clone()]), mint_msg.clone()).unwrap();

        // second sold out
        let err = execute(deps.as_mut(), mock_env(), mock_info(MINTER, &[payment.clone()]), mint_msg.clone()).unwrap_err();
        assert_eq!(err, ContractError::SoldOut {});
    }

    #[test]
    fn minting_fail() {
        let payment: Coin = Coin {
            denom: PRICE_DENOM.to_string(),
            amount: Uint128::new(PRICE_AMOUNT.into()),
        };

        let mut deps = mock_dependencies(&[]);
        setup_contract(deps.as_mut());

        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(MINTER),
            payment: Some(Coin {
                denom: "uusd".to_string(),
                amount: Uint128::new(100),
            }),
        });
        
        // msg, funds amount missmatch
        let medusa = mock_info(MINTER, &[payment.clone()]);
        let _res = execute(deps.as_mut(), mock_env(), medusa.clone(), mint_msg.clone()).unwrap_err();
        match _res {
            ContractError::Std(StdError::GenericErr { msg, .. }) => assert_eq!(
                msg,
                "Native token balance mismatch between the argument and the transferred".to_string()
            ),
            _ => panic!("Must return generic error"),
        }

        // msg, funds denom missmatch
        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(MINTER),
            payment: Some(Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(PRICE_AMOUNT.into()),
            }),
        });

        let _res = execute(deps.as_mut(), mock_env(), medusa.clone(), mint_msg.clone()).unwrap_err();
        match _res {
            ContractError::Std(StdError::GenericErr { msg, .. }) => assert_eq!(
                msg,
                "Native token denom mismatch between the argument and the transferred".to_string()
            ),
            _ => panic!("Must return generic error"),
        }

        // not enough funds
        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(MINTER),
            payment: Some(Coin {
                denom: PRICE_DENOM.to_string(),
                amount: Uint128::new(100),
            }),
        });
        let medusa = mock_info("buyer", &coins(100u128, PRICE_DENOM));
        let _res = execute(deps.as_mut(), mock_env(), medusa.clone(), mint_msg.clone()).unwrap_err();
        match _res {
            ContractError::InsufficientFunds{} => (),
            _ => panic!("Must return InsufficientFunds error"),
        }

        // wrong denom
        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(MINTER),
            payment: Some(Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(PRICE_AMOUNT.into()),
            }),
        });
        let medusa = mock_info("buyer", &coins(PRICE_AMOUNT.into(), "uluna"));
        let _res = execute(deps.as_mut(), mock_env(), medusa.clone(), mint_msg.clone()).unwrap_err();
        match _res {
            ContractError::WrongCoin{} => (),
            _ => panic!("Must return WrongCoin error"),
        }

        // No payment
        let medusa = mock_info(MINTER, &[]);
        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(MINTER),
            payment: None,
        });
        execute(deps.as_mut(), mock_env(), medusa.clone(), mint_msg.clone()).unwrap_err();

        // ensure num tokens increases
        let count: NumTokensResponse = 
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::NumTokens{}).unwrap()).unwrap();
        assert_eq!(0, count.count);
    }
    
    #[test]
    fn minting() {
        let payment: Coin = Coin {
            denom: PRICE_DENOM.to_string(),
            amount: Uint128::new(PRICE_AMOUNT.into()),
        };

        let mut deps = mock_dependencies(&[]);
        setup_contract(deps.as_mut());

        let token_uri = "ipfs://test/1.json".to_string();
        let minter = "medusa";

        // mint success
        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(minter),
            payment: Some(payment.clone()),
        });
        let medusa = mock_info(minter, &[payment.clone()]);
        let resp: Response = execute(deps.as_mut(), mock_env(), medusa, mint_msg).unwrap();
        let token_id = get_token_id_from_response(&resp).unwrap();
        assert_eq!("1", token_id);

        // ensure num tokens increases
        let count: NumTokensResponse = 
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::NumTokens{}).unwrap()).unwrap();
        assert_eq!(1, count.count);

        // unknown nft returns error
        query(deps.as_ref(), mock_env(), QueryMsg::NftInfo { token_id: "unknown".to_string() }).unwrap_err();

        // this nft info is correct
        let info: NftInfoResponse = 
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::NftInfo { token_id: token_id.clone() }).unwrap()).unwrap();
        assert_eq!(
            info,
            NftInfoResponse {
                token_uri: Some(token_uri),
            }
        );

        // owner info is correct
        let owner: OwnerOfResponse = 
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::OwnerOf {
                token_id: token_id.clone(),
                include_expired: Some(true),
            }).unwrap()).unwrap();
        assert_eq!(
            owner,
            OwnerOfResponse {
                owner: String::from(minter),
                approvals: vec![],
            }
        );

        // list the token_ids
        let tokens: TokensResponse = 
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::AllTokens{
                start_after: None, 
                limit: None}).unwrap()).unwrap();
        assert_eq!(1, tokens.tokens.len());
        assert_eq!(vec![token_id], tokens.tokens);
    }

    #[test]
    fn minting_whitelist() {
        let mut deps = mock_dependencies(&[]);
        setup_contract(deps.as_mut());

        let minter = CREATOR;

        // mint success
        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(minter),
            payment: None,
        });
        
        let medusa = mock_info(minter, &[]);
        let resp: Response = execute(deps.as_mut(), mock_env(), medusa, mint_msg).unwrap();
        let token_id = get_token_id_from_response(&resp).unwrap();
        assert_eq!("1", token_id);

        // ensure num tokens increases
        let count: NumTokensResponse = 
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::NumTokens{}).unwrap()).unwrap();
        assert_eq!(1, count.count);
    }

    #[test]
    fn transferring_nft() {
        let payment: Coin = Coin {
            denom: PRICE_DENOM.to_string(),
            amount: Uint128::new(PRICE_AMOUNT.into()),
        };

        let mut deps = mock_dependencies(&[]);
        setup_contract(deps.as_mut());

        // Mint a token
        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from("venus"),
            payment: Some(payment.clone()),
        });

        let minter = mock_info("venus", &[payment.clone()]);
        let resp: Response = execute(deps.as_mut(), mock_env(), minter, mint_msg).unwrap();
        let token_id: String = get_token_id_from_response(&resp).unwrap();

        // random cannot transfer
        let random = mock_info("random", &[payment.clone()]);
        let transfer_msg = ExecuteMsg::TransferNft {
            recipient: String::from("random"),
            token_id: token_id.clone(),
        };

        let err = execute(deps.as_mut(), mock_env(), random, transfer_msg).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        // owner can
        let random = mock_info("venus", &[payment.clone()]);
        let transfer_msg = ExecuteMsg::TransferNft {
            recipient: String::from("random"),
            token_id: token_id.clone(),
        };

        let res = execute(deps.as_mut(), mock_env(), random, transfer_msg).unwrap();

        assert_eq!(
            res,
            Response::new()
                .add_attribute("action", "transfer_nft")
                .add_attribute("sender", "venus")
                .add_attribute("recipient", "random")
                .add_attribute("token_id", token_id)
        );
    }

    #[test]
    fn sending_nft() {
        let payment: Coin = Coin {
            denom: PRICE_DENOM.to_string(),
            amount: Uint128::new(PRICE_AMOUNT.into()),
        };

        let mut deps = mock_dependencies(&[]);
        setup_contract(deps.as_mut());

        // Mint a token
        let minter_add = "venus";
        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(minter_add),
            payment: Some(payment.clone()),
        });

        let minter = mock_info(minter_add, &[payment.clone()]);
        let resp = execute(deps.as_mut(), mock_env(), minter, mint_msg).unwrap();
        let token_id = get_token_id_from_response(&resp).unwrap();

        // random can't send
        let msg = to_binary("You now have the melting power").unwrap();
        let target = String::from("another_contract");
        let send_msg = ExecuteMsg::SendNft {
            contract: target.clone(),
            token_id: token_id.clone(),
            msg: msg.clone(),
        };

        let random = mock_info("random", &[payment.clone()]);
        let err = execute(deps.as_mut(), mock_env(), random, send_msg.clone()).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        // but owner can
        let random = mock_info(minter_add, &[payment.clone()]);
        let res = execute(deps.as_mut(), mock_env(), random, send_msg).unwrap();

        let payload = Cw721ReceiveMsg {
            sender: String::from(minter_add),
            token_id: token_id.clone(),
            msg,
        };
        let expected = payload.into_cosmos_msg(target.clone()).unwrap();
        // ensure expected serializes as we think it should
        match &expected {
            CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, .. }) => {
                assert_eq!(contract_addr, &target)
            }
            m => panic!("Unexpected message type: {:?}", m),
        }
        // and make sure this is the request sent by the contract
        assert_eq!(
            res,
            Response::new()
                .add_message(expected)
                .add_attribute("action", "send_nft")
                .add_attribute("sender", minter_add)
                .add_attribute("recipient", "another_contract")
                .add_attribute("token_id", token_id)
        );
    }

    #[test]
    fn approving_revoking() {
        let payment: Coin = Coin {
            denom: PRICE_DENOM.to_string(),
            amount: Uint128::new(PRICE_AMOUNT.into()),
        };

        let mut deps = mock_dependencies(&[]);
        setup_contract(deps.as_mut());

        // Mint a token
        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from("demeter"),
            payment: Some(payment.clone()),
        });

        let minter = mock_info("demeter", &[payment.clone()]);
        let resp = execute(deps.as_mut(), mock_env(), minter, mint_msg).unwrap();
        let token_id = get_token_id_from_response(&resp).unwrap();

        // Give random transferring power
        let approve_msg = ExecuteMsg::Approve {
            spender: String::from("random"),
            token_id: token_id.clone(),
            expires: None,
        };
        let owner = mock_info("demeter", &[payment.clone()]);
        let res = execute(deps.as_mut(), mock_env(), owner, approve_msg).unwrap();
        assert_eq!(
            res,
            Response::new()
                .add_attribute("action", "approve")
                .add_attribute("sender", "demeter")
                .add_attribute("spender", "random")
                .add_attribute("token_id", token_id.clone())
        );

        // random can now transfer
        let random = mock_info("random", &[payment.clone()]);
        let transfer_msg = ExecuteMsg::TransferNft {
            recipient: String::from("person"),
            token_id: token_id.clone(),
        };
        execute(deps.as_mut(), mock_env(), random, transfer_msg).unwrap();

        // Approvals are removed / cleared
        let query_msg = QueryMsg::OwnerOf {
            token_id: token_id.clone(),
            include_expired: None,
        };
        let res: OwnerOfResponse =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg.clone()).unwrap()).unwrap();
        assert_eq!(
            res,
            OwnerOfResponse {
                owner: String::from("person"),
                approvals: vec![],
            }
        );

        // Approve, revoke, and check for empty, to test revoke
        let approve_msg = ExecuteMsg::Approve {
            spender: String::from("random"),
            token_id: token_id.clone(),
            expires: None,
        };
        let owner = mock_info("person", &[payment.clone()]);
        execute(deps.as_mut(), mock_env(), owner.clone(), approve_msg).unwrap();

        let revoke_msg = ExecuteMsg::Revoke {
            spender: String::from("random"),
            token_id,
        };
        execute(deps.as_mut(), mock_env(), owner, revoke_msg).unwrap();

        // Approvals are now removed / cleared
        let res: OwnerOfResponse =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(
            res,
            OwnerOfResponse {
                owner: String::from("person"),
                approvals: vec![],
            }
        );
    }

    #[test]
    fn approving_all_revoking_all() {
        let payment: Coin = Coin {
            denom: PRICE_DENOM.to_string(),
            amount: Uint128::new(PRICE_AMOUNT.into()),
        };

        let mut deps = mock_dependencies(&[]);
        setup_contract(deps.as_mut());

        // Mint a couple tokens (from the same owner)
        let mint_msg1 = ExecuteMsg::Mint(MintMsg {
            owner: String::from("demeter"),
            payment: Some(payment.clone()),
        });

        let minter = mock_info("demeter", &[payment.clone()]);
        let resp = execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg1).unwrap();
        let token_id1 = get_token_id_from_response(&resp).unwrap();

        let mint_msg2 = ExecuteMsg::Mint(MintMsg {
            owner: String::from("demeter"),
            payment: Some(payment.clone()),
        });

        let resp = execute(deps.as_mut(), mock_env(), minter, mint_msg2).unwrap();
        let token_id2 = get_token_id_from_response(&resp).unwrap();

        // demeter gives random full (operator) power over her tokens
        let approve_all_msg = ExecuteMsg::ApproveAll {
            operator: String::from("random"),
            expires: None,
        };
        let owner = mock_info("demeter", &[payment.clone()]);
        let res = execute(deps.as_mut(), mock_env(), owner, approve_all_msg).unwrap();
        assert_eq!(
            res,
            Response::new()
                .add_attribute("action", "approve_all")
                .add_attribute("sender", "demeter")
                .add_attribute("operator", "random")
        );

        // random can now transfer
        let random = mock_info("random", &[payment.clone()]);
        let transfer_msg = ExecuteMsg::TransferNft {
            recipient: String::from("person"),
            token_id: token_id1,
        };
        execute(deps.as_mut(), mock_env(), random.clone(), transfer_msg).unwrap();

        // random can now send
        let inner_msg = WasmMsg::Execute {
            contract_addr: "another_contract".into(),
            msg: to_binary("You now also have the growing power").unwrap(),
            funds: vec![],
        };
        let msg: CosmosMsg = CosmosMsg::Wasm(inner_msg);

        let send_msg = ExecuteMsg::SendNft {
            contract: String::from("another_contract"),
            token_id: token_id2,
            msg: to_binary(&msg).unwrap(),
        };
        execute(deps.as_mut(), mock_env(), random, send_msg).unwrap();

        // Approve_all, revoke_all, and check for empty, to test revoke_all
        let approve_all_msg = ExecuteMsg::ApproveAll {
            operator: String::from("operator"),
            expires: None,
        };
        // person is now the owner of the tokens
        let owner = mock_info("person", &[payment.clone()]);
        execute(deps.as_mut(), mock_env(), owner, approve_all_msg).unwrap();

        let res: ApprovedForAllResponse = from_binary(&query(deps.as_ref(), mock_env(),
            QueryMsg::ApprovedForAll {
                owner: String::from("person"),
                include_expired: Some(true),
                start_after: None,
                limit: None,
            }
        ).unwrap()).unwrap();

        assert_eq!(
            res,
            ApprovedForAllResponse {
                operators: vec![cw721::Approval {
                    spender: String::from("operator"),
                    expires: Expiration::Never {}
                }]
            }
        );

        // second approval
        let buddy_expires = Expiration::AtHeight(1234567);
        let approve_all_msg = ExecuteMsg::ApproveAll {
            operator: String::from("buddy"),
            expires: Some(buddy_expires),
        };
        let owner = mock_info("person", &[payment.clone()]);
        execute(deps.as_mut(), mock_env(), owner.clone(), approve_all_msg).unwrap();

        // and paginate queries
        let res: ApprovedForAllResponse = from_binary(&query(deps.as_ref(), mock_env(),
            QueryMsg::ApprovedForAll {
                owner: String::from("person"),
                include_expired: Some(true),
                start_after: None,
                limit: Some(1),
            }
        ).unwrap()).unwrap();
        assert_eq!(
            res,
            ApprovedForAllResponse {
                operators: vec![cw721::Approval {
                    spender: String::from("buddy"),
                    expires: buddy_expires,
                }]
            }
        );
        let res: ApprovedForAllResponse = from_binary(&query(deps.as_ref(), mock_env(),
            QueryMsg::ApprovedForAll {
                owner: String::from("person"),
                include_expired: Some(true),
                start_after: Some(String::from("buddy")),
                limit: Some(2),
            }
        ).unwrap()).unwrap();
        assert_eq!(
            res,
            ApprovedForAllResponse {
                operators: vec![cw721::Approval {
                    spender: String::from("operator"),
                    expires: Expiration::Never {}
                }]
            }
        );

        let revoke_all_msg = ExecuteMsg::RevokeAll {
            operator: String::from("operator"),
        };
        execute(deps.as_mut(), mock_env(), owner, revoke_all_msg).unwrap();

        // Approvals are removed / cleared without affecting others
        let res: ApprovedForAllResponse = from_binary(&query(deps.as_ref(), mock_env(),
            QueryMsg::ApprovedForAll {
                owner: String::from("person"),
                include_expired: Some(false),
                start_after: None,
                limit: None,
            }
        ).unwrap()).unwrap();
        assert_eq!(
            res,
            ApprovedForAllResponse {
                operators: vec![cw721::Approval {
                    spender: String::from("buddy"),
                    expires: buddy_expires,
                }]
            }
        );

        // ensure the filter works (nothing should be here
        let mut late_env = mock_env();
        late_env.block.height = 1234568; //expired
        let res: ApprovedForAllResponse = from_binary(&query(deps.as_ref(), late_env,
            QueryMsg::ApprovedForAll {
                owner: String::from("person"),
                include_expired: Some(false),
                start_after: None,
                limit: None,
            }
        ).unwrap()).unwrap();
        assert_eq!(0, res.operators.len());
    }

    #[test]
    fn query_tokens_by_owner() {
        let payment: Coin = Coin {
            denom: PRICE_DENOM.to_string(),
            amount: Uint128::new(PRICE_AMOUNT.into()),
        };

        let mut deps = mock_dependencies(&[]);
        setup_contract(deps.as_mut());

        // Mint a couple tokens
        let demeter = "Demete";
        let ceres = "Ceres";

        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(demeter),
            payment: Some(payment.clone()),
        });
        let resp = execute(deps.as_mut(), mock_env(), mock_info(demeter, &[payment.clone()]), mint_msg).unwrap();
        let token_id1 = get_token_id_from_response(&resp).unwrap();

        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(ceres),
            payment: Some(payment.clone()),
        });
        let resp = execute(deps.as_mut(), mock_env(), mock_info(ceres, &[payment.clone()]), mint_msg).unwrap();
        let token_id2 = get_token_id_from_response(&resp).unwrap();

        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(demeter),
            payment: Some(payment.clone()),
        });
        let resp = execute(deps.as_mut(), mock_env(), mock_info(demeter, &[payment.clone()]), mint_msg).unwrap();
        let token_id3 = get_token_id_from_response(&resp).unwrap();

        // get all tokens in order:
        let expected = vec![token_id1.clone(), token_id2.clone(), token_id3.clone()];
        let tokens: TokensResponse = from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::AllTokens{
            start_after: None, 
            limit: None}).unwrap()).unwrap();

        assert_eq!(&expected, &tokens.tokens);
        // paginate
        let tokens: TokensResponse = from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::AllTokens{
            start_after: None, 
            limit: Some(2)}).unwrap()).unwrap();

        assert_eq!(&expected[..2], &tokens.tokens[..]);
        let tokens: TokensResponse = from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::AllTokens{
            start_after: Some(expected[1].clone()), 
            limit: None}).unwrap()).unwrap();

        assert_eq!(&expected[2..], &tokens.tokens[..]);

        // get by owner
        let by_ceres = vec![token_id2];
        let by_demeter = vec![token_id1, token_id3];
        // all tokens by owner
        let tokens: TokensResponse = from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::Tokens {
            owner: String::from(demeter),
            start_after: None,
            limit: None,
        }).unwrap()).unwrap();

        assert_eq!(&by_demeter, &tokens.tokens);
        let tokens: TokensResponse = from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::Tokens {
            owner: String::from(ceres),
            start_after: None,
            limit: None,
        }).unwrap()).unwrap();

        assert_eq!(&by_ceres, &tokens.tokens);

        // paginate for demeter
        let tokens: TokensResponse = from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::Tokens {
            owner: String::from(demeter),
            start_after: None,
            limit: Some(1),
        }).unwrap()).unwrap();

        assert_eq!(&by_demeter[..1], &tokens.tokens[..]);
        let tokens: TokensResponse = from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::Tokens {
            owner: String::from(demeter),
            start_after: Some(by_demeter[0].clone()),
            limit: Some(3),
        }).unwrap()).unwrap();

        assert_eq!(&by_demeter[1..], &tokens.tokens[..]);
    }

    #[test]
    fn query_balance_of() {
        let payment: Coin = Coin {
            denom: PRICE_DENOM.to_string(),
            amount: Uint128::new(PRICE_AMOUNT.into()),
        };

        let mut deps = mock_dependencies(&[]);
        setup_contract(deps.as_mut());

        // Mint a couple tokens (from the same owner)
        let demeter = "Demete";
        let ceres = "Ceres";

        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(demeter),
            payment: Some(payment.clone()),
        });
        execute(deps.as_mut(), mock_env(), mock_info(demeter, &[payment.clone()]), mint_msg).unwrap();

        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(ceres),
            payment: Some(payment.clone()),
        });
        execute(deps.as_mut(), mock_env(), mock_info(ceres, &[payment.clone()]), mint_msg).unwrap();

        let mint_msg = ExecuteMsg::Mint(MintMsg {
            owner: String::from(demeter),
            payment: Some(payment.clone()),
        });
        execute(deps.as_mut(), mock_env(), mock_info(demeter, &[payment.clone()]), mint_msg).unwrap();

        // get by owner
        let by_ceres = 1;
        let by_demeter = 2;
        // all tokens by owner
        let tokens: NumTokensResponse = 
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::BalanceOf { 
                owner: String::from(demeter) }).unwrap()).unwrap();

        assert_eq!(&by_demeter, &tokens.count);
        let tokens: NumTokensResponse = 
            from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::BalanceOf { 
                owner: String::from(ceres) }).unwrap()).unwrap();
                
        assert_eq!(&by_ceres, &tokens.count);
    }
}
