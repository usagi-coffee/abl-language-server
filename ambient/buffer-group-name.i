/**
 * Returns the name (as a character string) of the tenant group to which the current record in a specified record
 * buffer belongs. If the buffer does not contain a record from a tenant group, the function returns the Unknown
 * value (?).
 *
 * Syntax
 *
 * `BUFFER-GROUP-NAME ( buffer-name )`
 *
 * Notes
 * - BUFFER-GROUP-NAME can be used in a WHERE or TENANT-WHERE option as long as the buffer specified
 * by buffer-name is not the same as the buffer of the query or FOR EACH statement.
 *
 * Example
 *
 * ```abl
 * BUFFER-GROUP-NAME ( Customer )
 * ```
 *
 * @param buffer-name An identifier that specifies the name of a record buffer.
 * @returns CHARACTER
 */
FUNCTION BUFFER-GROUP-NAME RETURNS CHARACTER (
 buffer-name AS CHARACTER
 ) FORWARD.
