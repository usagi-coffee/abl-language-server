/**
 * Returns, as an INT64 value, the time (in milliseconds) elapsed since the ABL session began or since ETIME
 * (elapsed time) was last set to 0. To set ETIME to 0, pass it a positive logical value, such as YES or TRUE.
 *
 * Syntax
 *
 * `ETIME [( logical )]`
 *
 * Notes
 * - ETIME is accurate to at least one-sixtieth of a second, but accuracy varies among systems.
 * - ABL resets ETIME during startup, not immediately after you enter the pro command. Therefore, the time
 * returned is only an approximation of the time elapsed since your session began.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE a AS INT64 NO-UNDO.
 * DO:
 * a = ETIME(yes).
 * RUN applhelp.p.
 * DISPLAY ETIME.
 * END.
 * ```
 *
 * @param logical A logical value, such as YES or TRUE. The default value is NO.
 * @returns INT64 The time (in milliseconds) elapsed since the ABL session began or since ETIME was last set to 0.
 */
FUNCTION ETIME RETURNS INT64 (
 )
 FORWARD.

FUNCTION ETIME RETURNS INT64 (
 LOGICAL
 )
 FORWARD.
