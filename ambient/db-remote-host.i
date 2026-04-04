/**
 * Returns a character string containing the IP address of the database connection. The IP address format is
 * determined by the Internet Protocol used when the connection was established. A single input parameter
 * identifies the database, which can be either the logical database name or database number. If the database
 * is not connected using TCP/IP or the -ipver IPv6 startup parameter is not used, the function returns the
 * Unknown value (?).
 *
 * Syntax
 *
 * `DB-REMOTE-HOST ( { logical-name | integer-expression } )`
 *
 * Notes
 * - For more information on the Internet Protocol (IP) version (-ipver) startup parameter, see Startup Command
 * and Parameter Reference.
 *
 * Example
 *
 * ```abl
 * display DB-REMOTE-HOST(1).
 * ```
 *
 * @param logical-name A character expression specifying the logical name of a connected database.
 * @param integer-expression The sequence number of a connected database. For example, DB-REMOTE-HOST(1) returns
 * information on the first connected database, DB-REMOTE-HOST(2) returns information on the second
 * connected database, and so on.
 * @returns A character string containing the IP address of the database connection.
 */
FUNCTION DB-REMOTE-HOST RETURNS CHARACTER (
 INPUT pcLogicalName AS CHARACTER
 ) FORWARD.

FUNCTION DB-REMOTE-HOST RETURNS CHARACTER (
 INPUT piDbNum AS INTEGER
 ) FORWARD.
