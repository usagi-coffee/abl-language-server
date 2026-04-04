/**
 * Returns an INTEGER value representing the time in milliseconds.
 * If the MTIME function has no arguments, it returns the current number of milliseconds
 * since midnight (similar to TIME, which returns seconds since midnight).
 *
 * Syntax
 *
 * `MTIME`
 *
 * `MTIME ( datetime-expression )`
 *
 * Notes
 * - The MTIME function gets the current system time of the client or server machine that
 * serves as the time source for applications running during the ABL session (specified
 * by the TIME-SOURCE attribute).
 * - If datetime-expression is a DATETIME-TZ, the MTIME function returns the local time
 * relative to the time zone of the DATETIME-TZ value. For example, a DATETIME-TZ field
 * created in London (time zone UTC+00:00) with a value of May 5, 2002 at 7:15:03.002 am
 * returns 26,103,002, regardless of the session's time zone.
 *
 * @returns INTEGER representing the time in milliseconds.
 */
FUNCTION MTIME RETURNS INTEGER (
 )
 FORWARD.

/**
 * Returns an INTEGER value representing the time in milliseconds.
 *
 * Syntax
 *
 * `MTIME ( datetime-expression )`
 *
 * @param datetime-expression An expression that evaluates to a DATETIME or DATETIME-TZ.
 * The MTIME function returns the time portion of datetime-expression in milliseconds.
 * @returns INTEGER representing the time in milliseconds.
 */
FUNCTION MTIME RETURNS INTEGER (
 INPUT datetime1 AS DATETIME
 )
 FORWARD.

/**
 * Returns an INTEGER value representing the time in milliseconds.
 *
 * Syntax
 *
 * `MTIME ( datetime-expression )`
 *
 * @param datetime-tz-expression An expression that evaluates to a DATETIME-TZ.
 * The MTIME function returns the time portion of datetime-expression in milliseconds.
 * @returns INTEGER representing the time in milliseconds.
 */
FUNCTION MTIME RETURNS INTEGER (
 INPUT datetimetz1 AS DATETIME-TZ
 )
 FORWARD.
