import { LCDClient, MsgExecuteContract, Fee } from "@terra-money/terra.js";
import { contractAdress } from "./address";

// ==== utils ====

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const until = Date.now() + 1000 * 60 * 60;
const untilInterval = Date.now() + 1000 * 60;

export const exec = (wallet, contract, msg, coins = undefined, fee = new Fee(200000, { uluna: 10000 })) => {
  return _exec(contractAdress(wallet.network.chainID, contract), msg, coins, fee)(wallet)
}

export const execRaw = (wallet, contract, msg, coins = undefined, fee = new Fee(200000, { uluna: 10000 })) => {
  return _exec(contract, msg, coins, fee)(wallet)
}

const _exec =
  (contract, msg, coins = undefined, fee = new Fee(200000, { uluna: 10000 })) =>
  async (wallet) => {
    const lcd = new LCDClient({
      URL: wallet.network.lcd,
      chainID: wallet.network.chainID,
    });
    
    const { result } = await wallet.post({
      fee,
      msgs: [
        new MsgExecuteContract(
          wallet.walletAddress,
          contract,
          msg,
          coins
        ),
      ],
    });
    
    /* eslint-disable no-constant-condition */
    while (true) {
      try {
        return await lcd.tx.txInfo(result.txhash);
      } catch (e) {
        if (Date.now() < untilInterval) {
          await sleep(500);
        } else if (Date.now() < until) {
          await sleep(1000 * 10);
        } else {
          throw new Error(
            `Transaction queued. To verify the status, please check the transaction hash: ${result.txhash}`
          );
        }
      }
    }
  };