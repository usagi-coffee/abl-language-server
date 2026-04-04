/**
 * Evaluates a date expression and returns the day of the week as an INTEGER value from 1 (Sunday) to 7 (Saturday).
 *
 * Syntax
 *
 * `WEEKDAY ( date-expression )`
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE birth-date AS DATE NO-UNDO.
 * DEFINE VARIABLE daynum AS INTEGER NO-UNDO.
 * daynum = WEEKDAY(birth-date).
 * ```
 *
 * @param date-expression A date expression for which you want the day of the week. Can be DATE, DATETIME, or DATETIME-TZ.
 * @returns An INTEGER value from 1 (Sunday) to 7 (Saturday) indicating the day of the week.
 */
FUNCTION WEEKDAY RETURNS INTEGER (
    INPUT date-expression AS DATE
  ) FORWARD.

FUNCTION WEEKDAY RETURNS INTEGER (
    INPUT date-expression AS DATETIME
  ) FORWARD.

FUNCTION WEEKDAY RETURNS INTEGER (
    INPUT date-expression AS DATETIME-TZ
  ) FORWARD.
