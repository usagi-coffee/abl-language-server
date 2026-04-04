/**
 * Returns a character string that describes features that are not supported for this database. You can use this function with OpenEdge DataServers.
 *
 * Syntax
 *
 * `DBRESTRICTIONS ( { integer-expression | logical-name | alias } [ , table-name ] )`
 *
 * Notes
 * - If you want to use the DBRESTRICTIONS function for a database, you must be connected to the database in the current ABL session.
 * - DBRESTRICTIONS returns a string. This string is a comma-separated list of keywords that represent features not supported by the specified database.
 * - The possible keyword values returned by DBRESTRICTIONS depends on the DataServer type.
 * - The form of the returned string makes it easy to use with the ENTRY and LOOKUP function.
 * - If you connect to a database with the Read Only (-RO) parameter, the AVM lists the character string READ-ONLY in the restrictions list for that database.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * REPEAT ix = 1 to NUM-DBS:
 * DISPLAY LDBNAME(ix) LABEL "Database"
 * DBRESTRICTIONS(ix) FORMAT "x(40)" LABEL "Restrictions".
 * END.
 * ```
 *
 * @param database A character expression equal to the logical name or alias of a connected database. An unquoted character string is not permitted.
 * @param table-name A character expression equal to the name of a table in the specified database. An unquoted character string is not permitted. If the table name is valid, DBRESTRICTIONS returns the list of unsupported features for the specified table.
 * @returns A character string that describes features that are not supported for this database.
 */
FUNCTION DBRESTRICTIONS RETURNS CHARACTER (
 INPUT database AS CHARACTER,
 INPUT table-name AS CHARACTER
 ) FORWARD.

/**
 * Returns a character string that describes features that are not supported for this database. You can use this function with OpenEdge DataServers.
 *
 * Syntax
 *
 * `DBRESTRICTIONS ( { integer-expression | logical-name | alias } [ , table-name ] )`
 *
 * @param database A character expression equal to the logical name or alias of a connected database. An unquoted character string is not permitted.
 * @returns A character string that describes features that are not supported for this database.
 */
FUNCTION DBRESTRICTIONS RETURNS CHARACTER (
 INPUT database AS CHARACTER
 ) FORWARD.

/**
 * Returns a character string that describes features that are not supported for this database. You can use this function with OpenEdge DataServers.
 *
 * Syntax
 *
 * `DBRESTRICTIONS ( { integer-expression | logical-name | alias } [ , table-name ] )`
 *
 * @param db-seq The sequence number of a database the ABL session is connected to. For example, DBRESTRICTIONS(1) returns information on the first database the ABL session is connected to, DBRESTRICTIONS(2) returns information on the second database the ABL session is connected to, and so on. If you specify a sequence number that does not correspond to a database the ABL session is connected to, the DBRESTRICTIONS function returns the Unknown value (?).
 * @param table-name A character expression equal to the name of a table in the specified database. An unquoted character string is not permitted. If the table name is valid, DBRESTRICTIONS returns the list of unsupported features for the specified table.
 * @returns A character string that describes features that are not supported for this database.
 */
FUNCTION DBRESTRICTIONS RETURNS CHARACTER (
 INPUT db-seq AS INTEGER,
 INPUT table-name AS CHARACTER
 ) FORWARD.

/**
 * Returns a character string that describes features that are not supported for this database. You can use this function with OpenEdge DataServers.
 *
 * Syntax
 *
 * `DBRESTRICTIONS ( { integer-expression | logical-name | alias } [ , table-name ] )`
 *
 * @param db-seq The sequence number of a database the ABL session is connected to. For example, DBRESTRICTIONS(1) returns information on the first database the ABL session is connected to, DBRESTRICTIONS(2) returns information on the second database the ABL session is connected to, and so on. If you specify a sequence number that does not correspond to a database the ABL session is connected to, the DBRESTRICTIONS function returns the Unknown value (?).
 * @returns A character string that describes features that are not supported for this database.
 */
FUNCTION DBRESTRICTIONS RETURNS CHARACTER (
 INPUT db-seq AS INTEGER
 ) FORWARD.
