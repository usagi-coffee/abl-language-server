/**
 * Converts a RAW or MEMPTR value into a string of type LONGCHAR consisting of an even number of hexadecimal digits (0 through 9 and A through F).
 *
 * Syntax
 *
 * `HEX-ENCODE( expression )`
 *
 * Notes
 * - If the expression is the Unknown value (?), the result is the Unknown value (?).
 * - If the expression is a zero-length value, the result is a zero-length value.
 * - The BASE64-ENCODE, BASE64-DECODE, HEX-ENCODE, and HEX-DECODE functions do not do any byte ordering. However, the binary output of these functions may be a binary data type such as a MEMPTR or RAW. If the target of these operation's output is another system where the order of bytes (endianness) differs, you must first normalize the data to what the receiving side is expecting.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE MyUUID AS RAW NO-UNDO.
 * DEFINE VARIABLE vChar AS LONGCHAR NO-UNDO.
 * ASSIGN
 * MyUUID = GENERATE-UUID
 * vChar = HEX-ENCODE(MyUUID).
 * ```
 *
 * @param expression A RAW or MEMPTR expression containing the value you want to convert.
 * @returns A LONGCHAR string consisting of an even number of hexadecimal digits.
 */
FUNCTION HEX-ENCODE RETURNS LONGCHAR (
    INPUT expression AS RAW
  ) FORWARD.

FUNCTION HEX-ENCODE RETURNS LONGCHAR (
    INPUT expression AS MEMPTR
  ) FORWARD.
