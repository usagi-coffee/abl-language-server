/**
 * Converts a single character string, a set of month, day, and year values, an integer expression, a DATETIME
 * expression, or a DATETIME-TZ expression into a DATE value.
 * If the DATE function cannot produce a valid date given the specified argument(s), it returns a run-time error.
 *
 * Syntax
 *
 * `DATE ( month , day , year )`
 *
 * `DATE ( string )`
 *
 * `DATE ( integer-expression )`
 *
 * `DATE ( datetime-expression )`
 *
 * Notes
 * - The resulting date from the DATE(integer-expression) function is guaranteed to be a
 * valid ABL date only if the integer-expression originated from the INTEGER(ABL-date) or
 * INT64(ABL-date) function.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ddate AS DATE NO-UNDO.
 * ddate = DATE(1, 15, 2024).
 * ddate = DATE("01/15/2024").
 * ```
 *
 * @param month A constant, field name, variable name, or expression whose value is an integer from 1 to 12
 * @param day An expression whose value is an integer from 1 to the highest valid day of the month
 * @param year An expression whose value is the year
 * @returns A DATE value
 */
FUNCTION DATE RETURNS DATE (
 INPUT month AS INTEGER,
 INPUT day AS INTEGER,
 INPUT year AS INTEGER
 ) FORWARD.

FUNCTION DATE RETURNS DATE (
 INPUT string AS CHARACTER
 ) FORWARD.

FUNCTION DATE RETURNS DATE (
 INPUT integer-expression AS INTEGER
 ) FORWARD.

FUNCTION DATE RETURNS DATE (
 INPUT datetime-expression AS DATETIME
 ) FORWARD.
