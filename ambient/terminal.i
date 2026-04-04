/**
 * Returns the terminal type.
 *
 * Syntax
 *
 * `TERMINAL`
 *
 * Notes
 * - In Windows, in graphical interfaces, TERMINAL returns WIN3.
 * - In Windows, in character interfaces, TERMINAL returns CO80, BW80, or MONO, depending on the monitor type.
 * - On UNIX, TERMINAL returns the value of the $TERM environment variable.
 * - In batch mode, TERMINAL returns a null string.
 * - Does not apply to SpeedScript programming.
 *
 * Example
 *
 * ```abl
 * MESSAGE "You are currently using a" TERMINAL "terminal.".
 * ```
 *
 * @returns Returns the terminal type as a character string.
 */
FUNCTION TERMINAL RETURNS CHARACTER (
  ) FORWARD.
