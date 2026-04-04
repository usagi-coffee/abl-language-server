/**
 * The GET-COLLATIONS function returns a comma-delimited list of the collations
 * either listed in convmap.cp or specified by the Conversion Map (-convmap)
 * startup parameter for the specified code page.
 *
 * Syntax
 *
 * `GET-COLLATIONS ( codepage )`
 *
 * @param codepage A code page name.
 * @returns A comma-delimited list of collations. If there are no collations
 * for the specified code page, returns the Unknown value (?).
 */
FUNCTION GET-COLLATIONS RETURNS CHARACTER (
 codepage AS CHARACTER
 ) FORWARD.
