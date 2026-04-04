/**
 * Returns the current REJECTED attribute setting for a ProDataSet temp-table buffer.
 *
 * Syntax
 *
 * `REJECTED( buffer-name )`
 *
 * Notes
 * - This function is typically used with the SAVE-ROW-CHANGES( ) method.
 * - You can invoke the REJECTED function from within a WHERE clause (unlike the corresponding attribute).
 *
 * @param buffer-name The name of a ProDataSet temp-table buffer.
 * @returns Returns the current REJECTED attribute setting for a ProDataSet temp-table buffer.
 */
FUNCTION REJECTED RETURNS LOGICAL (
    INPUT buffer-name AS HANDLE
  ) FORWARD.
