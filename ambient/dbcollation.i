/**
 * Returns, as a character string, the name of the collating sequence for character set information contained in
 * the database. This name corresponds to the definition of the collating sequence contained in the convmap.dat
 * file, which usually resides in the $DLC directory. If any parameter is invalid, DBCOLLATION returns the Unknown
 * value (?).
 *
 * Syntax
 *
 * `DBCOLLATION ( {integer-expression|logical-name|alias} )`
 *
 * Notes
 * - OpenEdge and non-OpenEdge DataServers can evaluate the syntactical expression stated in a
 * DBCOLLATION function. However, the methods used to process multiple byte code pages can differ based
 * on the actual server used. Keep this point in mind if the actual results you receive differ from the results you
 * expected.
 * - A database must be connected in order for the DBCOLLATION function to work as described.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * REPEAT ix = 1 TO NUM-DBS:
 * DISPLAY LDBNAME(ix) DBCOLLATION(ix) FORMAT "x(19)".
 * END.
 * ```
 *
 * @param integer-expression The sequence number of a database the ABL session is connected to
 * @param logical-name A character expression that specifies the database by its logical name
 * @param alias A character expression that specifies the database by its alias
 * @returns The name of the collating sequence as a character string, or Unknown value (?) if invalid
 */
FUNCTION DBCOLLATION RETURNS CHARACTER (
 INPUT pExpression AS INTEGER
 ) FORWARD.

FUNCTION DBCOLLATION RETURNS CHARACTER (
 INPUT pLogicalName AS CHARACTER
 ) FORWARD.
