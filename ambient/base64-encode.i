/**
 * Converts binary data into a Base64 character string, and returns a LONGCHAR containing the character data.
 * The resulting LONGCHAR is in the code page specified by -cpinternal.
 *
 * Syntax
 *
 * `BASE64-ENCODE ( expression )`
 *
 * Notes
 * - The BASE64-ENCODE and BASE64-DECODE functions use the Base64 Alphabet table as found in RFC 4648, do not enforce line lengths with separators, require use of the '=' padding character, and do not implement the Filename Safe Alphabet needed for Base64URL encoding.
 * - The BASE64-ENCODE, BASE64-DECODE, HEX-ENCODE, and HEX-DECODE functions do not do any byte ordering. However, the binary output of these functions may be a binary data type such as a MEMPTR or RAW. If the target of these operation's output is another system where the order of bytes (endianness) differs, you must first normalize the data to what the receiving side is expecting.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE encdlngc AS LONGCHAR NO-UNDO.
 * DEFINE VARIABLE encdmptr AS MEMPTR NO-UNDO.
 * COPY-LOB FROM FILE "C:\myicons\test.ico" TO encdmptr.
 * encdlngc = BASE64-ENCODE(encdmptr).
 * ```
 *
 * @param expression A MEMPTR or RAW expression containing the binary data you want to convert.
 * @returns LONGCHAR containing the Base64 character data.
 */
FUNCTION BASE64-ENCODE RETURNS LONGCHAR (
 expression AS MEMPTR
 ) FORWARD.

FUNCTION BASE64-ENCODE RETURNS LONGCHAR (
 expression AS RAW
 ) FORWARD.
