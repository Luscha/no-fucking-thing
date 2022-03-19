export function safeIpfsToUrl(url) {
    if (!url.startsWith('ipfs://')) {
        return url;
    }
    // pinata default gateway
    return "https://gateway.pinata.cloud/ipfs/"+url.substring('ipfs://'.length);
}
