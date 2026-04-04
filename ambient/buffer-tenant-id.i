/**
 * Returns the tenant ID (as an integer) of the tenant that owns the current record in a specified buffer.
 *
 * Syntax
 *
 * `BUFFER-TENANT-ID ( buffer-name )`
 *
 * Notes
 * - BUFFER-TENANT-ID can be used in a WHERE or TENANT-WHERE option as long as the buffer specified
 * by buffer-name is not the same as the buffer of the query or FOR EACH statement.
 * - This function can be used by database triggers on the buffers passed to the trigger to get the tenant ID of
 * these buffers. This allows the application provider to code tenant-specific database triggers.
 *
 * @param buffer-name An identifier that specifies the name of a record buffer.
 * @returns INTEGER - The tenant ID of the tenant that owns the current record. If the buffer is not populated with
 * a record, this function returns the Unknown value (?). If the buffer is for a shared table, a temp-table, or for
 * the default tenant of a multi-tenant table, the function returns the value zero (0). If the buffer contains a record
 * for a tenant group and the user is a super tenant, the function returns the tenant ID of one of the members of the
 * group (indeterminate, but always the tenant ID of a tenant in the group). For a regular tenant, the record in the
 * buffer always belongs to that tenant, so the function always returns the user's own tenant ID (even if the record
 * belongs to a tenant group).
 */
FUNCTION BUFFER-TENANT-ID RETURNS INTEGER (
 buffer-name AS HANDLE
 ) FORWARD.
