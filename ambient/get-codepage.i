/**
 * The GET-CODEPAGE function returns the code page of a LONGCHAR variable or CLOB field.
 *
 * Syntax
 *
 * `GET-CODEPAGE ( large-char-object )`
 *
 * @param large-char-object The name of a LONGCHAR variable or CLOB field. If the specified LONGCHAR is empty and the code page was not fixed using the FIX-CODEPAGE statement, the AVM returns the Unknown value (?).
 * @returns CHARACTER - The code page of the specified LONGCHAR variable or CLOB field, or the Unknown value (?) if the LONGCHAR is empty and the code page was not fixed.
 */
FUNCTION GET-CODEPAGE RETURNS CHARACTER (
 large-char-object AS LONGCHAR
 ) FORWARD.
