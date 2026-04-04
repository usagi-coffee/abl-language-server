/**
 * Authenticates a user identity for a specified database connection, verifying that the user ID and password
 * supplied to the SETUSERID function match a user account in the _User table of the database.
 *
 * Syntax
 *
 * `SETUSERID ( userid , password [, logical-dbname] )`
 *
 * Notes
 * - If the database is multi-tenant, it also sets the user's tenancy.
 * - If the user ID is not in the _User table or the password is incorrect, SETUSERID returns FALSE.
 * - Using this function overrides user identity previously set by SECURITY-POLICY:SET-CLIENT() or SET-DB-CLIENT.
 * - Within a transaction on a multi-tenant database, any attempt to set an identity that changes the current database tenancy raises a run-time error.
 * - To assign a user identity while making a database connection, use the CONNECT statement.
 * - This function does not generate any audit events, such as for login and logout.
 * - The domain of the user identity you want to set must be configured to use the _oeusertable authentication system.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE lSuccess AS LOGICAL NO-UNDO.
 * lSuccess = SETUSERID("user1", "password123").
 * ```
 *
 * @param userid The user's user ID. If the user is defined in a non-default domain, this value must be a fully qualified user ID including both the non-qualified user ID and domain separated by a domain delimiter (@).
 * @param password The user's password.
 * @param logical-dbname The logical name of the database on whose connection you want to check and set the user identity. If not specified, the compiler inserts the name of the database that is connected when the procedure is compiled.
 * @returns Returns TRUE if the user ID and password match a user account in the _User table, otherwise returns FALSE.
 */
FUNCTION SETUSERID RETURNS LOGICAL (
    INPUT userid AS CHARACTER,
    INPUT password AS CHARACTER,
    INPUT logical-dbname AS CHARACTER
  ) FORWARD.

FUNCTION SETUSERID RETURNS LOGICAL (
    INPUT userid AS CHARACTER,
    INPUT password AS CHARACTER
  ) FORWARD.
