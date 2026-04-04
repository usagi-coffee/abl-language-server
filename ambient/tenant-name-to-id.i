/**
 * Returns the tenant ID associated with an input tenant name.
 *
 * Syntax
 *
 * `TENANT-NAME-TO-ID ( tenant-name [, database-name] )`
 *
 * Notes
 * - If a super tenant executes this function and tenant-name is not valid, the AVM raises an error.
 * - If a regular tenant executes this function, tenant-name must be the user's own tenant name or the AVM raises an error.
 *
 * @param tenant-name A character expression that evaluates to the name of a tenant.
 * @param database-name A character expression that evaluates to a logical database name or database alias. If no database is specified and more than one database is connected, the AVM raises an error.
 * @returns The tenant ID associated with an input tenant name.
 */
FUNCTION TENANT-NAME-TO-ID RETURNS INTEGER (
    INPUT tenant-name AS CHARACTER,
    INPUT database-name AS CHARACTER
  ) FORWARD.

FUNCTION TENANT-NAME-TO-ID RETURNS INTEGER (
    INPUT tenant-name AS CHARACTER
  ) FORWARD.
