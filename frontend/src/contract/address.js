// sync-ed from root via `tr sync-refs`
import config from "../refs.terrain.json"
export const contractAdress = (chain, contract) => config[chain]?.[contract]?.contractAddresses?.default
