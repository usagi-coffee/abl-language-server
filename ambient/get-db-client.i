/**
 * Returns the handle to a copy of the sealed client-principal object that represents the user identity for the specified
 * database connection.
 *
 * @syntax GET-DB-CLIENT([db-exp])
 *
 * @param db-exp An optional character expression that evaluates to a case-insensitive logical or alias name of an
 * OpenEdge RDBMS. This expression can be unspecified or evaluate to the Unknown value (?) only if there is a single
 * OpenEdge database connection, in which case the client-principal object handle is returned for that connection.
 *
 * @returns HANDLE - The handle to a copy of the sealed client-principal object.
 *
 * @notes You can use the client-principal object returned by this function to set the identity for this and other database
 * connections or ABL sessions using single sign-on (SSO) operations unless the object represents an OpenEdge default
 * connection identity. A default connection identity is set by establishing a database connection without specifying the
 * User ID (-U) and Password (-P) connection parameters, either on the AVM startup command line or as options of the
 * CONNECT statement.
 *
 * To seal a client principal with a default connection identity, OpenEdge creates a unique domain access code to seal
 * the client-principal object returned by this function. Sealing the client principal with an internal domain access code
 * provides backwards compatibility with previous OpenEdge releases that prohibit a database connection from being
 * reverted back to the connection's default user identity. Thus, you can use the GET-DB-CLIENT function to return a valid
 * client-principal object from a database connection with a default connection identity. However, because the domain
 * access code used to seal the object is not configured for any registered domain, you cannot use that client-principal
 * in single sign-on (SSO) operations to assign a default user identity to any ABL session or database connection,
 * including the connection for which GET-DB-CLIENT returned the client-principal.
 *
 * To avoid a memory leak, you must explicitly delete the client-principal object whose handle is returned by this
 * function when you no longer need it.
 *
 * @example
 * DEFINE INPUT PARAMETER cUserID AS CHARACTER NO-UNDO.
 * DEFINE INPUT PARAMETER cPasswd AS CHARACTER NO-UNDO.
 * DEFINE VARIABLE hCP AS HANDLE NO-UNDO.
 * CONNECT C:\OpenEdge\WRK\db\Sports2020
 * VALUE( "-U " + cUserID +
 * " -P " + "oech1::" + AUDIT-POLICY:ENCRYPT-AUDIT-MAC-KEY(cPasswd))
 * -H dbserver -S 1900 NO-ERROR.
 * ASSIGN hCP = GET-DB-CLIENT("Sports2020").
 * SECURITY-POLICY:LOAD-DOMAINS("Sports2020").
 * SECURITY-POLICY:SET-CLIENT(hCP).
 * DELETE OBJECT hCP.
 * ASSIGN hCP = ?.
 */
FUNCTION GET-DB-CLIENT RETURNS HANDLE
  (INPUT db-exp AS CHARACTER) FORWARD.

FUNCTION GET-DB-CLIENT RETURNS HANDLE () FORWARD.
