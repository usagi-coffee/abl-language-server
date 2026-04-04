/**
 * Returns the name (as a character string) of the tenant that owns the current record in a specified buffer.
 *
 * Syntax
 *
 * `BUFFER-TENANT-NAME ( buffer-name )`
 *
 * Notes
 * - If the buffer is not populated with a record, this function returns the Unknown value (?). If the buffer
 * is for a shared table or a temp-table, the function returns the empty string ("").
 * - If the buffer contains a record that belongs to a tenant group and the user is a super tenant, the
 * function returns the tenant name of one of the members of the group. The tenant name returned is
 * indeterminate, but is always the name of a tenant in the group.
 * - For a regular tenant user, the record in the buffer always belongs to that tenant. So, the function
 * always returns the user's own tenant name. This is true even if the record belongs to a tenant group.
 * - BUFFER-TENANT-NAME can be used in a WHERE or TENANT-WHERE option as long as the buffer specified
 * by buffer-name is not the same as the buffer of the query or FOR EACH statement.
 *
 * @param buffer-name An identifier that specifies the name of a record buffer.
 * @returns CHARACTER The name of the tenant that owns the current record in the specified buffer.
 */
FUNCTION BUFFER-TENANT-NAME RETURNS CHARACTER (
 INPUT buffer-name AS HANDLE
 ) FORWARD.
