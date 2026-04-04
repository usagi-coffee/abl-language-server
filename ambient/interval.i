/**
 * Returns the time interval between two DATE, DATETIME, or DATETIME-TZ values as an INT64 value.
 *
 * Syntax
 *
 * `INTERVAL ( datetime1 , datetime2 , interval-unit )`
 *
 * Notes
 * - This function returns a signed integer value (positive or negative). For example, if datetime1 is less than datetime2, the INTERVAL function returns a negative value.
 * - If datetime1 or datetime2 is a DATE or DATETIME, the time value defaults to midnight and the time zone value defaults to the session's time zone, respectively.
 * - You are responsible for managing value overflow, if any.
 * - When used with 'months', the INTERVAL function does not take into account that not all months have the same number of days.
 *
 * @param datetime1 An expression whose value is a DATE, DATETIME, or DATETIME-TZ.
 * @param datetime2 An expression whose value is a DATE, DATETIME, or DATETIME-TZ.
 * @param interval-unit A character constant, or a character expression that evaluates to one of the following time units: 'years', 'months', 'weeks', 'days', 'hours', 'minutes', 'seconds' or 'milliseconds'.
 * @returns Returns the time interval as an INT64 value.
 */
FUNCTION INTERVAL RETURNS INT64 (
    INPUT datetime1 AS DATETIME-TZ,
    INPUT datetime2 AS DATETIME-TZ,
    INPUT interval-unit AS CHARACTER
  ) FORWARD.
