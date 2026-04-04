/**
 * Evaluates a key label for a key in the predefined set of keyboard keys and returns the corresponding key code as an INTEGER value.
 *
 * Syntax
 *
 * `KEYCODE ( key-label )`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 *
 * @param key-label A constant, field name, variable name, or expression that evaluates to a character string that contains a key label. If key-label is a constant, enclose it in quotation marks.
 * @returns The corresponding key code as an INTEGER value.
 */
FUNCTION KEYCODE RETURNS INTEGER (
    INPUT key-label AS CHARACTER
  ) FORWARD.
