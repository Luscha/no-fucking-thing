import { LCDClient } from '@terra-money/terra.js'
import {NETWORK_URL, CHAIN_ID} from '@/config.js'

export const getLCD = () => {
  console.log(NETWORK_URL)
  console.log(process.env.NETWORK_URL)
  console.log(CHAIN_ID)
  return new LCDClient({
    URL: NETWORK_URL,
    chainID: CHAIN_ID,
    // gasPrices: { uusd: 0.15 }
  })}