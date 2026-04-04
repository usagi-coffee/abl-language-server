/**
 * Returns, as a character string, a comma-separated list of the parameters used to connect to the database.
 *
 * Syntax
 *
 * `DBPARAM ( integer-expression )`
 *
 * `DBPARAM ( logical-name | alias )`
 *
 * Notes
 * - A database must be connected for the DBPARAM function to work as described.
 * - If the CONNECT statement does not contain a -db (database) parameter, which is permissible, the string DBPARAM returns includes the -db parameter and the database name.
 * - If the CONNECT statement contains the -pf parameter, which refers to a parameter file, the string DBPARAM returns includes the parameters in the file without "-pf" or any reference to the file.
 * - If the CONNECT statement contains a userid and a password, the string DBPARAM returns includes only the userid.
 * - The database can connect through the CONNECT statement, the command line, or an auto-connection.
 *
 * Example
 *
 * ```abl
 * DISPLAY DBPARAM(1).
 * ```
 *
 * @param sequence_number The sequence number of a database the ABL session is connected to. For example, DBPARAM(1) returns information on the first database the ABL session is connected to.
 * @param logical_name_or_alias The alias or logical name of a connected database.
 * @returns A comma-separated list of the parameters used to connect to the database as a character string.
 */
FUNCTION DBPARAM RETURNS CHARACTER (
 INPUT sequence_number AS INTEGER
 ) FORWARD.

FUNCTION DBPARAM RETURNS CHARACTER (
 INPUT logical_name_or_alias AS CHARACTER
 ) FORWARD.
