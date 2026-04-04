/**
 * Returns an INTEGER value giving the position of an expression in a list.
 *
 * Syntax
 *
 * `LOOKUP ( expression , list [ , delimiter ] )`
 *
 * Notes
 * - If expression or list contains the Unknown value (?), the result is the Unknown value (?).
 * - The delimiter is case sensitive. The default is a comma.
 * - If expression contains a delimiter, LOOKUP returns the beginning of a series of entries in list.
 * - Most character comparisons are case insensitive in ABL. If expression or list is defined as case sensitive, the comparison is also case sensitive.
 * - The LOOKUP function is double-byte enabled.
 * - LOOKUP always returns 0 if an expression is equal to a delimiter.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE stlist AS CHARACTER NO-UNDO
 *   INITIAL "ME,MA,VT,RI,CT,NH".
 * DEFINE VARIABLE state AS CHARACTER NO-UNDO FORMAT "x(2)".
 * REPEAT:
 *   SET state LABEL "Enter a New England state, 2 characters".
 *   IF LOOKUP(state, stlist) = 0 THEN
 *     MESSAGE "This is not a New England state".
 * END.
 * ```
 *
 * @param expression A constant, field name, variable name, or expression that results in a character value that you want to look up within a list of character expressions.
 * @param list A character expression that contains the expression you name with the expression argument. Each entry in the list is separated with a delimiter.
 * @param delimiter A single-character delimiter you define for the list. The default is a comma.
 * @returns Returns an INTEGER value giving the position of an expression in a list. Returns a 0 if the expression is not in the list. Returns Unknown (?) if either expression or list is Unknown (?).
 */
FUNCTION LOOKUP RETURNS INTEGER (
    INPUT expression AS CHARACTER,
    INPUT list AS CHARACTER,
    INPUT delimiter AS CHARACTER
  ) FORWARD.

FUNCTION LOOKUP RETURNS INTEGER (
    INPUT expression AS CHARACTER,
    INPUT list AS CHARACTER
  ) FORWARD.
