/**
 * Returns a TRUE value if a record is found that meets the specified FIND criteria;
 * otherwise it returns FALSE. CAN-FIND does not make the record available to the procedure.
 * You typically use the CAN-FIND function within a VALIDATE option in a data handling
 * statement, such as the UPDATE statement. You can use CAN-FIND to see if a record exists
 * with less system overhead than that of a FIND statement. The query capabilities are similar.
 * CAN-FIND is also useful for implementing inner joins among database tables.
 *
 * Syntax
 *
 * `CAN-FIND ( [ FIRST | LAST ] record [ constant ] [ OF table ] [ WHERE expression ] [ USE-INDEX index ] [ USING [ FRAME frame ] field [ AND [ FRAME frame ] field ] ... ] [ SHARE-LOCK | NO-LOCK ] [ NO-WAIT ] [ NO-PREFETCH ] )`
 *
 * Notes
 * - Fields do not have to be indexed to use them in a CAN-FIND function.
 * - You can name more than one field as part of the selection criteria.
 * - CAN-FIND supports selection criteria that uses inequality matches.
 * - EXCLUSIVE-LOCK is not allowed in a CAN-FIND because CAN-FIND does not return a record.
 * - If you use the CAN-FIND function to find a record in a work table, the AVM disregards the NO-WAIT, SHARE-LOCK, and NO-LOCK options.
 * - You can nest CAN-FIND functions.
 * - The CAN-FIND function does not cause FIND triggers to execute.
 * - You cannot use the CAN-FIND function in a query's WHERE clause.
 * - Within a CAN-FIND function, if you compare tables or fields from multiple databases, you must explicitly specify the database name.
 * - CAN-FIND does not raise a run-time error if the buffers in its predicate are unavailable.
 * - CAN-FIND may fail to find a newly created record.
 *
 * Example
 *
 * ```abl
 * REPEAT:
 * CREATE Customer.
 * UPDATE Customer.CustNum Customer.Name Customer.SalesRep
 * VALIDATE(CAN-FIND(SalesRep.SalesRep
 * WHERE SalesRep.SalesRep = Customer.SalesRep),
 * "Invalid sales rep -- please re-enter").
 * END.
 * ```
 *
 * @param find-specification The record finding specification including record, WHERE clause, and options
 * @returns LOGICAL TRUE if a record is found meeting the criteria, otherwise FALSE
 */
FUNCTION CAN-FIND RETURNS LOGICAL (
 find-specification AS CHARACTER
 ) FORWARD.
