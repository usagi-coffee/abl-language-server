/**
 * Returns a character value that indicates whether a string is an ABL keyword. This function returns all keywords and does not distinguish between reserved or unreserved keywords.
 *
 * Syntax
 *
 * `KEYWORD-ALL ( expression )`
 *
 * Notes
 * - Because KEYWORD-ALL recognizes abbreviations, it does not distinguish between FORM and FORMAT or between ACCUM and ACCUMULATE.
 * - This function returns the Unknown value (?) for colors and most data types, as well as all unreserved keywords.
 * - For SpeedScript, all ABL reserved keywords are also reserved for SpeedScript.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE formname AS CHARACTER NO-UNDO FORMAT "x(20)".
 * REPEAT ON ERROR UNDO, RETRY:
 *   UPDATE formname.
 *   IF KEYWORD-ALL(formname) NE ? THEN DO:
 *     MESSAGE formname + "cannot be used as a form name".
 *     UNDO, RETRY.
 *   END.
 *   ELSE LEAVE.
 * END.
 * ```
 *
 * @param expression A constant, field name, variable name, or expression that results in a character string.
 * @returns Returns the full keyword if expression matches an ABL keyword, whether reserved or unreserved or valid abbreviation of a keyword. Returns the Unknown value (?) if there is no match.
 */
FUNCTION KEYWORD-ALL RETURNS CHARACTER (
    INPUT expression AS CHARACTER
  ) FORWARD.
