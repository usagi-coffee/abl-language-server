/**
 * Returns the tenant ID of the effective tenant as an integer.
 *
 * @param database-name A character expression that evaluates to a logical database name or database alias. If no database is specified and more than one database is connected, the AVM raises an error. If the database name is not a valid name for a connected database, the AVM raises an error.
 * @returns The tenant ID of the effective tenant.
 *
 * @syntax GET-EFFECTIVE-TENANT-ID( [ database-name ] )
 *
 * @notes
 * - If a super tenant executes this function and they have already executed the SET-EFFECTIVE-TENANT function, this function returns the tenant ID of any effective tenant that is in scope. If they have not executed the SET-EFFECTIVE-TENANT function or there is no effective tenant in scope, this function returns the default tenant ID (0).
 * - If a regular-tenant user executes this function, it returns the tenant ID associated with the user's tenancy.
 */
FUNCTION GET-EFFECTIVE-TENANT-ID RETURNS INTEGER (
    INPUT database-name AS CHARACTER
) FORWARD.

FUNCTION GET-EFFECTIVE-TENANT-ID RETURNS INTEGER (
) FORWARD.
