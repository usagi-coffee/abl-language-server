/**
 * Returns a LOGICAL value indicating whether the specified query is positioned at the end of its result list (either before the first record or after the last record).
 *
 * Syntax
 *
 * `QUERY-OFF-END ( query-name )`
 *
 * Notes
 * - Searching for a query using a handle is more efficient than a character expression.
 * - To test whether a GET statement read beyond the last (or first) record, you can use the AVAILABLE function with the buffer name.
 *
 * Example
 *
 * ```abl
 * OPEN QUERY cust-query FOR EACH Customer.
 * REPEAT:
 *   GET NEXT cust-query.
 *   IF QUERY-OFF-END("cust-query") THEN LEAVE.
 *   DISPLAY Customer.CustNum Customer.Name.
 * END.
 * ```
 *
 * @param query-name A character expression that evaluates to the name of a currently open query. If query-name does not resolve to the name of a query, or if the query is not open, then the function returns the Unknown value (?).
 * @returns Returns TRUE if the query is positioned at the end of its result list.
 */
FUNCTION QUERY-OFF-END RETURNS LOGICAL (
    INPUT query-name AS CHARACTER
  ) FORWARD.
