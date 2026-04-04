/**
 * Returns the number of rows currently in the results list of a scrolling query.
 *
 * Syntax
 *
 * `NUM-RESULTS ( query-name )`
 *
 * Notes
 * - The query must be associated with a browse widget or defined with the SCROLLING option.
 * - If the query is empty, NUM-RESULTS returns 0.
 * - When possible, the AVM performs optimizations for GET LAST and REPOSITION statements that make the results list invalid, causing NUM-RESULTS to return the Unknown value (?). These optimizations do not occur if the query is opened with the PRESELECT option or has an associated browse widget.
 * - Searching for a query using a handle is more efficient than a character expression.
 *
 * Example
 *
 * ```abl
 * DEFINE QUERY qry FOR Customer.
 * OPEN QUERY qry PRESELECT EACH Customer.
 * MESSAGE "Total rows:" NUM-RESULTS("qry").
 * ```
 *
 * @param query-name A character expression that evaluates to the name of a currently open, scrolling query.
 * @returns Returns the number of rows currently in the results list of a scrolling query, or the Unknown value (?) if the query is not found, not open, or not scrolling.
 */
FUNCTION NUM-RESULTS RETURNS INTEGER (
    INPUT query-name AS CHARACTER
  ) FORWARD.
