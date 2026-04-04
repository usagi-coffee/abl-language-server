/**
 * Returns the unique internal identifier of the database record currently associated with the record buffer you name.
 *
 * Syntax
 *
 * `RECID ( record )`
 *
 * Notes
 * - This function is supported for backward compatibility. For most applications, use the ROWID function, instead.
 * - Use the RECID function to rapidly retrieve a previously identified record, even if that record has no unique index.
 * - If you want a called procedure to use the same record as a calling procedure, use the RECID function to ensure that you are retrieving the same record.
 * - Avoid storing RECID values in database fields because those RECIDs will change if you dump and reload the database.
 * - You cannot use a RECID to identify the partition of a record in a partitioned table.
 * - The RECID function returns the Unknown value (?) if a record cannot be accessed.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE crecid AS RECID NO-UNDO.
 * FIND Customer NO-LOCK USING Customer.CustNum.
 * crecid = RECID(Customer).
 * FIND Customer WHERE RECID(Customer) = crecid EXCLUSIVE-LOCK.
 * ```
 *
 * @param record The name of the record whose RECID you want.
 * @returns The unique internal identifier of the database record.
 */
FUNCTION RECID RETURNS RECID (
    INPUT record AS CHARACTER
  ) FORWARD.
