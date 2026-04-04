/**
 * Returns the partition ID (as an integer) of the partition of the current record in a specified buffer.
 *
 * Syntax
 *
 * `BUFFER-PARTITION-ID ( buffer-name )`
 *
 * buffer-name
 * An identifier that specifies the name of a record buffer.
 *
 * Notes
 * - If the buffer is not populated with a record, this function returns the Unknown value (?). If there is a record in the buffer but it is not for a partitioned table, this function returns zero (0).
 * - If you use this function in a WHERE clause, it will be evaluated for each iteration on the client side after the record has been retrieved. It will not participate in any index bracketing, so unless there are other partition-column criteria in the same WHERE clause, it will be very inefficient.
 * - The partition ID is only unique within a given table.
 *
 * @param buffer-name An identifier that specifies the name of a record buffer.
 * @returns The partition ID (as an integer) of the partition of the current record in a specified buffer.
 */
FUNCTION BUFFER-PARTITION-ID RETURNS INTEGER (
 buffer-name AS HANDLE
 ) FORWARD.
