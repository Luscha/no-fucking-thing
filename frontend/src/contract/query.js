import { LCDClient } from '@terra-money/terra.js'
import { contractAdress } from './address'

export const query = async (contract, func) => {
  const chainID = process.env.CHAIN_ID || 'localterra';
  const lcd = new LCDClient({
    URL: process.env.NETWORK_URL || 'http://localhost:1317',
    chainID: chainID,
  })
  return lcd.wasm.contractQuery(contractAdress(chainID, contract), func)
}

export const queryRaw = async (contract, func) => {
  const chainID = process.env.CHAIN_ID || 'localterra';
  const lcd = new LCDClient({
    URL: process.env.NETWORK_URL || 'http://localhost:1317',
    chainID: chainID,
  })
  return lcd.wasm.contractQuery(contract, func)
}
