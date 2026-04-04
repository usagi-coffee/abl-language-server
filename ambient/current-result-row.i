/**
 * Returns the number of the current row of a specified query as an INTEGER value.
 *
 * Syntax
 *
 * `CURRENT-RESULT-ROW ( query-name )`
 *
 * Notes
 * - To use the CURRENT-RESULT-ROW function with a query, the query must be associated with a browse
 * widget or you must define the query with the SCROLLING option.
 * - If the query is empty, CURRENT-RESULT-ROW returns the Unknown value (?).
 * - If the query is positioned before the first record, CURRENT-RESULT-ROW returns the value 1. If the query
 * is positioned beyond the last record, CURRENT-RESULT-ROW returns a value 1 greater than the number
 * of rows in the query result list.
 * - When possible, the AVM performs optimizations for GET LAST and REPOSITION statements. These
 * optimizations make the results list invalid. At that point, CURRENT-RESULT-ROW returns the Unknown
 * value (?). These optimizations do not occur if the query is opened with the PRESELECT option or has an
 * associated browse widget.
 *
 * Example
 *
 * ```abl
 * DEFINE QUERY cust-query FOR Customer SCROLLING.
 * OPEN QUERY cust-query FOR EACH Customer WHERE Customer.Country = "USA".
 * REPEAT:
 * GET NEXT cust-query.
 * IF QUERY-OFF-END("cust-query") THEN LEAVE.
 * DISPLAY CURRENT-RESULT-ROW("cust-query") LABEL "Result Row"
 * Customer.CustNum Customer.Name.
 * END.
 * ```
 *
 * @param query-name A character expression that evaluates to the name of a currently open, scrolling query. If query-name does not resolve to the name of a query, or if the query is not open or not scrolling, then the function returns the Unknown value (?).
 * @returns The number of the current row as an INTEGER value.
 */
FUNCTION CURRENT-RESULT-ROW RETURNS INTEGER (
 query-name AS CHARACTER
 ) FORWARD.
