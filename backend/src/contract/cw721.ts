import { query } from "@/contract/query"

export const CW721Info = async (address): Promise<any>=> {
    const info = await query(address, { contract_info: {} });
    return info;
}