/**
 * Returns a character string representing the user ID for the specified database connection identity.
 *
 * Syntax
 *
 * `USERID [ ( logical-dbname ) ]`
 *
 * Notes
 * - The user ID returned can be set for a database connection identity specified on the command line or when executing CONNECT, SECURITY-POLICY:SET-CLIENT(), SET-DB-CLIENT, or SETUSERID.
 * - For a single-tenant database connection, the user ID is non-qualified; for a multi-tenant database connection, it is fully qualified.
 * - Returns "" for a non-qualified user ID for the blank user name in the blank domain.
 * - Returns "user-name" for a non-qualified user ID for a specific user name in the blank domain.
 * - Returns "@domain-name" for a fully qualified user ID for the blank user name in a specific domain.
 * - Returns "user-name@domain-name" for a fully qualified user ID for a specific user in a specific domain.
 * - A compiler error occurs when no database is connected, or when omitting logical-dbname and more than one database is connected.
 * - When specifying logical-dbname, you must provide the name of the logical database, not the physical database.
 * - ABL user IDs in _User table accounts are case insensitive.
 *
 * Example
 *
 * ```abl
 * DISPLAY USERID("DICTDB") LABEL "You are logged in as" WITH SIDE-LABELS.
 * ```
 *
 * @param logical-dbname The logical name of the database from whose connection identity you want to retrieve the user ID.
 * @returns Returns a character string representing the user ID for the specified database connection identity.
 */
FUNCTION USERID RETURNS CHARACTER (
    INPUT logical-dbname AS CHARACTER
  ) FORWARD.

FUNCTION USERID RETURNS CHARACTER ( ) FORWARD.
