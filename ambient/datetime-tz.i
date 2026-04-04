/**
 * Converts a date, time, and time zone value, or a character string, into a DATETIME-TZ value.
 *
 * Syntax
 *
 * `DATETIME-TZ ( date-exp , mtime-exp , timezone-exp )`
 *
 * `DATETIME-TZ ( datetime-exp , timezone-exp )`
 *
 * `DATETIME-TZ ( datetime-tz-exp , timezone-exp )`
 *
 * `DATETIME-TZ ( month , day , year , hours , minutes , seconds , milliseconds , timezone-exp )`
 *
 * `DATETIME-TZ ( string )`
 *
 * Notes
 * - If any argument is the Unknown value (?), the result is the Unknown value (?).
 * - The date-exp is an expression whose value is a DATE.
 * - The mtime-exp is an expression whose value is an integer representing the number of milliseconds since midnight.
 * - The timezone-exp is an expression whose value is an integer representing the time zone offset from Coordinated Universal Time (UTC) in minutes. If not specified the function uses the ABL session's time zone.
 * - The datetime-exp is an expression whose value is a DATETIME.
 * - The datetime-tz-exp is an expression whose value is a DATETIME-TZ. If the timezone-exp is not specified, then the datetime-tz-exp value is returned as is. If the timezone-exp is specified, then the function returns the datetime-tz-exp value in the timezone-exp time zone.
 * - The month is an expression whose value is an integer from 1 to 12, inclusive.
 * - The day is an expression whose value is an integer from 1 to the highest valid day of the month.
 * - The year is an expression that evaluates to a year.
 * - The hours is an expression whose value is an integer from 0 to 23, inclusive.
 * - The minutes is an expression whose value is an integer from 0 to 59, inclusive.
 * - The seconds is an expression whose value is an integer from 0 to 61, inclusive. The upper limit is 61 for leap seconds.
 * - The milliseconds is an expression whose value is an integer from 0 to 999, inclusive.
 * - The string is a character expression whose value is a string containing a DATETIME-TZ. The date portion of the string must have the format specified by the DATE-FORMAT attribute. The time portion must be in a valid time format (HH:MM:SS, and so on). If the string contains a time zone, it must be in +HH:MM format. If the string does not contain a time zone, the DATETIME-TZ inherits the time zone of the machine running the ABL session.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE my-datetime-tz AS DATETIME-TZ NO-UNDO.
 * /* This statement is equivalent to "my-datetime-tz = NOW". */
 * my-datetime-tz = DATETIME-TZ(TODAY, MTIME, TIMEZONE).
 *
 * /* The following statements result in the same DATETIME-TZ value (when SESSION:DATE-FORMAT is mdy): */
 * my-datetime-tz = DATETIME-TZ(5, 5, 2002, 7, 15, 3, 0, -300).
 * my-datetime-tz = DATETIME-TZ("05-05-2002 07:15:03-05:00").
 * ```
 *
 * @param date-exp An expression whose value is a DATE
 * @param mtime-exp An expression whose value is an integer representing the number of milliseconds since midnight
 * @param timezone-exp An expression whose value is an integer representing the time zone offset from UTC in minutes
 * @returns A DATETIME-TZ value
 */
FUNCTION DATETIME-TZ RETURNS DATETIME-TZ (
 date-exp AS DATE,
 mtime-exp AS INTEGER,
 timezone-exp AS INTEGER
 ) FORWARD.

FUNCTION DATETIME-TZ RETURNS DATETIME-TZ (
 datetime-exp AS DATETIME,
 timezone-exp AS INTEGER
 ) FORWARD.

FUNCTION DATETIME-TZ RETURNS DATETIME-TZ (
 datetime-tz-exp AS DATETIME-TZ,
 timezone-exp AS INTEGER
 ) FORWARD.

FUNCTION DATETIME-TZ RETURNS DATETIME-TZ (
 month AS INTEGER,
 day AS INTEGER,
 year AS INTEGER,
 hours AS INTEGER,
 minutes AS INTEGER,
 seconds AS INTEGER,
 milliseconds AS INTEGER,
 timezone-exp AS INTEGER
 ) FORWARD.

FUNCTION DATETIME-TZ RETURNS DATETIME-TZ (
 string AS CHARACTER
 ) FORWARD.
