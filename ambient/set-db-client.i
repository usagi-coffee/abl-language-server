/**
 * Sets the user identity for a specified connected OpenEdge database using an unsealed or a sealed client-principal object.
 *
 * Syntax
 *
 * `SET-DB-CLIENT ( client-principal-handle [, database ] )`
 *
 * Notes
 * - For an unsealed client-principal object (in the INITIAL state), this function performs a user authentication operation.
 * - For a sealed client-principal object (in the LOGIN state), this function performs a single sign-on (SSO) operation.
 * - If the user identity is set successfully and the database is multi-tenant, the connection accesses the database through the tenant organization configured for the user's domain.
 * - If client-principal-handle is the Unknown value (?), the current identity of affected database connections remains unchanged, and the function unlocks and allows the connection identity to be set using the SET-CLIENT( ) method.
 * - For any errors while operating on database connections, SET-DB-CLIENT returns FALSE, records any errors in the ERROR-STATUS system handle, and leaves the current identity for a given database connection unchanged.
 * - Within a transaction on a multi-tenant database, any attempt to set an identity for the connection that changes the current database tenancy raises a run-time error.
 * - This function also checks the value of the LOGIN-EXPIRATION-TIMESTAMP attribute on the client-principal object.
 * - To set a connection identity through an SSO operation, the client-principal object must be sealed and set to the LOGIN state.
 * - After a user identity is set for a database connection, the AVM uses that identity to determine if the user has permission to access tables and fields in that particular database.
 * - You can also use this function, instead of the SETUSERID function, to set the user identity for a database connection whether or not the user account is in the _User table.
 * - Calling this method generates an audit event, and creates an audit record for the event in all connected audit-enabled databases according to each database's current audit policy settings.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE hCP AS HANDLE NO-UNDO.
 * CREATE CLIENT-PRINCIPAL hCP.
 * hCP:INITIALIZE(id,?,?,password).
 * IF NOT SET-DB-CLIENT(hCP,"DICTDB") THEN DO:
 *     MESSAGE "Sorry, userid/password is incorrect.".
 * END.
 * DELETE OBJECT hCP.
 * ```
 *
 * @param client-principal-handle A handle to a client-principal object.
 * @param database The sequence number, logical name, or alias of a connected OpenEdge database for which to set the user identity.
 * @returns Returns TRUE if the user identity is set successfully; if unsuccessful, returns FALSE and the connection identity remains unchanged.
 */
FUNCTION SET-DB-CLIENT RETURNS LOGICAL (
    INPUT client-principal-handle AS HANDLE,
    INPUT database AS INTEGER
  ) FORWARD.

FUNCTION SET-DB-CLIENT RETURNS LOGICAL (
    INPUT client-principal-handle AS HANDLE,
    INPUT database AS CHARACTER
  ) FORWARD.

FUNCTION SET-DB-CLIENT RETURNS LOGICAL (
    INPUT client-principal-handle AS HANDLE
  ) FORWARD.
