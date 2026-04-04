/**
 * Returns the name (as a character string) of the real tenant associated with a database connection.
 *
 * Syntax
 *
 * `TENANT-NAME ( [ database-name ] )`
 *
 * Notes
 * - For a super-tenant connection identity, this function returns the tenant name of the super tenant, not the effective tenant.
 * - If the database is not enabled for multi-tenancy, the function returns the empty string ("").
 * - If no database is specified and more than one database is connected, the AVM raises an error.
 * - If the database name is not a valid name for a connected database, the AVM raises an error.
 *
 * @param database-name A character expression that evaluates to a logical database name or database alias.
 * @returns Returns the name (as a character string) of the real tenant associated with a database connection.
 */
FUNCTION TENANT-NAME RETURNS CHARACTER (
    INPUT database-name AS CHARACTER
  ) FORWARD.

FUNCTION TENANT-NAME RETURNS CHARACTER (
  ) FORWARD.
