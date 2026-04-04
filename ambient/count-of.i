/**
 * Returns an INTEGER value that is the total number of selected records in the table or tables you are using
 * across break groups.
 *
 * Syntax
 *
 * `COUNT-OF ( break-group )`
 *
 * Notes
 * - The COUNT-OF function requires a sorted, preselected list of records. This sorted preselected list is not
 * constructed if the BREAK is by an active index because the compiler optimization knows the list is already
 * sorted. The Aggregate phrase COUNT gives the count of a break or sort group and may
 * be more appropriate. It does not require a preselection.
 *
 * Example
 *
 * ```abl
 * FOR EACH Customer NO-LOCK BREAK BY Customer.State:
 * DISPLAY Customer.CustNum Customer.Name Customer.SalesRep Customer.State.
 * ACCUMULATE Customer.State (SUB-COUNT BY Customer.State).
 * IF LAST-OF( Customer.State) THEN
 * DISPLAY 100 * (ACCUM SUB-COUNT BY Customer.State Customer.State) /
 * COUNT-OF(Customer.State) FORMAT "99.9999%"
 * COLUMN-LABEL "% of Total!Customers".
 * END.
 * ```
 *
 * @param break-group The name of a field or expression you named in the block header with the BREAK BY option.
 * @returns INTEGER value that is the total number of selected records across break groups.
 */
FUNCTION COUNT-OF RETURNS INTEGER (
 break-group AS CHARACTER
 ) FORWARD.
