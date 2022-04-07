import { getLCD } from "./lcd"

export const query = async (contract, func) => {
  return getLCD().wasm.contractQuery(contract, func)
}
