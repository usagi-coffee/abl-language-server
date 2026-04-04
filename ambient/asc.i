/**
 * Converts a character expression representing a single character into the corresponding ASCII (or internal code page) value, returned as an INTEGER.
 *
 * Syntax
 *
 * `ASC ( expression [, target-codepage [ , source-codepage ] ] )`
 *
 * Notes
 * - The ASC function returns the corresponding value in the specified character set. By default, the value of SESSION:CHARSET is iso8859-1. You can set a different internal code page by specifying the Internal Code Page (-cpinternal) parameter.
 * - The ASC function is both double-byte and Unicode enabled. If the expression argument yields a double-byte character or multi-byte Unicode character in the target code page, this function can return a value greater than 255.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * DEFINE VARIABLE ltrl AS INTEGER NO-UNDO EXTENT 27.
 * FOR EACH Customer NO-LOCK:
 * ix = ASC(SUBSTRING(Customer.Name,1,1)).
 * IF ix < ASC("A") or ix > ASC("Z") THEN ix = EXTENT(ltrl).
 * ELSE ix = ix - ASC("A") + 1.
 * ltrl[ix] = ltrl[ix] + 1.
 * END.
 * ```
 *
 * @param expression An expression with a value of a single character that you want to convert to an ASCII (or internal code page) integer value.
 * @param target-codepage A character-string expression that evaluates to the name of a code page.
 * @param source-codepage A character-string expression that evaluates to the name of a code page.
 * @returns INTEGER
 */
FUNCTION ASC RETURNS INTEGER (
 expression AS CHARACTER
 ) FORWARD.

FUNCTION ASC RETURNS INTEGER (
 expression AS CHARACTER,
 target-codepage AS CHARACTER
 ) FORWARD.

FUNCTION ASC RETURNS INTEGER (
 expression AS CHARACTER,
 target-codepage AS CHARACTER,
 source-codepage AS CHARACTER
 ) FORWARD.
