/**
 * Returns the group ID (as an integer) of the tenant group to which the current record in a specified record buffer
 * belongs. If the buffer does not contain a record from a tenant group, the function returns the Unknown value (?).
 *
 * Syntax
 *
 * `BUFFER-GROUP-ID ( buffer-name )`
 *
 * Notes
 * - BUFFER-GROUP-ID can be used in a WHERE or TENANT-WHERE option as long as the buffer specified by
 * buffer-name is not the same as the buffer of the query or FOR EACH statement.
 *
 * @param buffer-name An identifier that specifies the name of a record buffer.
 * @returns INTEGER - The group ID of the tenant group, or the Unknown value (?) if the buffer does not contain a record from a tenant group.
 */
FUNCTION BUFFER-GROUP-ID RETURNS INTEGER (
 buffer-name AS HANDLE
 ) FORWARD.
