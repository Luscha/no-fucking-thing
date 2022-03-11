import { query } from "@/contract/query"
import { Request, Response, NextFunction } from "express"

export const CW721OK = () => {
    return async (req: Request, resp: Response, next: NextFunction) => {
        const { signature, public_key } = req.body?.signature;
        const message = req.body?.message;
        if (!message || !message.contractAddrs) {
            resp.status(400).send('Server requires message.contractAddrs')
        }
        let error = undefined;
        await query(message.contractAddrs, { contract_info: {} })
        .catch(err => error = err);
        if (error) {
            console.log(error.response)
            if(error.response?.data?.code == 2) {
                resp.status(400).send('the provided address is not a cw721 contract')
            } else {
                resp.status(500).send("error retreiving contract information")
            }
            return
        }

        next()
    }
}