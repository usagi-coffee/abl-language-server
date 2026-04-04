/**
 * Encodes a source character string and returns the encoded character string result.
 *
 * Syntax
 *
 * `ENCODE ( expression )`
 *
 * Notes
 * - You can use the ENCODE function to encode a string that contains double-byte characters.
 * - The ENCODE function performs a one-way encoding operation that you cannot reverse. It is useful for storing scrambled copies of passwords in a database. It is impossible to determine the original password by examining the database. However, a procedure can prompt a user for a password, encode it, and compare the result with the stored, encoded password to determine if the user supplied the correct password.
 * - In order to ensure reliable results, the original encoding and any subsequent encoded comparisons must run in the same code page. In environments with multiple code pages, Progress Software Corporation strongly recommends that programs use the CODEPAGE-CONVERT function so that occurrences of the ENCODE function related to the same strings run in the same code page.
 * - The output of the ENCODE function is 16 characters long. Make sure the target field size is at least 16 characters long.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE password AS CHARACTER NO-UNDO FORMAT "x(16)".
 * DEFINE VARIABLE n-coded-p-wrd AS CHARACTER NO-UNDO FORMAT "x(16)".
 * SET password LABEL "Enter password" BLANK.
 * n-coded-p-wrd = ENCODE(password).
 * DISPLAY n-coded-p-wrd LABEL "Encoded password".
 * ```
 *
 * @param expression An expression that results in a character string value. If you use a constant, you must enclose it in quotation marks (" ").
 * @returns The encoded character string result (16 characters long).
 */
FUNCTION ENCODE RETURNS CHARACTER (
 expression AS CHARACTER
 ) FORWARD.
