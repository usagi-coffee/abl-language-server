/**
 * Returns an INTEGER value that represents the current change state of a static ProDataSet temp-table buffer.
 *
 * Syntax
 *
 * `ROW-STATE ( buffer-name )`
 *
 * buffer-name
 * The name of a ProDataSet temp-table buffer (preferably a before-image temp-table buffer).
 *
 * Notes
 *
 * The ROW-STATE function corresponds to the ROW-STATE attribute.
 * When the TRACKING-CHANGES attribute is set to TRUE for a ProDataSet temp-table, the
 * AVM tracks changes to the data in that temp-table using a before-image temp-table that contains the original
 * version of each modified row. You can think of the temp-table itself as the after-image because it contains
 * the latest version of each row.
 * Every row in the after-image table that has been modified or created corresponds to a row in the before-image
 * table. Deleted rows do not appear in the after-image table, because it reflects the current state of the data.
 * Every row in the before-image table has a non-zero ROW-STATE, because every row is the before-image
 * of a deleted, created, or modified row in the after-image table. Unchanged rows do not appear in the
 * before-image table.
 * You can use the ROW-STATE function on each row in either the after-image table or the before-image table
 * to determine whether a row has changed and how it has changed.
 *
 * The possible return values can be expressed as compiler constants:
 * - ROW-UNMODIFIED (0) - The row was not modified
 * - ROW-DELETED (1) - The row was deleted
 * - ROW-MODIFIED (2) - The row was modified
 * - ROW-CREATED (3) - The row was created
 *
 * The ROW-STATE function returns the Unknown value (?) when the specified temp-table buffer:
 * - Does not contain a record
 * - Is an after-image table with no associated before-image table
 *
 * You can invoke the ROW-STATE function from within a WHERE clause (unlike the corresponding attribute).
 * For example: WHERE ROW-STATE(ttOrder) = ROW-MODIFIED.
 *
 * @param buffer-name The name of a ProDataSet temp-table buffer (preferably a before-image temp-table buffer).
 * @returns An INTEGER value representing the current change state of the buffer.
 */
FUNCTION ROW-STATE RETURNS INTEGER (
 buffer-name AS HANDLE
 ) FORWARD.
