import { Request, Response, NextFunction } from "express"
import { verifySignature } from "@/signature/signature";

export const signedMessageOK = () => {
    return (req: Request, resp: Response, next: NextFunction) => {
      if (!req.body?.signature) {
        resp.status(400).send('Server requires signature')
      }
      const {signature, public_key} = req.body?.signature;
      const message = req.body?.message;
      if (!signature || !public_key) {
        resp.status(400).send('Server requires signature')
      }
      if (!message) {
        resp.status(400).send('Server requires message')
      }
      if (!verifySignature(Buffer.from(JSON.stringify(message)), signature, public_key)) {
        resp.status(401).send('could not verify the message with the provided signature')
      } else {
        next()
      }
    }
  }
  