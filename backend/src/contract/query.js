import { LCDClient } from '@terra-money/terra.js'

export const query = async (contract, func) => {
  const chainID = process.env.CHAIN_ID || 'localterra';
  const lcd = new LCDClient({
    URL: process.env.NETWORK_URL || 'http://localhost:1317',
    chainID: chainID,
  })
  return lcd.wasm.contractQuery(contract, func)
}
