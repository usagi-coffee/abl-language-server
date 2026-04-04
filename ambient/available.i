/**
 * Returns a TRUE value if the record buffer you name contains a record and returns a FALSE value if the record
 * buffer is empty.
 * When you use the FIND statement or the FOR EACH statement to find a record, the AVM reads that record
 * from the database into a record buffer. This record buffer has the same name as the file used by the FIND or
 * FOR EACH statement, unless you specify otherwise. The CREATE statement creates a new record in a record
 * buffer.
 *
 * Syntax
 *
 * `AVAILABLE record`
 *
 * Notes
 * - To access a record in a table defined for multiple databases, you might have to qualify the
 * record's table name with the database name. See the record definition in the Record phrase on
 * reference entry for more information.
 *
 * Example
 *
 * ```abl
 * REPEAT:
 * PROMPT-FOR Item.ItemNum.
 * FIND Item USING ItemNum NO-ERROR.
 * IF AVAILABLE Item THEN
 * DISPLAY Item.ItemName Item.Price.
 * ELSE
 * MESSAGE "Not found".
 * END.
 * ```
 *
 * @param record The name of the record buffer you want to check.
 * @returns Returns TRUE if the record buffer contains a record, FALSE if the record buffer is empty.
 */
FUNCTION AVAILABLE RETURNS LOGICAL (
  record AS HANDLE
  ) FORWARD.
