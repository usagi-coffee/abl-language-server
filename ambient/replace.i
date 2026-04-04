/**
 * Returns a string with specified substring replacements.
 *
 * Syntax
 *
 * `REPLACE ( source-string , from-string , to-string )`
 *
 * Notes
 * - The REPLACE function replaces all occurrences of from-string within source-string. After replacing a substring, the REPLACE function resumes searching the string after the inserted text. Thus, the inserted text is not recursively searched (in whole or in part) for from-string.
 * - The search for occurrences of from-string within source-string is not case sensitive, unless one of the three values used in the function (source-string, to-string, or from-string) is a case-sensitive field or variable.
 * - If any of the arguments to the REPLACE function evaluate to the Unknown value (?), the function returns the Unknown value (?).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE greeting AS CHARACTER NO-UNDO FORMAT "x(40)"
 *     INITIAL "Starting user's session . . . ".
 * IF USERID("DICTDB") <> "" THEN
 *     greeting = REPLACE(greeting, "user", USERID("DICTDB")).
 * DISPLAY greeting WITH NO-LABELS.
 * ```
 *
 * @param source-string Specifies the base string to make replacements in.
 * @param from-string Specifies the substring to replace.
 * @param to-string Specifies the replacement substring.
 * @returns Returns a string with specified substring replacements. The data type of the returned value matches the data type of the expression passed to the function.
 */
FUNCTION REPLACE RETURNS CHARACTER (
    INPUT source-string AS CHARACTER,
    INPUT from-string AS CHARACTER,
    INPUT to-string AS CHARACTER
  ) FORWARD.
