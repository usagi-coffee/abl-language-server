/**
 * Returns the length of a record in a buffer as an INTEGER value.
 *
 * Syntax
 *
 * `RECORD-LENGTH ( buffer )`
 *
 * Notes
 * - The RECORD-LENGTH function is especially useful when implementing ABL-based database replication, which involves storing entire database records in log record fields.
 * - ABL limits records to 32K. Before you transfer a record to a raw field in another record, you can use RECORD-LENGTH to ensure that you are not expanding the record beyond the 32K limit.
 *
 * @param buffer A database buffer containing a record.
 * @returns Returns the length of a record in a buffer as an INTEGER value.
 */
FUNCTION RECORD-LENGTH RETURNS INTEGER (
    INPUT buffer AS HANDLE
  ) FORWARD.
