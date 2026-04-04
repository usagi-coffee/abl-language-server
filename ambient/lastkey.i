/**
 * Returns the integer key code of the most recent event read from the user.
 *
 * Syntax
 *
 * `LASTKEY`
 *
 * Notes
 * - The LASTKEY function is double-byte enabled. It returns values only after the input method places the data in the keyboard buffer. It returns the key code of the most recent key sequence returned from the keyboard buffer.
 * - If you used a READKEY statement that timed out (you specified a number of seconds by using the PAUSE option with the READKEY statement), or if a PAUSE statement times out, the value of LASTKEY is -1.
 * - If you use the PAUSE option with the READKEY statement, the value of LASTKEY is the key you press to end the PAUSE.
 * - When the ABL session starts, the value of LASTKEY is -1. This value remains the same until the first input, READKEY, or procedure pause occurs. The LASTKEY function is reset to -1 each time you return to the Procedure Editor.
 * - If you read data from a file, LASTKEY is set to the last character read from the file. For an INSERT, PROMPT-FOR, SET or UPDATE statement, this is always KEYCODE("RETURN"). For a READKEY statement, this is the character read from the file. If you reach past the end of the file, LASTKEY is -2.
 *
 * Example
 *
 * ```abl
 * FIND FIRST Customer.
 * REPEAT:
 *   UPDATE Customer.CustNum Customer.Name GO-ON(F9 F10).
 *   IF LASTKEY = KEYCODE("F9") THEN UNDO, RETRY.
 *   ELSE IF LASTKEY = KEYCODE("F10") THEN FIND NEXT Customer.
 * END.
 * ```
 *
 * @returns The integer key code of the most recent event read from the user (from the keyboard or mouse).
 */
FUNCTION LASTKEY RETURNS INTEGER (
  ) FORWARD.
