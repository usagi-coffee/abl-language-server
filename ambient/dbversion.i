/**
 * Returns, as a character string, the version number of an OpenEdge database.
 *
 * Syntax
 *
 * `DBVERSION ( integer-expression )`
 * `DBVERSION ( logical-name|alias )`
 *
 * Notes
 * - DBVERSION does not apply to non-OpenEdge data sources.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * REPEAT ix = 1 TO NUM-DBS:
 * DISPLAY LDBNAME(ix) DBVERSION(ix) WITH 1 DOWN.
 * END.
 * ```
 *
 * @param integer-expression The sequence number of a database the ABL session is connected to.
 * @param logical-name|alias A quoted character string or character expression representing an alias or logical name of a connected database.
 * @returns The version number of the specified database as a character string, or Unknown value (?) if the database is not found.
 */
FUNCTION DBVERSION RETURNS CHARACTER (
 INPUT piSequence AS INTEGER
 ) FORWARD.

FUNCTION DBVERSION RETURNS CHARACTER (
 INPUT pcLogicalNameOrAlias AS CHARACTER
 ) FORWARD.
