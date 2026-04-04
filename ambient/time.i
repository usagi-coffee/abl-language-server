/**
 * Returns an INTEGER value representing the time as the number of seconds since midnight. Use this function together with the STRING function to produce the time in hours, minutes, and seconds.
 *
 * Syntax
 *
 * `TIME`
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE hour AS INTEGER NO-UNDO.
 * DEFINE VARIABLE minute AS INTEGER NO-UNDO.
 * DEFINE VARIABLE sec AS INTEGER NO-UNDO.
 * DEFINE VARIABLE timeleft AS INTEGER NO-UNDO.
 * timeleft = (24 * 60 * 60) - TIME.
 * sec = timeleft MOD 60.
 * timeleft = (timeleft - sec) / 60.
 * minute = timeleft MOD 60.
 * hour = (timeleft - minute) / 60.
 * DISPLAY "Time to midnight:" hour minute sec.
 * DISPLAY STRING(TIME, "HH:MM:SS").
 * ```
 *
 * @returns Returns an INTEGER value representing the time as the number of seconds since midnight.
 */
FUNCTION TIME RETURNS INTEGER (
  ) FORWARD.
