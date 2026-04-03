/**
 * Returns the first position of `target` in `source`.
 *
 * Notes
 * - If either operand is case sensitive, then the search is case sensitive.
 * - If the target string is null, the result is 0.
 * - The INDEX function is double-byte enabled. You can specify target and source strings for the INDEX function that contain double-byte characters.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE pos AS INTEGER NO-UNDO.
 * pos = INDEX("banana", "na").
 * ```
 *
 * @param source-string The string to search in.
 * @param target-string The substring to search for.
 * @returns Returns a value that indicates the position of the target string within the source string.
 */
FUNCTION INDEX RETURNS INTEGER (
    source-string AS CHARACTER,
    target-string AS CHARACTER
  ) FORWARD.

/**
  * Returns the first position of `target` in `source`.
  *
  * Notes
  * - If either operand is case sensitive, then the search is case sensitive.
  * - If the target string is null, the result is 0.
  * - The INDEX function is double-byte enabled. You can specify target and source strings for the INDEX function that contain double-byte characters.
  *
  * Example
  *
  * ```abl
  * DEFINE VARIABLE pos AS INTEGER NO-UNDO.
  * pos = INDEX("banana", "na").
  * ```
  *
  * @param source-string The string to search in.
  * @param target-string The substring to search for.
  * @param start-position The 1-based position to start searching from.
  * @returns Returns a value that indicates the position of the target string within the source string.
  */
FUNCTION INDEX RETURNS INTEGER (
    source-string AS CHARACTER,
    target-string AS CHARACTER,
    start-position AS INTEGER
  ) FORWARD.
