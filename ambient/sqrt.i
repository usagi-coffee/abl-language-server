/**
 * Returns the square root (as a DECIMAL value) of an expression you specify.
 *
 * Syntax
 *
 * `SQRT ( expression )`
 *
 * Notes
 * - If the value of the expression is negative, SQRT returns the Unknown value (?).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE iNum AS INTEGER NO-UNDO FORMAT ">,>>9"
 * LABEL "Enter a number between 1 and 9,999".
 * REPEAT WITH SIDE-LABELS CENTERED
 * TITLE "SQUARE ROOT GENERATOR" COLUMN 20 1 DOWN:
 *   DISPLAY SKIP(2).
 *   SET iNum SKIP(2).
 *   DISPLAY "The square root of " + STRING(iNum) + " is" FORMAT "x(27)"
 *     SQRT(iNum) FORMAT ">>>.9999".
 * END.
 * ```
 *
 * @param expression A numeric expression.
 * @returns The square root as a DECIMAL value.
 */
FUNCTION SQRT RETURNS DECIMAL (
    INPUT expression AS DECIMAL
  ) FORWARD.
