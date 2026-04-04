/**
 * Returns an INTEGER value that uniquely identifies a database's transaction.
 *
 * Syntax
 *
 * `DBTASKID ( integer-expression | logical-name | alias )`
 *
 * Notes
 * - If the application is not in a transaction, DBTASKID returns the Unknown value (?).
 * - If the client is connected to two databases and both databases participate in the transaction, DBTASKID does not necessarily return the same value for each database. The value DBTASKID returns for a database is for that database only.
 * - DBTASKID returns the Unknown value (?) for DataServers and the temp-table database.
 * - DBTASKID is designed for database replication. When you create a log record for a transaction, you can call DBTASKID and store the transaction ID. When you load the transaction, you can group log records by transaction ID.
 *
 * @param integer-expression The sequence number of a database the ABL session is connected to. For example, DBTASKID(1) returns information on the first database the ABL session is connected to, DBTASKID(2) returns information on the second database the ABL session is connected to, etc. If you specify a sequence number that does not correspond to a database the ABL session is connected to, the DBTASKID function returns the Unknown value (?).
 * @param logical-name A character expression that evaluates to the logical name of a database that is connected to the current ABL session. If the character expression does not evaluate to the logical name of a connected database, DBTASKID returns the Unknown value (?).
 * @param alias A character expression that evaluates to the alias of a database that is connected to the current ABL session. If the character expression does not evaluate to the alias of a connected database, DBTASKID returns the Unknown value (?).
 * @returns An INTEGER value that uniquely identifies a database's transaction.
 */
FUNCTION DBTASKID RETURNS INTEGER (
 INPUT pid AS INTEGER
 ) FORWARD.

/**
 * Returns an INTEGER value that uniquely identifies a database's transaction.
 *
 * Syntax
 *
 * `DBTASKID ( integer-expression | logical-name | alias )`
 *
 * Notes
 * - If the application is not in a transaction, DBTASKID returns the Unknown value (?).
 * - If the client is connected to two databases and both databases participate in the transaction, DBTASKID does not necessarily return the same value for each database. The value DBTASKID returns for a database is for that database only.
 * - DBTASKID returns the Unknown value (?) for DataServers and the temp-table database.
 * - DBTASKID is designed for database replication. When you create a log record for a transaction, you can call DBTASKID and store the transaction ID. When you load the transaction, you can group log records by transaction ID.
 *
 * @param logical-name A character expression that evaluates to the logical name of a database that is connected to the current ABL session. If the character expression does not evaluate to the logical name of a connected database, DBTASKID returns the Unknown value (?).
 * @param alias A character expression that evaluates to the alias of a database that is connected to the current ABL session. If the character expression does not evaluate to the alias of a connected database, DBTASKID returns the Unknown value (?).
 * @returns An INTEGER value that uniquely identifies a database's transaction.
 */
FUNCTION DBTASKID RETURNS INTEGER (
 INPUT pdbname AS CHARACTER
 ) FORWARD.
