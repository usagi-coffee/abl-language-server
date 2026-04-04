/**
 * Returns the absolute value of a numeric value.
 *
 * Syntax
 *
 * `ABSOLUTE ( n )`
 *
 * n
 * An integer or decimal expression. The return value is the same format as n.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE mark-start AS DECIMAL NO-UNDO.
 * DEFINE VARIABLE mark-finish AS DECIMAL NO-UNDO.
 * DISPLAY ABSOLUTE(mark-start - mark-finish).
 * ```
 *
 * @param n An integer or decimal expression. The return value is the same format as n.
 * @returns The absolute value of a numeric value.
 */
FUNCTION ABSOLUTE RETURNS INTEGER (
 n AS INTEGER
 ) FORWARD.

FUNCTION ABSOLUTE RETURNS DECIMAL (
 n AS DECIMAL
 ) FORWARD.
