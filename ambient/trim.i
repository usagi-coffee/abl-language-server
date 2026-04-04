/**
 * Removes leading and trailing white space, or other specified characters, from a CHARACTER or LONGCHAR expression.
 *
 * Syntax
 *
 * `TRIM ( expression [, trim-chars ] )`
 *
 * Notes
 * - The TRIM function is double-byte enabled. The specified expression and trim-chars arguments can contain double-byte characters. TRIM does not remove double-byte space characters by default.
 * - If expression is a case-sensitive field or variable, then trim-chars is also case sensitive. Otherwise, trim-chars is not case sensitive.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE word AS CHARACTER NO-UNDO.
 * word = TRIM("  hello  ").
 * word = TRIM("...hello...", ".").
 * ```
 *
 * @param expression An expression whose value is a CHARACTER or LONGCHAR.
 * @param trim-chars A character expression that specifies the characters to trim from expression. If not specified, removes spaces, tabs, line feeds, and carriage returns.
 * @returns The trimmed string with the same data type as the expression.
 */
FUNCTION TRIM RETURNS CHARACTER (
    INPUT expression AS CHARACTER,
    INPUT trim-chars AS CHARACTER
  ) FORWARD.

FUNCTION TRIM RETURNS CHARACTER (
    INPUT expression AS CHARACTER
  ) FORWARD.
