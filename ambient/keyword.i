/**
 * Returns a character value that indicates whether a string is an ABL reserved keyword.
 *
 * Syntax
 *
 * `KEYWORD ( expression )`
 *
 * Notes
 * - Because KEYWORD recognizes abbreviations, it does not distinguish between FORM and FORMAT or between ACCUM and ACCUMULATE.
 * - This function returns the Unknown value (?) for colors and most data types, as well as all unreserved keywords.
 * - KEYWORD is less restrictive than the KEYWORD-ALL function. Use this function if you do not want to use ABL reserved keywords as field names, for example.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE formname AS CHARACTER NO-UNDO FORMAT "x(20)".
 * REPEAT ON ERROR UNDO, RETRY:
 *   UPDATE formname.
 *   IF KEYWORD(formname) NE ? THEN DO:
 *     MESSAGE formname + " may not be used as a form name".
 *     UNDO, RETRY.
 *   END.
 *   ELSE LEAVE.
 * END.
 * ```
 *
 * @param expression A constant, field name, variable name, or expression that results in a character string.
 * @returns Returns the full keyword if expression matches an ABL reserved keyword or valid abbreviation. Returns Unknown value (?) if no match.
 */
FUNCTION KEYWORD RETURNS CHARACTER (
    INPUT expression AS CHARACTER
  ) FORWARD.
