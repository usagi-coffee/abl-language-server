/**
 * Returns the number of elements in a list of character strings.
 *
 * Syntax
 *
 * `NUM-ENTRIES ( list [, character ] )`
 *
 * Notes
 * - NUM-ENTRIES returns the number of delimiters plus 1, and it returns 0 if list equals the empty string ("").
 * - The NUM-ENTRIES function is multi-byte enabled. The specified list can contain entries that have multi-byte characters and the character delimiter can be a multi-byte character.
 * - If you use an alphabetic character, this delimiter is case sensitive.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE regions AS CHARACTER NO-UNDO
 *     INITIAL "Northeast,Southeast,Midwest,Northwest,Southwest".
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * REPEAT ix = 1 TO NUM-ENTRIES(regions):
 *     DISPLAY ENTRY(ix, regions) FORMAT "x(12)".
 * END.
 * ```
 *
 * @param list A character expression containing a list of character strings separated with a character delimiter.
 * @param character A delimiter you define for the list. The default is a comma (,).
 * @returns Returns the number of elements in a list of character strings as an INTEGER value.
 */
FUNCTION NUM-ENTRIES RETURNS INTEGER (
    INPUT list AS CHARACTER,
    INPUT character AS CHARACTER
  ) FORWARD.

FUNCTION NUM-ENTRIES RETURNS INTEGER (
    INPUT list AS CHARACTER
  ) FORWARD.
