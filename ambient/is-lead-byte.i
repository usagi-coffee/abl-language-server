/**
 * Returns TRUE if the first character of the string is the lead-byte of a multi-byte character.
 *
 * Syntax
 *
 * `IS-LEAD-BYTE ( string )`
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE result AS LOGICAL NO-UNDO.
 * result = IS-LEAD-BYTE("あ").
 * ```
 *
 * @param string A character expression whose value is a character.
 * @returns TRUE if the first character is the lead-byte of a multi-byte character, FALSE otherwise.
 */
FUNCTION IS-LEAD-BYTE RETURNS LOGICAL (
    INPUT string AS CHARACTER
  ) FORWARD.
