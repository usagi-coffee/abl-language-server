/**
 * Description
 * Returns an INTEGER value that indicates the position of the target string within the source string.
 * In contrast to the INDEX function, R-INDEX performs the search from right to left.
 *
 * Syntax
 *
 * `R-INDEX ( source , target [ , starting ] )`
 *
 * Notes
 * - If either operand is case sensitive, then the R-INDEX function is also case sensitive.
 * - If either the source string or target pattern is null, the result is 0.
 * - The R-INDEX function is double-byte enabled.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE pos AS INTEGER NO-UNDO.
 * pos = R-INDEX("banana", "na").
 * ```
 *
 * @param source-string The string to search in.
 * @param target-string The substring to search for.
 * @param start-position The 1-based position to start searching from.
 * @returns Returns a value that indicates the position of the target string within the source string.
 */
FUNCTION R-INDEX RETURNS INTEGER (
 source-string AS CHARACTER,
 target-string AS CHARACTER,
 start-position AS INTEGER
 ) FORWARD.

FUNCTION R-INDEX RETURNS INTEGER (
 source-string AS CHARACTER,
 target-string AS CHARACTER
 ) FORWARD.
