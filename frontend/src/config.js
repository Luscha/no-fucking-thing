import {contractAdress} from "@/contract/address";

export const NETWORK_URL = process.env.NETWORK_URL || 'http://localhost:1317';
export const CHAIN_ID = process.env.CHAIN_ID || 'localterra';

export const MARKETPLACE_ADDRESS = process.env.MARKETPLACE_ADDRESS || contractAdress(CHAIN_ID, "marketplace");
export const GEN0_ADDRESS = process.env.GEN0_ADDRESS || contractAdress(CHAIN_ID, "gen0");

export const BACKEND_REST_API_ENDPOINT = process.env.BACKEND_REST_API_ENDPOINT || 'http://localhost:3000'