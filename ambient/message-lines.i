/**
 * Returns the number of lines in the message area at the bottom of the window.
 * Note: Does not apply to SpeedScript programming.
 *
 * Syntax
 *
 * `MESSAGE-LINES`
 *
 * Notes
 * - Returns, as an INTEGER value, the number of lines in the message area at the bottom of the window.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * DO ix = 1 TO MESSAGE-LINES:
 * MESSAGE "This is message line" ix.
 * END.
 * ```
 *
 * @returns INTEGER value representing the number of lines in the message area.
 */
FUNCTION MESSAGE-LINES RETURNS INTEGER (
 ) FORWARD.
