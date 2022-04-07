import { LCDClient } from '@terra-money/terra.js'
import {NETWORK_URL, CHAIN_ID} from '@/config.js'

export const getLCD = () => {
  return new LCDClient({
    URL: NETWORK_URL,
    chainID: CHAIN_ID,
    // gasPrices: { uusd: 0.15 }
  })}