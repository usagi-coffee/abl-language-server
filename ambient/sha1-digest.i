/**
 * Hashes the specified data using the United States Government Secure Hash Algorithm (SHA-1), and returns a 20-byte binary message digest value as a RAW value.
 *
 * Syntax
 *
 * `SHA1-DIGEST( data-to-hash [, hash-key] )`
 *
 * Notes
 * - If the data is a CHARACTER or LONGCHAR value, the AVM converts it to UTF-8 (which ensures a consistent value regardless of code page settings). To avoid this automatic conversion, specify a RAW or MEMPTR value.
 * - If the hash-key is a CHARACTER or LONGCHAR value, the AVM converts it to UTF-8. To avoid this automatic conversion, specify a RAW or MEMPTR value.
 * - If the hash-key value contains a null character, the null character is included in the hash operation.
 * - This function is not compatible with HMAC-SHA-1.
 *
 * @param data-to-hash The source data to hash. The data may be of type CHARACTER, LONGCHAR, RAW, or MEMPTR.
 * @param hash-key An optional key value to use in the hash operation. The key may be of type CHARACTER, LONGCHAR, RAW, or MEMPTR.
 * @returns Returns a 20-byte binary message digest value as a RAW value.
 */
FUNCTION SHA1-DIGEST RETURNS RAW (
    INPUT data-to-hash AS RAW,
    INPUT hash-key AS RAW
  ) FORWARD.

FUNCTION SHA1-DIGEST RETURNS RAW (
    INPUT data-to-hash AS RAW
  ) FORWARD.
