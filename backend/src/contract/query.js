import { LCDClient } from '@terra-money/terra.js'
import { NETWORK_URL, CHAIN_ID } from "../config"

export const query = async (contract, func) => {
  const lcd = new LCDClient({
    URL: NETWORK_URL,
    chainID: CHAIN_ID,
  })
  return lcd.wasm.contractQuery(contract, func)
}
