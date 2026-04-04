/**
 * Returns the logical name of a database that is currently connected.
 *
 * Syntax
 *
 * `LDBNAME ( { integer-expression | logical-name | alias | BUFFER bufname } )`
 *
 * Parameters
 *
 * `integer-expression`
 * The sequence number of a database the ABL session is connected to. For example,
 * LDBNAME(1) returns information on the first database the ABL session is connected to,
 * LDBNAME(2) returns information on the second database the ABL session is connected to, etc.
 * If you specify a sequence number that does not correspond to a database the ABL session
 * is connected to, the LDBNAME function returns the Unknown value (?).
 *
 * `logical-name` or `alias`
 * These forms of the LDBNAME function require a quoted character string or a character
 * expression as a parameter. If the parameter is the logical name of a connected database
 * or an alias of a connected database then the logical name is returned. Otherwise, the
 * AVM returns the Unknown value (?).
 *
 * `BUFFER bufname`
 * The name of a database table or buffer. The BUFFER option lets you determine the database
 * a certain table belongs to without hard-coding the logical database name or alias.
 *
 * Example
 *
 * ```abl
 * DO WHILE LDBNAME(1) <> ? : /* the parameter. is the number 1 */
 * DISCONNECT VALUE(LDBNAME(1)).
 * END.
 * ```
 *
 * ```abl
 * DEFINE VARIABLE testnm AS CHARACTER NO-UNDO.
 * SET testnm.
 * IF LDBNAME(testnm) = testnm THEN
 * MESSAGE testnm "is a true logical database name.".
 * ELSE IF LDBNAME(testnm) = ? THEN
 * MESSAGE testnm "is not the name or alias of any connected database.".
 * ELSE
 * MESSAGE testnm "is an ALIAS for database " LDBNAME(testnm).
 * ```
 *
 * @param piDatabaseNum The sequence number of a database the ABL session is connected to.
 * @param pcLogicalName A quoted character string or character expression representing a logical name.
 * @param pcAlias A quoted character string or character expression representing an alias.
 * @param phBuffer The handle of a buffer object.
 * @returns CHARACTER - The logical name of the database, or Unknown value (?) if not found.
 */
FUNCTION LDBNAME RETURNS CHARACTER (
 INPUT piDatabaseNum AS INTEGER
 ) FORWARD.

FUNCTION LDBNAME RETURNS CHARACTER (
 INPUT pcLogicalName AS CHARACTER
 ) FORWARD.

FUNCTION LDBNAME RETURNS CHARACTER (
 INPUT pcAlias AS CHARACTER
 ) FORWARD.

FUNCTION LDBNAME RETURNS CHARACTER (
 INPUT phBuffer AS HANDLE
 ) FORWARD.
