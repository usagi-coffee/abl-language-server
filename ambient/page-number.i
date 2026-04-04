/**
 * Returns the page number of the output destination as an INTEGER value.
 *
 * Syntax
 *
 * `PAGE-NUMBER [ ( stream | STREAM-HANDLE handle ) ]`
 *
 * Example
 *
 * ```abl
 * OUTPUT TO pagenum.txt PAGED.
 * FOR EACH Customer NO-LOCK:
 *     FORM HEADER "Customer report" AT 30
 *         "Page:" AT 60 PAGE-NUMBER FORMAT ">>9" SKIP(1).
 *     DISPLAY Customer.CustNum Customer.Name Customer.Address Customer.City
 *         Customer.State Customer.Country.
 * END.
 * ```
 *
 * @param stream The name of an output stream. If you do not name a stream, PAGE-NUMBER returns the page number of the default unnamed output stream.
 * @param stream-handle The handle to an output stream. If handle is not a valid handle to a stream, the AVM generates a run-time error.
 * @returns The page number of the output destination as an INTEGER value. If the output stream is not paged, returns a value of 0.
 */
FUNCTION PAGE-NUMBER RETURNS INTEGER (
    INPUT stream AS CHARACTER,
    INPUT stream-handle AS HANDLE
  ) FORWARD.

FUNCTION PAGE-NUMBER RETURNS INTEGER (
    INPUT stream AS CHARACTER
  ) FORWARD.

FUNCTION PAGE-NUMBER RETURNS INTEGER ( ) FORWARD.
