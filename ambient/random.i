/**
 * Returns a random INTEGER value between two integers (inclusive).
 *
 * Syntax
 *
 * `RANDOM ( low , high )`
 *
 * Notes
 * - This function returns a number from a pseudorandom sequence of numbers rather than a truly random sequence.
 * - The Alternate Random Number Generator (-rand) parameter determines whether the same sequence of random numbers is generated for each session.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE roll AS INTEGER NO-UNDO.
 * roll = RANDOM(1, 6).
 * ```
 *
 * @param low An integer expression that is the lower of the two expressions.
 * @param high An integer expression that is the higher of the two expressions.
 * @returns A random INTEGER value between low and high (inclusive).
 */
FUNCTION RANDOM RETURNS INTEGER (
    INPUT low AS INTEGER,
    INPUT high AS INTEGER
  ) FORWARD.
