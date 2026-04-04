/**
 * Tells whether a database is connected. If logical name is the logical name or alias is the alias of a
 * connected database, the CONNECTED function returns TRUE; otherwise, it returns FALSE.
 *
 * Syntax
 *
 * `CONNECTED ( logical-name | alias )`
 *
 * Notes
 * - By default the AVM disconnects all databases at the end of a session. The DISCONNECT statement, which
 * explicitly disconnects a database, does not execute until all active procedures that reference the database
 * end or stop. Therefore the disconnection may happen later than the end of a transaction.
 *
 * Example
 *
 * ```abl
 * IF CONNECTED("Sports2020") THEN RUN r-dispcu.p.
 * ```
 *
 * @param logical-name Refers to a logical name. It can be a quoted string or a character expression. An unquoted character string is not allowed.
 * @param alias Refers to an alias. It can be a quoted string or a character expression. An unquoted character string is not allowed.
 * @returns TRUE if the database is connected; otherwise FALSE.
 */
FUNCTION CONNECTED RETURNS LOGICAL (
 INPUT pcLogicalNameOrAlias AS CHARACTER
 ) FORWARD.
