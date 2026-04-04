/**
 * Returns an INTEGER value representing the time zone offset from Coordinated Universal Time (UTC), in minutes.
 * Use this function together with the STRING function to produce the time in hours, minutes, and seconds.
 *
 * Note: Coordinated Universal Time (UTC) is the current universal standard for time. Local time zone values
 * are relative to UTC. For example, Eastern Standard Time is UTC-05:00.
 *
 * Syntax
 *
 * `TIMEZONE ( [ datetime-tz-expression | char-expression ] )`
 *
 * Notes
 * - If the TIMEZONE function has no arguments, it returns the client or server machine that serves as
 * the time source for applications running during the ABL session (specified by the TIME-SOURCE attribute).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE v-dt-tz AS DATETIME-TZ NO-UNDO
 * INITIAL 2002-05-05T07:15:03.002-05:00.
 * DEFINE VARIABLE v-tz AS INTEGER NO-UNDO.
 * ASSIGN
 * v-tz = TIMEZONE("+08:00") /* v-tz = 480 */
 * v-tz = TIMEZONE(v-dt-tz). /* v-tz = -300 */
 * ```
 *
 * @param datetime-tz-expression An expression whose value is a DATETIME-TZ.
 * @param char-expression A character expression representing the time zone offset. The format of the expression must be +HH:MM.
 * @returns INTEGER The time zone offset from UTC in minutes.
 */
FUNCTION TIMEZONE RETURNS INTEGER (
 ) FORWARD.

FUNCTION TIMEZONE RETURNS INTEGER (
 INPUT datetime-tz-expression AS DATETIME-TZ
 ) FORWARD.

FUNCTION TIMEZONE RETURNS INTEGER (
 INPUT char-expression AS CHARACTER
 ) FORWARD.
