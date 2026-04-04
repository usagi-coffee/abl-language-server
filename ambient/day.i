/**
 * Evaluates a date expression and returns a day of the month as an INTEGER value from 1 to 31, inclusive.
 *
 * Syntax
 *
 * `DAY ( date )`
 *
 * `DAY ( datetime-expression )`
 *
 * @param date An expression whose value is a DATE.
 * @returns An INTEGER value from 1 to 31 representing the day of the month.
 */
FUNCTION DAY RETURNS INTEGER (
 INPUT date1 AS DATE
 ) FORWARD.

/**
 * Evaluates a date expression and returns a day of the month as an INTEGER value from 1 to 31, inclusive.
 *
 * Syntax
 *
 * `DAY ( datetime-expression )`
 *
 * @param datetime-expression An expression that evaluates to a DATETIME or DATETIME-TZ.
 * @returns An INTEGER value from 1 to 31 representing the day of the month.
 */
FUNCTION DAY RETURNS INTEGER (
 INPUT datetime1 AS DATETIME
 ) FORWARD.

/**
 * Evaluates a date expression and returns a day of the month as an INTEGER value from 1 to 31, inclusive.
 *
 * Syntax
 *
 * `DAY ( datetime-expression )`
 *
 * @param datetime-tz-expression An expression that evaluates to a DATETIME-TZ.
 * @returns An INTEGER value from 1 to 31 representing the day of the month.
 */
FUNCTION DAY RETURNS INTEGER (
 INPUT datetimetz1 AS DATETIME-TZ
 ) FORWARD.
