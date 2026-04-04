/**
 * Returns the page size (lines per page) of an output destination as an INTEGER value.
 *
 * Syntax
 *
 * `PAGE-SIZE [ ( stream | STREAM-HANDLE handle ) ]`
 *
 * Notes
 * - If the output stream is not paged, PAGE-SIZE returns a value of 0.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE lines-per-page AS INTEGER NO-UNDO.
 * lines-per-page = PAGE-SIZE.
 * ```
 *
 * @param stream The name of a stream.
 * @param handle A handle to a stream.
 * @returns Returns the page size (lines per page) of an output destination.
 */
FUNCTION PAGE-SIZE RETURNS INTEGER (
    INPUT stream AS CHARACTER
  ) FORWARD.

FUNCTION PAGE-SIZE RETURNS INTEGER (
    INPUT handle AS HANDLE
  ) FORWARD.

FUNCTION PAGE-SIZE RETURNS INTEGER (
  ) FORWARD.
