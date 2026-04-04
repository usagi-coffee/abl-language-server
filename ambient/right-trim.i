/**
 * Removes trailing white space, or other specified characters, from a CHARACTER or LONGCHAR expression.
 *
 * Syntax
 *
 * `RIGHT-TRIM ( expression [ , trim-chars ] )`
 *
 * Notes
 * - The RIGHT-TRIM function is similar to the TRIM function except that it trims characters only from the right end of the string.
 * - If expression is a case-sensitive field or variable, then trim-chars is also treated as case sensitive.
 * - The RIGHT-TRIM function is double-byte enabled.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE txt AS CHARACTER NO-UNDO INITIAL "***** This is a test *****".
 * MESSAGE RIGHT-TRIM(txt, "* ").
 * ```
 *
 * @param expression An expression whose value is a CHARACTER or LONGCHAR.
 * @param trim-chars A character expression that specifies the characters to trim from expression.
 * @returns The data type of the returned value matches the data type of the expression passed to the function.
 */
FUNCTION RIGHT-TRIM RETURNS CHARACTER (
    INPUT expression AS CHARACTER,
    INPUT trim-chars AS CHARACTER
  ) FORWARD.

FUNCTION RIGHT-TRIM RETURNS CHARACTER (
    INPUT expression AS CHARACTER
  ) FORWARD.
