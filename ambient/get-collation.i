/**
 * The GET-COLLATION function returns the collation name for a CLOB field.
 *
 * Syntax
 *
 * `GET-COLLATION ( clob-field )`
 *
 * @param clob-field A CLOB field name.
 * @returns The collation name for the CLOB field.
 */
FUNCTION GET-COLLATION RETURNS CHARACTER (
 clob-field AS CLOB
 ) FORWARD.
