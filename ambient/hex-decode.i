/**
 * Converts a string consisting of an even number of hexadecimal digits into a MEMPTR value.
 *
 * Syntax
 *
 * `HEX-DECODE( hex-string )`
 *
 * Notes
 * - If the expression does not contain an even number of hexadecimal digits, or it is the Unknown value (?), the result is the Unknown value (?).
 * - If the expression is a zero-length value, the result is a zero-length value.
 * - The BASE64-ENCODE, BASE64-DECODE, HEX-ENCODE, and HEX-DECODE functions do not do any byte ordering. However, the binary output of these functions may be a binary data type such as a MEMPTR or RAW. If the target of these operation's output is another system where the order of bytes (endianness) differs, you must first normalize the data to what the receiving side is expecting.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE vMemptr as MEMPTR NO-UNDO.
 * vMemptr = HEX-DECODE(HEX-ENCODE(GENERATE-UUID)).
 * ```
 *
 * @param hex-string A CHAR or LONGCHAR expression containing the hexadecimal value to convert.
 * @returns A MEMPTR containing the decoded binary value.
 */
FUNCTION HEX-DECODE RETURNS MEMPTR (
    INPUT hex-string AS CHARACTER
  ) FORWARD.
