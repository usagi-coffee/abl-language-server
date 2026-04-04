/**
 * Calculates the logarithm of an expression using a specified base and returns that logarithm as a DECIMAL value.
 *
 * Syntax
 *
 * `LOG ( expression [, base] )`
 *
 * Notes
 * - The LOG function is accurate to approximately 10 decimal places.
 * - After converting the base and exponent to floating-point format, the LOG function uses standard system routines. On some machines, the logarithm routines do not handle large numbers well and might cause your terminal to hang.
 * - The base must be greater than 1.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE base AS DECIMAL NO-UNDO FORMAT ">>>,>>>.9999".
 * DEFINE VARIABLE number AS DECIMAL NO-UNDO.
 * REPEAT:
 *   UPDATE base VALIDATE(base > 1, "Base must be greater than 1").
 *   REPEAT:
 *     UPDATE number VALIDATE(number > 0, "Number must be positive").
 *     DISPLAY number LOG(number, base) LABEL "LOG(NUMBER, BASE)".
 *   END.
 * END.
 * ```
 *
 * @param expression A decimal expression that you want the logarithm of.
 * @param base A numeric expression that is the base you want to use. If you do not specify a base, LOG returns the natural logarithm, base (e). The base must be greater than 1.
 * @returns The logarithm as a DECIMAL value.
 */
FUNCTION LOG RETURNS DECIMAL (
    INPUT expression AS DECIMAL,
    INPUT base AS DECIMAL
  ) FORWARD.

FUNCTION LOG RETURNS DECIMAL (
    INPUT expression AS DECIMAL
  ) FORWARD.
