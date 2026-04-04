/**
 * Returns the offset of the file pointer in a text file as an INT64 value.
 *
 * Syntax
 *
 * `SEEK ( INPUT )`
 *
 * `SEEK ( OUTPUT )`
 *
 * `SEEK ( name )`
 *
 * `SEEK ( STREAM-HANDLE handle )`
 *
 * Notes
 * - The first byte in a file is byte 0.
 * - You cannot use the SEEK function with the INPUT THROUGH statement, the INPUT-OUTPUT THROUGH statement, or the OUTPUT THROUGH statement. When used with one of these statements, the SEEK function returns the Unknown value (?).
 * - After you assign the value of the SEEK function to a procedure variable, you can use that value to reposition the file in the event of an error.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE m-pos AS INT64 NO-UNDO.
 * OUTPUT TO test.fil.
 * m-pos = SEEK(OUTPUT).
 * EXPORT "some data".
 * OUTPUT CLOSE.
 * ```
 *
 * @returns The current position of the file pointer as an INT64 value.
 */
FUNCTION SEEK RETURNS INT64 (
    INPUT seek-input AS CHARACTER
  ) FORWARD.

FUNCTION SEEK RETURNS INT64 (
    INPUT seek-output AS CHARACTER
  ) FORWARD.

FUNCTION SEEK RETURNS INT64 (
    INPUT stream-name AS CHARACTER
  ) FORWARD.

FUNCTION SEEK RETURNS INT64 (
    INPUT stream-handle AS HANDLE
  ) FORWARD.
