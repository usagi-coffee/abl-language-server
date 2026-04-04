/**
 * Returns the logical name of the schema holder database for a connected non-OpenEdge database.
 *
 * Syntax
 *
 * `SDBNAME ( { integer-expression | logical-name | alias } )`
 *
 * Parameters
 *
 * `integer-expression`
 * If the parameter supplied to SDBNAME is an integer expression, and there are, for example,
 * three connected databases, then SDBNAME(1), SDBNAME(2), and SDBNAME(3) return the logical names
 * of their respective schema holder databases. Also, if there are three connected databases,
 * SDBNAME(4), SDBNAME(5), etc., return the Unknown value (?).
 *
 * `logical-name` or `alias`
 * These forms of the SDBNAME function require a quoted character string or a character expression
 * as a parameter. If the parameter is the logical name of a connected database or an alias of a
 * connected database, then the logical name of the schema holder database is returned according
 * to the rule. Otherwise, SDBNAME returns the Unknown value (?).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * REPEAT ix = 1 TO NUM-DBS:
 * DISPLAY SDBNAME(ix) SDBNAME(ix) = LDBNAME(ix)
 * FORMAT "SCHEMA-HOLDER/SUB-SCHEMA"
 * COLUMN-LABEL " DataServer!Classification".
 * END.
 * ```
 *
 * @param piDatabaseNum The sequence number of a database the ABL session is connected to.
 * @param pcLogicalName A quoted character string or character expression representing a logical name.
 * @param pcAlias A quoted character string or character expression representing an alias.
 * @returns CHARACTER - The logical name of the schema holder database, or Unknown value (?) if not found.
 */
FUNCTION SDBNAME RETURNS CHARACTER (
 INPUT piDatabaseNum AS INTEGER
 ) FORWARD.

FUNCTION SDBNAME RETURNS CHARACTER (
 INPUT pcLogicalName AS CHARACTER
 ) FORWARD.

FUNCTION SDBNAME RETURNS CHARACTER (
 INPUT pcAlias AS CHARACTER
 ) FORWARD.
