use cw721::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Coin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    WithdrawNft { offering_id: String },
    Buy { offering_id: String, payment: Coin },
    // TODO: SECURITY use cw721 Approve msg instead of transfering the nft
    // to allow the marketplace to transfer the nft once the sell is concluded
    // in this way the user do not have to transfer the nft to the platform
    ReceiveNft(Cw721ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SellNft {
    pub list_price: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // Offerings returns a list of all offerings
    Offerings {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    OfferingsByOwner {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    OfferingsByCollection {
        collection_contract: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    OfferingByNft {
        collection_contract: String,
        token_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}