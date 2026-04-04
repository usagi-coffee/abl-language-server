/**
 * Removes leading white space, or other specified characters, from a CHARACTER or LONGCHAR expression.
 *
 * Syntax
 *
 * `LEFT-TRIM ( expression [ , trim-chars ] )`
 *
 * Notes
 * - The LEFT-TRIM function is similar to the TRIM function except that it trims characters only from the left end of the string.
 * - If expression is a case-sensitive field or variable, then trim-chars is also case sensitive.
 * - The LEFT-TRIM function is double-byte enabled.
 * - LEFT-TRIM does not remove double-byte space characters by default.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE txt AS CHARACTER NO-UNDO INITIAL "***** This is a test *****".
 * txt = LEFT-TRIM(txt, "* ").
 * ```
 *
 * @param expression An expression whose value is a CHARACTER or LONGCHAR.
 * @param trim-chars A character expression that specifies the characters to be trimmed.
 * @returns The expression with leading white space or specified characters removed.
 */
FUNCTION LEFT-TRIM RETURNS CHARACTER (
    INPUT expression AS CHARACTER,
    INPUT trim-chars AS CHARACTER
  ) FORWARD.

FUNCTION LEFT-TRIM RETURNS LONGCHAR (
    INPUT expression AS LONGCHAR,
    INPUT trim-chars AS LONGCHAR
  ) FORWARD.

FUNCTION LEFT-TRIM RETURNS CHARACTER (
    INPUT expression AS CHARACTER
  ) FORWARD.

FUNCTION LEFT-TRIM RETURNS LONGCHAR (
    INPUT expression AS LONGCHAR
  ) FORWARD.
