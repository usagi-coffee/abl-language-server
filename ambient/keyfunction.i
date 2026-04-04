/**
 * Evaluates an integer expression (such as 301) and returns a character string that is the function
 * of the key associated with that integer expression (such as GO).
 *
 * Syntax
 *
 * `KEYFUNCTION ( expression )`
 *
 * Notes
 * - The value returned by the KEYFUNCTION function is affected by any ON statements you use to
 * redefine the value of the key represented by expression.
 * - If the key represented by expression has no function currently assigned to it or if it has
 * the function of BELL, KEYFUNCTION returns a null value.
 * - KEYFUNCTION(-2) is equal to ENDKEY.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE func AS CHARACTER NO-UNDO.
 * READKEY.
 * func = KEYFUNCTION(LASTKEY).
 * IF func = "CURSOR-DOWN" THEN
 * MESSAGE "Cursor down pressed".
 * ```
 *
 * @param expression A constant, field name, variable name, or expression whose value is an integer key code.
 * @returns CHARACTER - A character string that is the function of the key associated with the integer expression.
 */
FUNCTION KEYFUNCTION RETURNS CHARACTER (
 INPUT expression AS INTEGER
 ) FORWARD.
