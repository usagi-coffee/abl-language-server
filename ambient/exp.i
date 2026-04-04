/**
 * Returns the result of raising a number to a power. The number is called the base and the power is called the
 * exponent.
 *
 * Syntax
 *
 * `EXP ( base , exponent )`
 *
 * Notes
 * - After converting the base and exponent to the floating-point format, the EXP function uses standard system
 * library routines. On some machines, these routines do not handle large numbers well and might cause your
 * terminal to hang. Also, because the calculations are done in floating-point arithmetic, full decimal precision
 * is not possible beyond 1-12 significant digits on most machines.
 * - The EXP function is precise to approximately 10 decimal points.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE principal AS DECIMAL NO-UNDO.
 * DEFINE VARIABLE rate AS INTEGER NO-UNDO.
 * DEFINE VARIABLE num-yrs AS INTEGER NO-UNDO.
 * DEFINE VARIABLE final-amt AS DECIMAL NO-UNDO.
 *
 * final-amt = principal * EXP(1 + rate / 100, num-yrs).
 * ```
 *
 * @param base A constant, field name, variable name, or expression that evaluates to a numeric value.
 * @param exponent A numeric expression.
 * @returns The result of raising base to the power of exponent.
 */
FUNCTION EXP RETURNS DECIMAL (
 base AS DECIMAL,
 exponent AS DECIMAL
 ) FORWARD.
