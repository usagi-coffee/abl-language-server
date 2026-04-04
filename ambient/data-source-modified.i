/**
 * Returns TRUE if data in the data source associated with the specified ProDataSet temp-table buffer has been
 * modified.
 *
 * Syntax
 *
 * `DATA-SOURCE-MODIFIED( buffer-name )`
 *
 * Notes
 * - The AVM sets the value of this function from the SAVE-ROW-CHANGES( ) method.
 * - The DATA-SOURCE-MODIFIED function corresponds to the DATA-SOURCE-MODIFIED attribute.
 * - You can invoke the DATA-SOURCE-MODIFIED function from within a WHERE clause (unlike the corresponding attribute).
 *
 * @param buffer-name The name of a ProDataSet temp-table buffer.
 * @returns TRUE if data in the data source has been modified.
 */
FUNCTION DATA-SOURCE-MODIFIED RETURNS LOGICAL (
 buffer-name AS HANDLE
 ) FORWARD.
