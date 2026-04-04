/**
 * Returns a character string that is made up of a base string plus the substitution of arguments in the string.
 *
 * Syntax
 *
 * `SUBSTITUTE ( base-string [, arg ]... )`
 *
 * Notes
 * - The function is double-byte enabled. The specified base-string and arg values can contain double-byte characters.
 * - To include an ampersand character in base-string, enter two ampersands (&&).
 * - The character following the ampersand character must be a digit, or the AVM returns a run-time error.
 * - If you use a substitution parameter in base-string but do not specify a corresponding argument, the AVM replaces the substitution parameter with an empty string.
 * - The SUBSTITUTE function converts Unknown value (?) parameters into empty strings.
 * - If the length of the resulting substitution exceeds the maximum length for the data type of the original string, an error occurs.
 * - Any substitution parameter can appear multiple times in base string.
 *
 * Example
 *
 * ```abl
 * MESSAGE SUBSTITUTE("There were &1 records in &2 tables", rec-count, table-count).
 * ```
 *
 * @param base-string A CHARACTER or LONGCHAR variable optionally containing substitution parameters of the form &n, where n is an integer between 1 and 9, inclusive.
 * @param arg A constant, field name, variable, or expression that results in a CHARACTER or LONGCHAR value. These argument values replace substitution parameters in base-string.
 * @returns Returns a character string that is made up of a base string plus the substitution of arguments in the string.
 */
FUNCTION SUBSTITUTE RETURNS CHARACTER (
    INPUT base-string AS CHARACTER,
    INPUT arg1 AS CHARACTER,
    INPUT arg2 AS CHARACTER,
    INPUT arg3 AS CHARACTER,
    INPUT arg4 AS CHARACTER,
    INPUT arg5 AS CHARACTER,
    INPUT arg6 AS CHARACTER,
    INPUT arg7 AS CHARACTER,
    INPUT arg8 AS CHARACTER,
    INPUT arg9 AS CHARACTER
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS CHARACTER (
    INPUT base-string AS CHARACTER,
    INPUT arg1 AS CHARACTER,
    INPUT arg2 AS CHARACTER,
    INPUT arg3 AS CHARACTER,
    INPUT arg4 AS CHARACTER,
    INPUT arg5 AS CHARACTER,
    INPUT arg6 AS CHARACTER,
    INPUT arg7 AS CHARACTER,
    INPUT arg8 AS CHARACTER
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS CHARACTER (
    INPUT base-string AS CHARACTER,
    INPUT arg1 AS CHARACTER,
    INPUT arg2 AS CHARACTER,
    INPUT arg3 AS CHARACTER,
    INPUT arg4 AS CHARACTER,
    INPUT arg5 AS CHARACTER,
    INPUT arg6 AS CHARACTER,
    INPUT arg7 AS CHARACTER
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS CHARACTER (
    INPUT base-string AS CHARACTER,
    INPUT arg1 AS CHARACTER,
    INPUT arg2 AS CHARACTER,
    INPUT arg3 AS CHARACTER,
    INPUT arg4 AS CHARACTER,
    INPUT arg5 AS CHARACTER,
    INPUT arg6 AS CHARACTER
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS CHARACTER (
    INPUT base-string AS CHARACTER,
    INPUT arg1 AS CHARACTER,
    INPUT arg2 AS CHARACTER,
    INPUT arg3 AS CHARACTER,
    INPUT arg4 AS CHARACTER,
    INPUT arg5 AS CHARACTER
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS CHARACTER (
    INPUT base-string AS CHARACTER,
    INPUT arg1 AS CHARACTER,
    INPUT arg2 AS CHARACTER,
    INPUT arg3 AS CHARACTER,
    INPUT arg4 AS CHARACTER
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS CHARACTER (
    INPUT base-string AS CHARACTER,
    INPUT arg1 AS CHARACTER,
    INPUT arg2 AS CHARACTER,
    INPUT arg3 AS CHARACTER
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS CHARACTER (
    INPUT base-string AS CHARACTER,
    INPUT arg1 AS CHARACTER,
    INPUT arg2 AS CHARACTER
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS CHARACTER (
    INPUT base-string AS CHARACTER,
    INPUT arg1 AS CHARACTER
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS CHARACTER (
    INPUT base-string AS CHARACTER
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS LONGCHAR (
    INPUT base-string AS LONGCHAR,
    INPUT arg1 AS LONGCHAR,
    INPUT arg2 AS LONGCHAR,
    INPUT arg3 AS LONGCHAR,
    INPUT arg4 AS LONGCHAR,
    INPUT arg5 AS LONGCHAR,
    INPUT arg6 AS LONGCHAR,
    INPUT arg7 AS LONGCHAR,
    INPUT arg8 AS LONGCHAR,
    INPUT arg9 AS LONGCHAR
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS LONGCHAR (
    INPUT base-string AS LONGCHAR,
    INPUT arg1 AS LONGCHAR,
    INPUT arg2 AS LONGCHAR,
    INPUT arg3 AS LONGCHAR,
    INPUT arg4 AS LONGCHAR,
    INPUT arg5 AS LONGCHAR,
    INPUT arg6 AS LONGCHAR,
    INPUT arg7 AS LONGCHAR,
    INPUT arg8 AS LONGCHAR
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS LONGCHAR (
    INPUT base-string AS LONGCHAR,
    INPUT arg1 AS LONGCHAR,
    INPUT arg2 AS LONGCHAR,
    INPUT arg3 AS LONGCHAR,
    INPUT arg4 AS LONGCHAR,
    INPUT arg5 AS LONGCHAR,
    INPUT arg6 AS LONGCHAR,
    INPUT arg7 AS LONGCHAR
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS LONGCHAR (
    INPUT base-string AS LONGCHAR,
    INPUT arg1 AS LONGCHAR,
    INPUT arg2 AS LONGCHAR,
    INPUT arg3 AS LONGCHAR,
    INPUT arg4 AS LONGCHAR,
    INPUT arg5 AS LONGCHAR,
    INPUT arg6 AS LONGCHAR
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS LONGCHAR (
    INPUT base-string AS LONGCHAR,
    INPUT arg1 AS LONGCHAR,
    INPUT arg2 AS LONGCHAR,
    INPUT arg3 AS LONGCHAR,
    INPUT arg4 AS LONGCHAR,
    INPUT arg5 AS LONGCHAR
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS LONGCHAR (
    INPUT base-string AS LONGCHAR,
    INPUT arg1 AS LONGCHAR,
    INPUT arg2 AS LONGCHAR,
    INPUT arg3 AS LONGCHAR,
    INPUT arg4 AS LONGCHAR
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS LONGCHAR (
    INPUT base-string AS LONGCHAR,
    INPUT arg1 AS LONGCHAR,
    INPUT arg2 AS LONGCHAR,
    INPUT arg3 AS LONGCHAR
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS LONGCHAR (
    INPUT base-string AS LONGCHAR,
    INPUT arg1 AS LONGCHAR,
    INPUT arg2 AS LONGCHAR
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS LONGCHAR (
    INPUT base-string AS LONGCHAR,
    INPUT arg1 AS LONGCHAR
  ) FORWARD.

FUNCTION SUBSTITUTE RETURNS LONGCHAR (
    INPUT base-string AS LONGCHAR
  ) FORWARD.
