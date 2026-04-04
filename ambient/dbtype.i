/**
 * Returns, as a character string, the database type of a currently connected database.
 * This function returns one of the following strings: "MSS", "ORACLE", or "PROGRESS".
 *
 * Syntax
 *
 * `DBTYPE ( integer-expression )`
 *
 * `DBTYPE ( logical-name | alias )`
 *
 * Notes
 * - You can reference the DBTYPE function within a preprocessor expression.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * REPEAT ix = 1 TO NUM-DBS:
 * DISPLAY LDBNAME(ix) DBTYPE(ix) FORMAT "x(40)".
 * END.
 * ```
 *
 * @param integer-expression The sequence number of a database the ABL session is connected to.
 * @param logical-name A quoted character string or character expression representing the logical name of a connected database.
 * @param alias A quoted character string or character expression representing an alias of a connected database.
 * @returns CHARACTER The database type ("MSS", "ORACLE", or "PROGRESS"), or the Unknown value (?) if not found.
 */
FUNCTION DBTYPE RETURNS CHARACTER (
 INPUT p_integer_expression AS INTEGER
 ) FORWARD.

FUNCTION DBTYPE RETURNS CHARACTER (
 INPUT p_logical_name_or_alias AS CHARACTER
 ) FORWARD.
