/**
 * Converts date and time values, or a character string, into a DATETIME value.
 *
 * Syntax
 *
 * `DATETIME ( date-expression [, mtime-expression] )`
 *
 * `DATETIME ( string )`
 *
 * `DATETIME ( month , day , year , hours , minutes [, seconds [, milliseconds]] )`
 *
 * Notes
 * - If any argument is the Unknown value (?), the result is the Unknown value (?).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE my-datetime AS DATETIME NO-UNDO.
 * /* This statement is equivalent to "my-datetime = NOW". */
 * my-datetime = DATETIME(TODAY, MTIME).
 *
 * my-datetime = DATETIME(5, 5, 2002, 7, 15, 3).
 * my-datetime = DATETIME("05-05-2002 07:15:03").
 * ```
 *
 * @param date-expression An expression whose value is a DATE.
 * @param mtime-expression An expression whose value is an integer representing the number of milliseconds since midnight.
 * @param string A character expression whose value is a string containing a DATETIME. The date portion of the string must have the format specified by the DATE-FORMAT attribute. The time portion must be in a valid time format (HH:MM:SS, and so on).
 * @param month An expression whose value is an integer from 1 to 12, inclusive.
 * @param day An expression whose value is an integer from 1 to the highest valid day of the month.
 * @param year An expression that evaluates to a year.
 * @param hours An expression whose value is an integer from 0 to 23, inclusive.
 * @param minutes An expression whose value is an integer from 0 to 59, inclusive.
 * @param seconds An expression whose value is an integer from 0 to 61, inclusive. The upper limit is 61 for leap seconds.
 * @param milliseconds An expression whose value is an integer from 0 to 999, inclusive.
 * @returns DATETIME
 */
FUNCTION DATETIME RETURNS DATETIME (
 INPUT date-expression AS DATE,
 INPUT mtime-expression AS INTEGER
 ) FORWARD.

FUNCTION DATETIME RETURNS DATETIME (
 INPUT string AS CHARACTER
 ) FORWARD.

FUNCTION DATETIME RETURNS DATETIME (
 INPUT month AS INTEGER,
 INPUT day AS INTEGER,
 INPUT year AS INTEGER,
 INPUT hours AS INTEGER,
 INPUT minutes AS INTEGER,
 INPUT seconds AS INTEGER,
 INPUT milliseconds AS INTEGER
 ) FORWARD.
