/**
 * Converts encrypted data (a binary byte stream) to its original source format, and returns a MEMPTR containing
 * the decrypted data.
 * You must use the same cryptographic algorithm, initialization vector, and encryption key values to encrypt and
 * decrypt the same data instance.
 *
 * Syntax
 *
 * `DECRYPT ( data-to-decrypt [, encrypt-key [, iv-value [, algorithm [, tag [, aad ] ] ]]] )`
 *
 * Notes
 * - The tag and aad parameters are applicable only to the AES_GCM_128, AES_GCM_192 and AES_GCM_256 encryption algorithms. They are not applicable to other supported encryption algorithms.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE cData AS MEMPTR NO-UNDO.
 * cData = DECRYPT(data-to-decrypt, encrypt-key, iv-value, algorithm, tag, aad).
 * ```
 *
 * @param data-to-decrypt The encrypted data to decrypt. The value may be of type RAW or MEMPTR.
 * @param encrypt-key An optional RAW expression that evaluates to the encryption key (a binary value) originally used to encrypt the specified data.
 * @param iv-value An optional RAW expression that evaluates to the initialization vector value to use with the specified encryption key in the original encryption operation.
 * @param algorithm An optional CHARACTER expression that evaluates to the name of the symmetric cryptographic algorithm originally used to encrypt the specified data instance.
 * @param tag For AES_GCM encryption, a mandatory MEMPTR expression of a cryptographic checksum generated on unencrypted data and additional authenticated data.
 * @param aad For AES_GCM encryption, an optional RAW expression of input data that is authenticated but not encrypted.
 * @returns A MEMPTR containing the decrypted data.
 */
FUNCTION DECRYPT RETURNS MEMPTR (
 DATA-TO-DECRYPT AS {RAW|MEMPTR},
 ENCRYPT-KEY AS RAW,
 IV-VALUE AS RAW,
 ALGORITHM AS CHARACTER,
 TAG AS MEMPTR,
 AAD AS RAW
 ) FORWARD.

FUNCTION DECRYPT RETURNS MEMPTR (
 DATA-TO-DECRYPT AS {RAW|MEMPTR}
 ) FORWARD.

FUNCTION DECRYPT RETURNS MEMPTR (
 DATA-TO-DECRYPT AS {RAW|MEMPTR},
 ENCRYPT-KEY AS RAW
 ) FORWARD.

FUNCTION DECRYPT RETURNS MEMPTR (
 DATA-TO-DECRYPT AS {RAW|MEMPTR},
 ENCRYPT-KEY AS RAW,
 IV-VALUE AS RAW
 ) FORWARD.

FUNCTION DECRYPT RETURNS MEMPTR (
 DATA-TO-DECRYPT AS {RAW|MEMPTR},
 ENCRYPT-KEY AS RAW,
 IV-VALUE AS RAW,
 ALGORITHM AS CHARACTER
 ) FORWARD.

FUNCTION DECRYPT RETURNS MEMPTR (
 DATA-TO-DECRYPT AS {RAW|MEMPTR},
 ENCRYPT-KEY AS RAW,
 IV-VALUE AS RAW,
 ALGORITHM AS CHARACTER,
 TAG AS MEMPTR
 ) FORWARD.
