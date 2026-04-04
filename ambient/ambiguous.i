/**
 * Returns a TRUE value if the last FIND statement for a particular record found more than one record that met
 * the specified index criteria.
 *
 * Syntax
 *
 * `AMBIGUOUS record`
 *
 * Notes
 * - AMBIGUOUS is useful only when there is an index. If you use the AMBIGUOUS function to test a work file
 * record, the function returns a value of FALSE because work files do not have indexes.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE cName NO-UNDO LIKE Customer.Name LABEL "Cust Name".
 * REPEAT:
 * SET cName.
 * FIND Customer NO-LOCK WHERE Customer.Name = cName NO-ERROR.
 * IF AVAILABLE Customer THEN
 * DISPLAY Customer.CustNum Customer.Address Customer.City Customer.State
 * Customer.PostalCode.
 * ELSE IF AMBIGUOUS Customer THEN
 * MESSAGE "There is more than one customer with that name".
 * ELSE
 * MESSAGE "Cannot find customer with that name".
 * END.
 * ```
 *
 * @param record The name of a record or record buffer used in a previous FIND statement.
 * @returns TRUE if more than one record matched the selection criteria; FALSE otherwise.
 */
FUNCTION AMBIGUOUS RETURNS LOGICAL (
  record AS HANDLE
  ) FORWARD.
