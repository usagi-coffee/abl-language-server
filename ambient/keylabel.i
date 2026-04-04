/**
 * Evaluates a key code and returns a character string that is the predefined keyboard label for that key.
 *
 * Syntax
 *
 * `KEYLABEL ( key-code )`
 *
 * Example
 *
 * ```abl
 * DISPLAY "Press the " + KBLABEL("GO") + " key to leave procedure"
 * FORMAT "x(50)".
 * REPEAT:
 * READKEY.
 * HIDE MESSAGE.
 * IF LASTKEY = KEYCODE(KBLABEL("GO")) THEN RETURN.
 * MESSAGE "Sorry, you pressed the" KEYLABEL(LASTKEY) "key.".
 * END.
 * ```
 *
 * @param key-code The key code of the key whose label you want to know.
 * @returns A character string that is the predefined keyboard label for that key.
 */
FUNCTION KEYLABEL RETURNS CHARACTER (
    INPUT key-code AS INTEGER
  ) FORWARD.
