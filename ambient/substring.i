/**
 * Extracts a portion of a character string from a field or variable.
 *
 * Syntax
 *
 * `SUBSTRING ( source , position [ , length [ , type ] ] )`
 *
 * Notes
 * - If you specify 0 as the sub-string length to retrieve, the function does not throw an error and returns an empty sub-string.
 * - If source is a LONGCHAR expression, "CHARACTER" is the only valid type and the default type.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE inv-num AS CHARACTER NO-UNDO.
 * inv-num = SUBSTRING(STRING(TODAY), 1, 2, "CHARACTER").
 * ```
 *
 * @param source-string A CHARACTER or LONGCHAR expression from which to extract characters or bytes.
 * @param position An integer expression that indicates the position of the first character to extract.
 * @param length An integer expression that indicates the number of characters to extract. If -1, uses the remainder of the string.
 * @param type A CHARACTER expression: "CHARACTER", "FIXED", "COLUMN", or "RAW".
 * @returns Returns a portion of the source string starting at the specified position.
 */
FUNCTION SUBSTRING RETURNS CHARACTER (
    INPUT source-string AS CHARACTER,
    INPUT position AS INTEGER,
    INPUT length AS INTEGER,
    INPUT type AS CHARACTER
  ) FORWARD.

FUNCTION SUBSTRING RETURNS CHARACTER (
    INPUT source-string AS CHARACTER,
    INPUT position AS INTEGER,
    INPUT length AS INTEGER
  ) FORWARD.

FUNCTION SUBSTRING RETURNS CHARACTER (
    INPUT source-string AS CHARACTER,
    INPUT position AS INTEGER
  ) FORWARD.
