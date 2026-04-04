/**
 * Extracts bytes from a field. (ORACLE only)
 *
 * Syntax
 *
 * `RAW ( field [, position[, length]] )`
 *
 * Notes
 * - If position is less than 1, or length is less than 0, the AVM returns a run-time error.
 * - If (position + length - 1) is greater than the length of the field from which you are extracting the bytes, the AVM returns a run-time error.
 *
 * Example
 *
 * ```abl
 * /*You must connect to a non-OpenEdge demo database to run this procedure*/
 * DEFINE VARIABLE r1 AS RAW NO-UNDO.
 * FIND FIRST Customer NO-LOCK.
 * r1 = RAW(Customer.Name, 8, 4).
 * ```
 *
 * @param field Any field from which you want to extract bytes.
 * @param position An integer expression that indicates the position of the first byte you want to extract from field. The default value of position is 1.
 * @param length An integer expression that indicates the number of bytes you want to extract from field. If you do not use the length argument, RAW uses field from position to end.
 * @returns Returns a RAW value containing the extracted bytes.
 */
FUNCTION RAW RETURNS RAW (
    INPUT field AS RAW,
    INPUT position AS INTEGER,
    INPUT length AS INTEGER
  ) FORWARD.

FUNCTION RAW RETURNS RAW (
    INPUT field AS RAW,
    INPUT position AS INTEGER
  ) FORWARD.

FUNCTION RAW RETURNS RAW (
    INPUT field AS RAW
  ) FORWARD.
