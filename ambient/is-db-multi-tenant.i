/**
 * Returns TRUE if a specified database is multi-tenant enabled, and returns FALSE if it is not.
 *
 * Syntax
 *
 * `IS-DB-MULTI-TENANT( [ database-name ] )`
 *
 * Notes
 * - If no database is specified and more than one database is connected, the AVM raises an error.
 * - If the database name is not a valid name for a connected database, the AVM raises an error.
 * - If the value of database-name is the Unknown value (?), the function returns the Unknown value (?).
 *
 * Example
 *
 * ```abl
 * IF IS-DB-MULTI-TENANT("sports2000") THEN
 * MESSAGE "Database is multi-tenant enabled".
 * ELSE
 * MESSAGE "Database is not multi-tenant enabled".
 * ```
 *
 * @param database-name A character expression that evaluates to a logical database name or database alias.
 * If no database is specified and more than one database is connected, the AVM raises an error.
 * @returns LOGICAL - TRUE if the specified database is multi-tenant enabled, FALSE if it is not.
 */
FUNCTION IS-DB-MULTI-TENANT RETURNS LOGICAL (
 INPUT database-name AS CHARACTER
 ) FORWARD.

FUNCTION IS-DB-MULTI-TENANT RETURNS LOGICAL (
 ) FORWARD.
