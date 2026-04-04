/**
 * Returns, as a character string, the name of a connected database's code page.
 *
 * Syntax
 *
 * `DBCODEPAGE ( {integer-expression|logical-name|alias} )`
 *
 * Notes
 * - A database must be connected in order for the DBCODEPAGE function to work as described.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * REPEAT ix = 1 TO NUM-DBS:
 * DISPLAY LDBNAME(ix) DBCODEPAGE(ix) FORMAT "x(19)".
 * END.
 * ```
 *
 * @param database Integer expression representing the sequence number of a connected database, or character expression specifying the logical name or alias of a database.
 * @returns The name of the connected database's code page, or the Unknown value (?) if the parameter is invalid or the database is not connected.
 */
FUNCTION DBCODEPAGE RETURNS CHARACTER (
 database AS INT
 ) FORWARD.
FUNCTION DBCODEPAGE RETURNS CHARACTER (
 database AS CHAR
 ) FORWARD.
