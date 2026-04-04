/**
 * Returns a TRUE value in the following cases:
 * - a record is not available to a prior FIND . . . NO-WAIT statement because another user has locked the
 * record
 * - a record is not available to a GET . . . NO-WAIT statement because another user has locked the record
 *
 * Syntax
 *
 * `LOCKED record`
 *
 * Notes
 * - To use the LOCKED function with a record in a table defined for multiple databases, you must qualify
 * the record's table name with the database name. See the record definition in the Record phrase
 * reference entry for more information.
 * - The result of the LOCKED function depends on the lock mode specified. For example, if the FIND
 * statement uses SHARE-LOCK and no user has an EXCLUSIVE-LOCK on the record, the LOCKED function
 * returns FALSE. If the FIND statement uses SHARE-LOCK and another user has an EXCLUSIVE-LOCK on
 * the record, the LOCKED function returns TRUE.
 *
 * Example
 *
 * ```abl
 * REPEAT:
 * PROMPT-FOR Customer.CustNum.
 * FIND Customer USING Customer.CustNum NO-ERROR NO-WAIT.
 * IF NOT AVAILABLE Customer THEN DO:
 * IF LOCKED Customer THEN
 * MESSAGE "Customer record is locked".
 * ELSE
 * MESSAGE "Customer record was not found".
 * NEXT.
 * END.
 * DISPLAY Customer.CustNum Customer.Name Customer.City Customer.State.
 * END.
 * ```
 *
 * @param record The name of a record or buffer.
 * @returns Returns TRUE if the record is locked by another user; FALSE otherwise.
 */
FUNCTION LOCKED RETURNS LOGICAL (
 record AS HANDLE
 ) FORWARD.
