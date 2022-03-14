export const signMessage = async (wallet, message) => {
    if (!wallet) {
        throw new Error("wallet is required to sign a message")
    }

    const buff = Buffer.from(JSON.stringify(message));
    const res = await wallet.signBytes(buff)
    return {
        message: message,
        signature: {
            public_key: res.result.public_key.toData(),
            signature: Array.from(res.result.signature)
        }
    }
}