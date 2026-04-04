/**
 * Converts a Base64 character string into a binary value. The result is a MEMPTR containing the binary data.
 *
 * Syntax
 *
 * `BASE64-DECODE ( expression )`
 *
 * Notes
 * - The BASE64-ENCODE and BASE64-DECODE functions use the Base64 Alphabet table as found in RFC 4648.
 * - Do not enforce line lengths with separators.
 * - Require use of the '=' padding character.
 * - Do not implement the Filename Safe Alphabet needed for Base64URL encoding.
 * - The BASE64-ENCODE, BASE64-DECODE, HEX-ENCODE, and HEX-DECODE functions do not do any byte ordering. However, the binary output of these functions may be a binary data type such as a MEMPTR or RAW. If the target of these operation's output is another system where the order of bytes (endianness) differs, you must first normalize the data to what the receiving side is expecting. Otherwise, the results may not be as expected.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE decdmptr AS MEMPTR NO-UNDO.
 * DEFINE VARIABLE decdlngc AS LONGCHAR NO-UNDO.
 * COPY-LOB FROM FILE "C:\myicons\testencode" TO decdlngc.
 * decdmptr = BASE64-DECODE(decdlngc).
 * COPY-LOB FROM decdmptr TO FILE "C:\myicons\test.ico".
 * ```
 *
 * @param expression A CHARACTER or LONGCHAR expression containing the string you want to convert.
 * @returns A MEMPTR containing the binary data.
 */
FUNCTION BASE64-DECODE RETURNS MEMPTR (
 expression AS CHARACTER
 ) FORWARD.

FUNCTION BASE64-DECODE RETURNS MEMPTR (
 expression AS LONGCHAR
 ) FORWARD.
