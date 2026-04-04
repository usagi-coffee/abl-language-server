/**
 * Sets the effective tenancy of a multi-tenant database connection for a super-tenant user.
 *
 * Syntax
 *
 * `SET-EFFECTIVE-TENANT ( tenant-expression [ , database-name ] )`
 *
 * Notes
 * - The function returns a logical TRUE if successful, and raises an error if not successful.
 * - If a regular-tenant user executes this function, the tenant identified by tenant-expression must match the user's tenancy; otherwise, the function raises an error.
 * - All super-tenant users login with access to shared data and data owned by the effective default tenant.
 * - When you execute SET-EFFECTIVE-TENANT, buffers and cursors for existing tenant data from a previous effective tenancy are not cleared.
 * - The scope of SET-EFFECTIVE-TENANT ends when a new identity is set for the database connection.
 * - An UNDO does not undo the tenancy of a SET-EFFECTIVE-TENANT.
 *
 * @param tenant-expression An integer or character string expression representing a valid tenant ID or tenant name.
 * @param database-name A character expression that evaluates to a logical database name or database alias.
 * @returns Returns a logical TRUE if successful, and raises an error if not successful.
 */
FUNCTION SET-EFFECTIVE-TENANT RETURNS LOGICAL (
    INPUT tenant-expression AS INTEGER,
    INPUT database-name AS CHARACTER
  ) FORWARD.

FUNCTION SET-EFFECTIVE-TENANT RETURNS LOGICAL (
    INPUT tenant-expression AS CHARACTER,
    INPUT database-name AS CHARACTER
  ) FORWARD.

FUNCTION SET-EFFECTIVE-TENANT RETURNS LOGICAL (
    INPUT tenant-expression AS INTEGER
  ) FORWARD.

FUNCTION SET-EFFECTIVE-TENANT RETURNS LOGICAL (
    INPUT tenant-expression AS CHARACTER
  ) FORWARD.
