/**
 * Indicates whether an error occurred during a FILL or SAVE-ROW-CHANGES operation on the specified
 * ProDataSet temp-table buffer.
 *
 * Syntax
 *
 * `ERROR( buffer-name )`
 *
 * Notes
 * - The ERROR function corresponds to the ERROR attribute.
 * - You can invoke the ERROR function from within a WHERE clause (unlike the corresponding attribute).
 *
 * @param buffer-name The name of a ProDataSet temp-table buffer.
 * @returns LOGICAL
 */
FUNCTION ERROR RETURNS LOGICAL (
 buffer-name AS HANDLE
 ) FORWARD.
