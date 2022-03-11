import jscrypto from 'jscrypto';
import secp256k1 from 'secp256k1';
import {
    SimplePublicKey
} from '@terra-money/terra.js'

export function verifySignature(
  bytes: Buffer,
  signBytesResult: any,
  publicKeyData: any,
): boolean {
  const publicKey = SimplePublicKey.fromData(publicKeyData).toProto();
  const signature = Uint8Array.from(signBytesResult)

  if (publicKey && 'key' in publicKey) {
    return secp256k1.ecdsaVerify(
      signature,
      Buffer.from(
        jscrypto.SHA256.hash(new jscrypto.Word32Array(bytes)).toString(),
        'hex',
      ),
      publicKey.key,
    );
  }

  return false;
}