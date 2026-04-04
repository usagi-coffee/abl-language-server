/**
 * Returns an INTEGER value that indicates the position of a search string within a source string.
 *
 * Syntax
 *
 * `INDEX ( source , searchString [ , starting ] )`
 *
 * Notes
 * - If source is a CHARACTER or LONGCHAR expression and either operand is case sensitive, the search is case sensitive.
 * - If source is a MEMPTR, case sensitivity depends on the searchString. If searchString is case sensitive, the search is case sensitive.
 * - If searchString is an empty string ("") or a null string ("?"), the function returns 0.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE pos AS INTEGER NO-UNDO.
 * pos = INDEX("banana", "na").
 * ```
 *
 * @param source A CHARACTER, LONGCHAR or MEMPTR expression.
 * @param search-string A CHARACTER or LONGCHAR expression whose position you want to locate in source.
 * @param starting An integer that specifies at which left-most position in the string to start the search.
 * @returns Returns an INTEGER value that indicates the position of a search string within a source string.
 */
FUNCTION INDEX RETURNS INTEGER (
    INPUT source AS CHARACTER,
    INPUT search-string AS CHARACTER,
    INPUT starting AS INTEGER
  ) FORWARD.

FUNCTION INDEX RETURNS INTEGER (
    INPUT source AS CHARACTER,
    INPUT search-string AS CHARACTER
  ) FORWARD.
