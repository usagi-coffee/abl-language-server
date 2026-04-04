/**
 * Returns the tenant ID (as an integer) of the real tenant associated with a database connection.
 *
 * Syntax
 *
 * `TENANT-ID ( [ database-name ] )`
 *
 * Notes
 * - If the database is not enabled for multi-tenancy, the function returns zero (0).
 * - If no database is specified and more than one database is connected, the AVM raises an error.
 * - If the database name is not a valid name for a connected database, the AVM raises an error.
 * - The tenant ID of the default tenant is 0.
 * - The tenant ID of a non-default regular tenant is greater than 0 and that of a super tenant is less than 0.
 * - For a super-tenant connection identity, this function returns the tenant ID of the super tenant, not the effective tenant.
 *
 * @param database-name A character expression that evaluates to a logical database name or database alias.
 * @returns Returns the tenant ID (as an integer) of the real tenant associated with a database connection.
 */
FUNCTION TENANT-ID RETURNS INTEGER (
    INPUT database-name AS CHARACTER
  ) FORWARD.

FUNCTION TENANT-ID RETURNS INTEGER (
  ) FORWARD.
