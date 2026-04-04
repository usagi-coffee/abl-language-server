/**
 * Generates a password-based encryption key, based on the PKCS#5/RFC 2898 standard, and returns the key
 * as a RAW value.
 *
 * Syntax
 *
 * `GENERATE-PBE-KEY( password [ , salt] )`
 *
 * Notes
 * - You are responsible for generating, storing, and transporting these values.
 * - The size of the generated encryption key is determined by the cryptographic algorithm specified by the
 * SYMMETRIC-ENCRYPTION-ALGORITHM attribute.
 * - Before invoking this function, be sure to set the PBE-HASH-ALGORITHM attribute to the name of the hash
 * algorithm to use.
 * - If you call this function multiple times with the same password string, hash algorithm, number of iterations,
 * and salt value, the same binary key is generated each time.
 *
 * @param password The password (a binary value) to use in generating the encryption key. This value may be of type
 * CHARACTER, LONGCHAR, RAW, or MEMPTR.
 * @param salt An optional RAW expression that evaluates to the salt value (a random series of 8 bytes) to use in
 * generating the encryption key.
 * @returns The generated password-based encryption key as a RAW value.
 */
FUNCTION GENERATE-PBE-KEY RETURNS RAW (
 password AS {CHARACTER|LONGCHAR|RAW|MEMPTR},
 salt AS RAW
 ) FORWARD.

FUNCTION GENERATE-PBE-KEY RETURNS RAW (
 password AS {CHARACTER|LONGCHAR|RAW|MEMPTR}
 ) FORWARD.
