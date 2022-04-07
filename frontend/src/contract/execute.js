import { MsgExecuteContract } from "@terra-money/terra.js";
import { getLCD } from "./lcd"

// ==== utils ====

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const until = Date.now() + 1000 * 60 * 60;
const untilInterval = Date.now() + 1000 * 60;

export const exec = (wallet, contract, msg, coins = undefined) => {
  return _exec(contract, msg, coins)(wallet)
}

const _exec =
  (contract, msg, coins = undefined) =>
  async (wallet) => {
    const lcd = getLCD();
    const { result } = await wallet.post({
      //fee,
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