/**
 * Hashes any of several types of source data using the specified hashing algorithm, and returns
 * a RAW message digest value whose size and security depends on the algorithm.
 *
 * Syntax
 *
 * `MESSAGE-DIGEST ( hash-algorithm , data-to-hash [ , hash-key ] )`
 *
 * Notes
 * - The hash-algorithm parameter specifies the hashing algorithm to use:
 * * "MD5" - RSA Message Digest Hash Algorithm, returns a 16-byte RAW binary message digest value.
 * Provided for backward compatibility only and should not be used in new development.
 * * "SHA-1" - United States Government Secure Hash Algorithm, returns a RAW 20-byte binary message digest value.
 * * "SHA-256" - United States Government Secure Hash Algorithm, returns a RAW 32-byte binary message digest value.
 * * "SHA-512" - United States Government Secure Hash Algorithm, returns a RAW 64-byte binary message digest value.
 * * "HMAC-SHA-1" - United States Government HMAC algorithm, returns a RAW 20-byte binary message digest value.
 * * "HMAC-SHA-256" - United States Government HMAC algorithm, returns a RAW 32-byte binary message digest value.
 * * "HMAC-SHA-384" - United States Government HMAC algorithm, returns a RAW 48-byte binary message digest value.
 * * "HMAC-SHA-512" - United States Government HMAC algorithm, returns a RAW 64-byte binary message digest value.
 * - If data-to-hash or hash-key is CHARACTER or LONGCHAR, the AVM converts it to UTF-8.
 * To avoid automatic conversion, specify RAW or MEMPTR values.
 * - For HMAC algorithms, it is strongly recommended that you specify a hash key.
 * - If hash-key is not specified it defaults to the empty string ("").
 * - If either data-to-hash or hash-key is the Unknown value (?), then the Unknown value (?) is returned.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE mydata AS CHARACTER NO-UNDO INITIAL "My data to hash".
 * DEFINE VARIABLE hashkey AS CHARACTER NO-UNDO INITIAL "My hash key".
 * DEFINE VARIABLE mdvalue AS RAW NO-UNDO.
 * mdvalue = MESSAGE-DIGEST("SHA-512", mydata, hashkey).
 * MESSAGE "LENGTH: " LENGTH(mdvalue) SKIP
 * "VALUE: " STRING(mdvalue) VIEW-AS ALERT-BOX.
 * ```
 *
 * @param hash-algorithm A character string that specifies the hashing algorithm to use to hash the data.
 * @param data-to-hash The source data to hash. The data may be of type CHARACTER, LONGCHAR, RAW, or MEMPTR.
 * @param hash-key A key value to use in the hash operation for the hashing algorithms. The key may be of type
 * CHARACTER, LONGCHAR, RAW, or MEMPTR.
 * @returns RAW - A message digest value whose size depends on the algorithm.
 */
FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS CHARACTER,
 INPUT hash-key AS CHARACTER
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS CHARACTER,
 INPUT hash-key AS LONGCHAR
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS CHARACTER,
 INPUT hash-key AS RAW
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS CHARACTER,
 INPUT hash-key AS MEMPTR
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS CHARACTER
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS LONGCHAR,
 INPUT hash-key AS CHARACTER
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS LONGCHAR,
 INPUT hash-key AS LONGCHAR
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS LONGCHAR,
 INPUT hash-key AS RAW
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS LONGCHAR,
 INPUT hash-key AS MEMPTR
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS LONGCHAR
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS RAW,
 INPUT hash-key AS CHARACTER
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS RAW,
 INPUT hash-key AS LONGCHAR
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS RAW,
 INPUT hash-key AS RAW
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS RAW,
 INPUT hash-key AS MEMPTR
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS RAW
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS MEMPTR,
 INPUT hash-key AS CHARACTER
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS MEMPTR,
 INPUT hash-key AS LONGCHAR
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS MEMPTR,
 INPUT hash-key AS RAW
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS MEMPTR,
 INPUT hash-key AS MEMPTR
 ) FORWARD.

FUNCTION MESSAGE-DIGEST RETURNS RAW (
 INPUT hash-algorithm AS CHARACTER,
 INPUT data-to-hash AS MEMPTR
 ) FORWARD.
