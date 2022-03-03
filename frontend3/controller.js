import {
    getChainOptions,
    WalletController,
  } from '@terra-money/wallet-controller';
  
let instance = undefined;

export async function initController() {
  const chainOptions = await getChainOptions();

  instance = new WalletController({
    ...chainOptions,
  });
}

export function getController() {
  return instance;
}
