/**
 * Returns the current system date, time, and time zone as a DATETIME-TZ value.
 * The NOW function returns the system date and time of the client or server machine that serves as the time
 * source for applications running during the ABL session (specified by the TIME-SOURCE attribute).
 *
 * Syntax
 *
 * `NOW`
 *
 * Notes
 * - NOW returns a DATETIME-TZ value containing the current date, time, and time zone.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE v-datetime AS DATETIME NO-UNDO.
 * DEFINE VARIABLE v-datetime-tz AS DATETIME-TZ NO-UNDO.
 * ASSIGN
 * v-datetime = NOW
 * v-datetime-tz = NOW.
 * ```
 *
 * @returns DATETIME-TZ The current system date, time, and time zone.
 */
FUNCTION NOW RETURNS DATETIME-TZ (
 ) FORWARD.
