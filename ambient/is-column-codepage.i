/**
 * Returns a logical value indicating whether a CLOB or BLOB column is associated with a codepage.
 *
 * Syntax
 *
 * `IS-COLUMN-CODEPAGE ( column-name [, codepage ] )`
 *
 * Notes
 * - This function applies only to CLOB and BLOB columns that have codepage attributes.
 * - If codepage is specified, the function returns TRUE if the column has that specific codepage.
 * - If codepage is not specified, the function returns TRUE if the column has any codepage defined.
 * - The function returns FALSE if the column does not have a codepage attribute or if the specified codepage does not match.
 *
 * @param column-name The name of a CLOB or BLOB column in a temp-table or database table.
 * @param codepage Optional character expression specifying a codepage name to compare against.
 * @returns LOGICAL - TRUE if the column has the specified codepage (or any codepage if not specified), FALSE otherwise.
 */
FUNCTION IS-COLUMN-CODEPAGE RETURNS LOGICAL (
 INPUT column-name AS HANDLE
 ) FORWARD.
FUNCTION IS-COLUMN-CODEPAGE RETURNS LOGICAL (
 INPUT column-name AS HANDLE,
 INPUT codepage AS CHARACTER
 ) FORWARD.
