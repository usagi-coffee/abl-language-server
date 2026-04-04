/**
 * Returns the current line position in a report output stream.
 *
 * Syntax
 *
 * `LINE-COUNTER [ ( stream-expression ) ]`
 *
 * Notes
 * - If you do not specify a stream, the AVM uses the unnamed stream.
 * - The LINE-COUNTER function returns the current line position in a report output stream.
 * - You can use the LINE-COUNTER function to determine when you are at the end of a page.
 * - The value returned by the LINE-COUNTER function is affected by the PAGE-SIZE option of the OUTPUT TO statement.
 *
 * Example
 *
 * ```abl
 * OUTPUT TO PRINTER.
 * FOR EACH Customer NO-LOCK:
 * IF LINE-COUNTER > 50 THEN PAGE.
 * DISPLAY Customer.CustNum Customer.Name.
 * END.
 * OUTPUT CLOSE.
 * ```
 *
 * @param stream-expression The name of a stream to determine the current line for. If you do not specify a stream, the AVM uses the unnamed stream.
 * @returns INTEGER value representing the current line position in a report output stream.
 */
FUNCTION LINE-COUNTER RETURNS INTEGER (
 INPUT p-stream AS CHARACTER
 ) FORWARD.
