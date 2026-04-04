/**
 * Generates a pseudorandom (rather than a truly random) series of bytes to use as an encryption key, and
 * returns the key as a RAW value.
 *
 * Syntax
 *
 * `GENERATE-RANDOM-KEY`
 *
 * Notes
 * - You are responsible for generating, storing, and transporting this value.
 * - The size of the generated encryption key is determined by the cryptographic algorithm specified by the
 * SYMMETRIC-ENCRYPTION-ALGORITHM attribute.
 * - The Alternate Random Number Generator (-rand) startup parameter setting has no effect on this function.
 *
 * @returns RAW - A pseudorandom series of bytes to use as an encryption key
 */
FUNCTION GENERATE-RANDOM-KEY RETURNS RAW (
 ) FORWARD.
