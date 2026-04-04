/**
 * Converts a string value from one code page to another.
 *
 * Syntax
 *
 * ```
 * CODEPAGE-CONVERT
 * ( source-string
 *
 * [ , target-codepage [ , source-codepage ] ]
 * )
 * ```
 *
 * Notes
 * - The CODEPAGE-CONVERT function returns the corresponding character string in the specified code page. By default, the value of SESSION:CHARSET is iso8859-1. You can set a different internal code page by specifying the Internal Code Page (-cpinternal) parameter.
 * - This function is especially useful if you plan to run a procedure in an ABL session in which the SESSION:CHARSET code page is different from the native code page of the procedure.
 * - When you write procedures with ABL, you must use 7-bit (that is, ASCII) characters for field names and variable names. But you can use 8-bit and multi-byte characters, including Unicode, for data values such as character strings and constants. Thus, a procedure written and compiled on a system using one code page can be run on a system using another code page as long as you convert all embedded character strings to the internal code page. Using CODEPAGE-CONVERT as shown in the example allows your procedures to be virtually code page independent.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE cp850string
 * AS CHARACTER NO-UNDO
 * INITIAL "text with umlaut (ä)".
 * DEFINE VARIABLE charsetstring AS CHARACTER NO-UNDO.
 * charsetstring = CODEPAGE-CONVERT(cp850string, SESSION:CHARSET, "ibm850").
 * ```
 *
 * @param source-string A CHARACTER or LONGCHAR expression to be converted.
 * @param target-codepage A character-string expression that evaluates to the name of a code page. The returned character value is relative to target-codepage. If you do not specify target-codepage, no code page conversions occur.
 * @param source-codepage A character-string expression that evaluates to the name of a code page. The source-codepage specifies the name of the code page to which source-string is relative. The default value of source-codepage is the value of CHARSET attribute of the SESSION handle. If source-string is a LONGCHAR variable, the source-codepage argument is not valid.
 * @returns The corresponding character string in the specified code page, or the Unknown value (?) if an invalid code page name is specified.
 */
FUNCTION CODEPAGE-CONVERT RETURNS CHARACTER (
 INPUT source-string AS CHARACTER,
 INPUT target-codepage AS CHARACTER,
 INPUT source-codepage AS CHARACTER
 ) FORWARD.

FUNCTION CODEPAGE-CONVERT RETURNS CHARACTER (
 INPUT source-string AS CHARACTER,
 INPUT target-codepage AS CHARACTER
 ) FORWARD.

FUNCTION CODEPAGE-CONVERT RETURNS CHARACTER (
 INPUT source-string AS CHARACTER
 ) FORWARD.

FUNCTION CODEPAGE-CONVERT RETURNS LONGCHAR (
 INPUT source-string AS LONGCHAR,
 INPUT target-codepage AS CHARACTER
 ) FORWARD.

FUNCTION CODEPAGE-CONVERT RETURNS LONGCHAR (
 INPUT source-string AS LONGCHAR
 ) FORWARD.
