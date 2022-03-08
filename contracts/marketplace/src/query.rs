#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::package::{OfferingsResponse, QueryOfferingsResult, OfferingResponse};
use crate::state::{Offering, offerings};
use cosmwasm_std::Pair;
use cosmwasm_std::{
    to_binary, Api, Binary, Env, Deps,
    Order, StdResult, StdError
};
use std::str::from_utf8;

use crate::msg::{QueryMsg};
use cw_storage_plus::Bound;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Offerings {
            start_after,
            limit,
        } => to_binary(&query_offerings(deps, start_after, limit)?),
        QueryMsg::OfferingsByOwner {
            owner,
            start_after,
            limit,
        } => to_binary(&query_offerings_owner(deps, owner, start_after, limit)?),
        QueryMsg::OfferingsBycollection {
            collection_contract,
            start_after,
            limit,
        } => to_binary(&query_offerings_collection(deps, collection_contract, start_after, limit)?),
        QueryMsg::OfferingByNFT {
            collection_contract,
            token_id,
        } => to_binary(&query_offering_nft(deps, collection_contract, token_id)?),
    }
}

// ============================== Query Executers ==============================
const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 30;

fn query_offerings(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<OfferingsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let res: StdResult<Vec<QueryOfferingsResult>> = offerings()
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|kv_item| parse_offering(deps.api, kv_item))
        .collect();

    Ok(OfferingsResponse {
        offerings: res?, // Placeholder
    })
}

fn query_offerings_owner(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<OfferingsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let owner_addr = deps.api.addr_canonicalize(&deps.api.addr_validate(&owner)?.as_str())?;

    let res: StdResult<Vec<QueryOfferingsResult>> = offerings()
        .idx.seller
        .prefix(owner_addr.as_slice().to_vec())
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|kv_item| parse_offering(deps.api, kv_item))
        .collect();

    Ok(OfferingsResponse {
        offerings: res?, // Placeholder
    })
}

fn query_offerings_collection(
    deps: Deps,
    collection_contract: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<OfferingsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let collection_addr = deps.api.addr_canonicalize(&deps.api.addr_validate(&collection_contract)?.as_str())?;

    let res: StdResult<Vec<QueryOfferingsResult>> = offerings()
        .idx.contract
        .prefix(collection_addr.as_slice().to_vec())
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|kv_item| parse_offering(deps.api, kv_item))
        .collect();

    Ok(OfferingsResponse {
        offerings: res?, // Placeholder
    })
}

fn query_offering_nft(
    deps: Deps,
    collection_contract: String,
    token_id: String,
) -> StdResult<OfferingResponse>  {
    let collection_addr = deps.api.addr_canonicalize(&deps.api.addr_validate(&collection_contract)?.as_str())?;

    let res: StdResult<Vec<QueryOfferingsResult>> = offerings()
        .idx.contract
        .prefix(collection_addr.as_slice().to_vec())
        .range(deps.storage, None, None, Order::Ascending)
        .filter(|o| {
            o.as_ref().unwrap().1.token_id == token_id
        })
        .map(|kv_item| parse_offering(deps.api, kv_item))
        .collect();

    if res.as_ref().unwrap().is_empty() {
        return Err(StdError::not_found("pair collection, token_id not found"));
    }

    Ok(OfferingResponse{offering: res.as_ref().unwrap()[0].clone()})
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
