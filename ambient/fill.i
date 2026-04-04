/**
 * Generates a character string made up of a character string that is repeated a specified number of times.
 *
 * Syntax
 *
 * `FILL ( expression , repeats )`
 *
 * Notes
 * - If the value of repeats is less than or equal to 0, FILL produces a null string.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE fillchar AS CHARACTER NO-UNDO FORMAT "x" INITIAL "*".
 * DEFINE VARIABLE percentg AS INTEGER NO-UNDO FORMAT ">>9".
 *
 * FOR EACH Customer NO-LOCK:
 * ACCUMULATE Customer.Balance (TOTAL).
 * END.
 *
 * FOR EACH Customer NO-LOCK WHERE Customer.Balance > 0:
 * percentg = Customer.Balance / (ACCUM TOTAL Customer.Balance) * 100.
 * DISPLAY Customer.Name percentg FILL(fillchar, percentg * 3) @ bar.
 * END.
 * ```
 *
 * @param expression An expression that yields a character value. This expression can contain double-byte characters.
 * @param repeats A constant, field name, variable name, or expression with an integer value. The FILL function uses this value to repeat the expression you specify.
 * @returns CHARACTER string made up of the character string repeated the specified number of times.
 */
FUNCTION FILL RETURNS CHARACTER (
 expression AS CHARACTER,
 repeats AS INTEGER
 ) FORWARD.
