/**
 * Returns the digital certificate subject name for an OpenEdge database connected via TLS.
 *
 * Syntax
 *
 * `SSL-SERVER-NAME ( logical-database-name )`
 *
 * Example
 *
 * ```abl
 * SSL-SERVER-NAME (mydb)
 * ```
 *
 * @param logical-database-name A quoted character string or character expression that specifies the database by its logical name.
 * @returns The digital certificate subject name for an OpenEdge database connected via TLS, or the Unknown value (?) if no database connection exists or the connection is not using TLS.
 */
FUNCTION SSL-SERVER-NAME RETURNS CHARACTER (
    INPUT logical-database-name AS CHARACTER
  ) FORWARD.
