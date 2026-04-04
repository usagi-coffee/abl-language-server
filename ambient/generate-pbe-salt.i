/**
 * Generates a random salt value (a series of 8 bytes) to use in generating an encryption key, and returns the
 * salt value as a RAW value. Using a salt value can help to ensure that a password key value is unique.
 *
 * Syntax
 *
 * `GENERATE-PBE-SALT`
 *
 * Notes
 * - This salt value is combined with a password value and hashed some number of times to generate a
 * password-based encryption key (using the algorithm specified by the PBE-HASH-ALGORITHM attribute
 * and the number of iterations specified by the PBE-KEY-ROUNDS attribute).
 * - You are responsible for generating, storing, and transporting this value.
 *
 * @returns A RAW value containing the generated salt.
 */
FUNCTION GENERATE-PBE-SALT RETURNS RAW () FORWARD.
