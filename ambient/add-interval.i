/**
 * Adds a time interval to, or subtracts a time interval from, a DATE, DATETIME, or DATETIME-TZ value, and
 * returns the new value.
 *
 * Syntax
 *
 * `ADD-INTERVAL (datetime, interval-amount, interval-unit)`
 *
 * Notes
 * - To add or subtract months or years, this function converts the date to Gregorian before adding or subtracting
 * the year or month value. If the result is an invalid date, the function decrements the day part of the date until
 * a valid date is obtained. For example:
 * - Adding 1 month to January 30, 2003 yields February 28, 2003
 * - Adding 13 months to January 30, 2003 yields February 29, 2004 (2004 is a leap year)
 * - Subtracting 1 month from December 31, 2003 yields November 30, 2003
 *
 * @param datetime An expression whose value is a DATE, DATETIME, or DATETIME-TZ.
 * @param interval-amount A signed integer (positive or negative) indicating the amount of time you want to add to or subtract from datetime value.
 * @param interval-unit A character constant, or a character expression that evaluates to one of the following time units: 'years', 'months', 'weeks', 'days', 'hours', 'minutes', 'seconds' or 'milliseconds'. These values are case insensitive and may be singular.
 * @returns The new DATE, DATETIME, or DATETIME-TZ value.
 */
FUNCTION ADD-INTERVAL RETURNS DATE (
 datetime AS DATE,
 interval-amount AS INTEGER,
 interval-unit AS CHARACTER
 ) FORWARD.

FUNCTION ADD-INTERVAL RETURNS DATETIME (
 datetime AS DATETIME,
 interval-amount AS INTEGER,
 interval-unit AS CHARACTER
 ) FORWARD.

FUNCTION ADD-INTERVAL RETURNS DATETIME-TZ (
 datetime AS DATETIME-TZ,
 interval-amount AS INTEGER,
 interval-unit AS CHARACTER
 ) FORWARD.
