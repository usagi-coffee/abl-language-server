/**
 * Returns the name of the effective tenant as a character string.
 *
 * Syntax
 *
 * `GET-EFFECTIVE-TENANT-NAME( [ database-name ] )`
 *
 * Notes
 * - If a super tenant executes this function and they have already executed the SET-EFFECTIVE-TENANT
 *   function, this function returns the name of any effective tenant that is in scope. If they have not
 *   executed the SET-EFFECTIVE-TENANT function or there is no effective tenant in scope, this function
 *   returns the name of the default tenant ("Default").
 * - If a regular-tenant user executes this function, it returns the tenant name associated with the user's tenancy.
 *
 * @param database-name A character expression that evaluates to a logical database name or database alias. If no database is specified and more than one database is connected, the AVM raises an error. If the database name is not a valid name for a connected database, the AVM raises an error.
 * @returns CHARACTER The name of the effective tenant.
 */
FUNCTION GET-EFFECTIVE-TENANT-NAME RETURNS CHARACTER (
    INPUT database-name AS CHARACTER
) FORWARD.

FUNCTION GET-EFFECTIVE-TENANT-NAME RETURNS CHARACTER (
) FORWARD.
