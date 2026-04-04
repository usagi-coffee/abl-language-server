/**
 * Converts source data into a particular format, and returns a MEMPTR containing the encrypted data (a binary byte stream).
 * You must use the same cryptographic algorithm, initialization vector, and encryption key values to encrypt and decrypt the same data instance.
 *
 * Syntax
 *
 * `ENCRYPT ( data-to-encrypt [, encrypt-key [, iv-value [, algorithm [, tag [, aad ] ] ]]] )`
 *
 * Notes
 * - The tag and aad parameters are applicable only to the AES_GCM_128, AES_GCM_192 and AES_GCM_256 encryption algorithms. They are not applicable to other supported encryption algorithms.
 * - If you use the GENERATE-RANDOM-KEY function to generate an encryption key, be sure to invoke the function before invoking the ENCRYPT function (not within the ENCRYPT function, which would render the key irretrievable).
 *
 * @param data-to-encrypt The source data to encrypt. The value may be of type CHARACTER, LONGCHAR, RAW, or MEMPTR.
 * @param encrypt-key An optional RAW expression that evaluates to the name of the encryption key (a binary value) to use in encrypting the specified data. If you specify the Unknown value (?), the current value of the SYMMETRIC-ENCRYPTION-KEY attribute is used. If the value of the SYMMETRIC-ENCRYPTION-KEY attribute is also the Unknown value (?), the AVM generates a run-time error.
 * @param iv-value An optional RAW expression that evaluates to an initialization vector value to use with the specified encryption key in the encryption operation. Using an initialization vector value increases the strength of the specified encryption key. If you specify the Unknown value (?), the current value of the SYMMETRIC-ENCRYPTION-IV attribute is used.
 * @param algorithm An optional CHARACTER expression that evaluates to the name of the symmetric cryptographic algorithm to use in encrypting the specified data instance. If you specify the Unknown value (?), the current value of the SYMMETRIC-ENCRYPTION-ALGORITHM attribute is used.
 * @param tag For AES_GCM encryption, a mandatory MEMPTR expression of a cryptographic checksum generated on the unencrypted data and additional authenticated data.
 * @param aad For AES_GCM encryption, an optional RAW expression of input data that is authenticated but not encrypted.
 * @returns A MEMPTR containing the encrypted data (a binary byte stream).
 */
FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS CHARACTER
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS LONGCHAR
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS RAW
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS MEMPTR
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS CHARACTER,
 INPUT encrypt-key AS RAW
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS LONGCHAR,
 INPUT encrypt-key AS RAW
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS RAW,
 INPUT encrypt-key AS RAW
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS MEMPTR,
 INPUT encrypt-key AS RAW
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS CHARACTER,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS LONGCHAR,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS RAW,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS MEMPTR,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS CHARACTER,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW,
 INPUT algorithm AS CHARACTER
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS LONGCHAR,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW,
 INPUT algorithm AS CHARACTER
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS RAW,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW,
 INPUT algorithm AS CHARACTER
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS MEMPTR,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW,
 INPUT algorithm AS CHARACTER
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS CHARACTER,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW,
 INPUT algorithm AS CHARACTER,
 INPUT tag AS MEMPTR
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS LONGCHAR,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW,
 INPUT algorithm AS CHARACTER,
 INPUT tag AS MEMPTR
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS RAW,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW,
 INPUT algorithm AS CHARACTER,
 INPUT tag AS MEMPTR
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS MEMPTR,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW,
 INPUT algorithm AS CHARACTER,
 INPUT tag AS MEMPTR
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS CHARACTER,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW,
 INPUT algorithm AS CHARACTER,
 INPUT tag AS MEMPTR,
 INPUT aad AS RAW
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS LONGCHAR,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW,
 INPUT algorithm AS CHARACTER,
 INPUT tag AS MEMPTR,
 INPUT aad AS RAW
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS RAW,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW,
 INPUT algorithm AS CHARACTER,
 INPUT tag AS MEMPTR,
 INPUT aad AS RAW
 ) FORWARD.

FUNCTION ENCRYPT RETURNS MEMPTR (
 INPUT data-to-encrypt AS MEMPTR,
 INPUT encrypt-key AS RAW,
 INPUT iv-value AS RAW,
 INPUT algorithm AS CHARACTER,
 INPUT tag AS MEMPTR,
 INPUT aad AS RAW
 ) FORWARD.
