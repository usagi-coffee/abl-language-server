/**
 * Converts an integer value to its corresponding character value.
 *
 * Syntax
 *
 * `CHR ( expression [, target-codepage [, source-codepage]] )`
 *
 * Notes
 * - The CHR function returns the corresponding character in the specified code page. By default, the value of SESSION:CHARSET is iso8859-1. You can set a different internal code page by specifying the Internal Code Page (-cpinternal) parameter.
 * - The CHR function is double-byte enabled. For a value greater than 255 and less than 65535, it checks for a lead-byte value. If the lead-byte value is valid, the AVM creates and returns a double-byte character.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * DEFINE VARIABLE letter AS CHARACTER NO-UNDO FORMAT "X(1)" EXTENT 26.
 * DO ix = 1 TO 26:
 * letter[ix] = CHR((ASC("A")) - 1 + ix).
 * END.
 * ```
 *
 * @param expression An expression that yields an integer value that you want to convert to a character value. If the value of expression is in the range of 1 to 255, CHR returns a single character.
 * @param target-codepage A character-string expression that evaluates to the name of a code page. The name that you specify must be a valid code page name available in the DLC/convmap.cp file.
 * @param source-codepage A character-string expression that evaluates to the name of a code page. The name that you specify must be a valid code page name available in the DLC/convmap.cp file. The default value of source-codepage is the value of SESSION:CHARSET.
 * @returns CHARACTER
 */
FUNCTION CHR RETURNS CHARACTER (
 INPUT expression AS INTEGER,
 INPUT target-codepage AS CHARACTER,
 INPUT source-codepage AS CHARACTER
 ) FORWARD.

/**
 * Converts an integer value to its corresponding character value.
 *
 * Syntax
 *
 * `CHR ( expression [, target-codepage] )`
 *
 * @param expression An expression that yields an integer value that you want to convert to a character value.
 * @param target-codepage A character-string expression that evaluates to the name of a code page.
 * @returns CHARACTER
 */
FUNCTION CHR RETURNS CHARACTER (
 INPUT expression AS INTEGER,
 INPUT target-codepage AS CHARACTER
 ) FORWARD.

/**
 * Converts an integer value to its corresponding character value.
 *
 * Syntax
 *
 * `CHR ( expression )`
 *
 * @param expression An expression that yields an integer value that you want to convert to a character value.
 * @returns CHARACTER
 */
FUNCTION CHR RETURNS CHARACTER (
 INPUT expression AS INTEGER
 ) FORWARD.
