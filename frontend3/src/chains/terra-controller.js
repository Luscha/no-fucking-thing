import {
    getChainOptions,
    WalletController,
  } from '@terra-money/wallet-controller';
  
let instance = undefined;

export async function initController() {
  if (instance) {
    return;
  }
  
  const chainOptions = await getChainOptions();

  instance = new WalletController({
    ...chainOptions,
  });
}

export function getController() {
  if (instance) {
    return instance;
  }
}
