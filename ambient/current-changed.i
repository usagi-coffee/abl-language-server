/**
 * Returns TRUE if the copy of the record in the buffer after executing a FIND CURRENT or GET CURRENT
 * differs from the copy of the record in the buffer before executing the FIND CURRENT or GET CURRENT. That
 * is, if the current application changes the record, but no other user changes the record during its scope in the
 * current application, CURRENT-CHANGED returns FALSE.
 *
 * Syntax
 *
 * `CURRENT-CHANGED record`
 *
 * Notes
 * - The CURRENT-CHANGED function is valid only when called after a FIND CURRENT or GET CURRENT statement.
 * - If a client application modifies the buffer, the AVM compares the newly read record with the buffer contents from that application, rather than with the record read from the server. The CURRENT-CHANGED function continues to return a value based on the contents of the buffer until the next FIND CURRENT or GET CURRENT operates on that buffer or until the buffer goes out of scope or is released.
 * - The CURRENT-CHANGED function can compare the current values with the initial values of BLOB or CLOB fields provided the table is in an OpenEdge database.
 *
 * Example
 *
 * ```abl
 * DO TRANSACTION:
 * FIND CURRENT Customer EXCLUSIVE-LOCK.
 * IF CURRENT-CHANGED Customer THEN DO:
 * MESSAGE "This record has been changed by another user" SKIP
 * "Please re-enter your changes." VIEW-AS ALERT-BOX.
 * DISPLAY Customer.Name Customer.Balance WITH FRAME upd.
 * RETURN NO-APPLY.
 * END.
 * ASSIGN Customer.Name Customer.Balance.
 * END.
 * ```
 *
 * @param record The name of a table or buffer.
 * @returns TRUE if the copy of the record in the buffer has been changed by another user since it was read; otherwise FALSE.
 */
FUNCTION CURRENT-CHANGED RETURNS LOGICAL (
 INPUT record AS HANDLE
 ) FORWARD.
