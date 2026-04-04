/**
 * Hashes the specified data using the RSA Message Digest Hash Algorithm (MD5), and returns a 16-byte binary message digest value as a RAW value.
 *
 * Syntax
 *
 * `MD5-DIGEST ( data-to-hash [, hash-key] )`
 *
 * Notes
 * - If the data-to-hash is a CHARACTER or LONGCHAR value, the AVM converts it to UTF-8. To avoid this automatic conversion, specify a RAW or MEMPTR value.
 * - If the hash-key is a CHARACTER or LONGCHAR value, the AVM converts it to UTF-8. To avoid this automatic conversion, specify a RAW or MEMPTR value.
 * - If the hash-key value contains a null character, the null character is included in the hash operation.
 * - This function can be used for backward compatibility but should not be used in new development.
 *
 * @param data-to-hash The source data to hash. The data may be of type CHARACTER, LONGCHAR, RAW, or MEMPTR.
 * @param hash-key An optional key value to use in the hash operation. The key may be of type CHARACTER, LONGCHAR, RAW, or MEMPTR.
 * @returns Returns a 16-byte binary message digest value as a RAW value.
 */
FUNCTION MD5-DIGEST RETURNS RAW (
    INPUT data-to-hash AS CHARACTER,
    INPUT hash-key AS CHARACTER
  ) FORWARD.

FUNCTION MD5-DIGEST RETURNS RAW (
    INPUT data-to-hash AS CHARACTER
  ) FORWARD.
