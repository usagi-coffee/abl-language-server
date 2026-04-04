/**
 * The IS-CODEPAGE-FIXED function returns TRUE if the code page of the specified LONGCHAR variable is fixed; otherwise it returns FALSE.
 *
 * Syntax
 *
 * `IS-CODEPAGE-FIXED ( longchar )`
 *
 * @param longchar The name of a LONGCHAR variable.
 * @returns LOGICAL - TRUE if the code page is fixed, FALSE otherwise.
 */
FUNCTION IS-CODEPAGE-FIXED RETURNS LOGICAL (
 longchar AS LONGCHAR
 ) FORWARD.
