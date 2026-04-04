/**
 * Returns the unique internal identifier of the database record currently associated with the record buffer you name.
 *
 * Syntax
 *
 * `ROWID ( record )`
 *
 * Notes
 * - The ROWID function corresponds to the ROWID attribute.
 * - This function replaces the RECID function for most applications.
 * - You must use the RECID function for maintaining schema objects in the ABL meta-schema files.
 * - Use the ROWID function to rapidly retrieve a previously identified record, even if that record has no unique index.
 * - The ROWID data type is a variable-length byte string capable of representing a record identifier for any DataServer database.
 * - You cannot return a ROWID for a view because view records do not have unique identifiers.
 * - You can compare ROWID values using the ABL relational operators (=, >, <, <>, >=, and <=).
 * - You can use a ROWID value in a REPOSITION statement to specify the new position for a query cursor.
 * - You can store a ROWID value in a work table or temp-table, but not directly in a database table.
 * - You can use the STRING function to convert a ROWID value to a character string, and convert it back using the TO-ROWID function.
 * - The ROWID function returns the Unknown value (?) if a record cannot be accessed.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE crowid AS ROWID NO-UNDO.
 * FIND Customer USING Customer.CustNum NO-LOCK.
 * crowid = ROWID(Customer).
 * FIND Customer WHERE ROWID(Customer) = crowid EXCLUSIVE-LOCK.
 * UPDATE Customer.CreditLimit.
 * ```
 *
 * @param record The name of the record whose ROWID you want.
 * @returns Returns the unique internal identifier of the database record.
 */
FUNCTION ROWID RETURNS ROWID (
    INPUT record AS HANDLE
  ) FORWARD.
