/**
 * Generates a universally unique identifier (UUID), as a 16-byte RAW value.
 *
 * Syntax
 *
 * `GENERATE-UUID`
 *
 * Notes
 * - You can use the GENERATE-UUID function with the BASE64-ENCODE function to generate a UUID and convert it to use in a Base64 character index.
 * - You can also remove the two trailing Base64 pad characters to reduce the size of the UUID. For example: SUBSTRING(BASE64-ENCODE(GENERATE-UUID), 1, 22)
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE MyUUID AS RAW NO-UNDO.
 * DEFINE VARIABLE Base64UUID AS CHARACTER NO-UNDO.
 * ASSIGN
 * MyUUID = GENERATE-UUID
 * Base64UUID = BASE64-ENCODE(MyUUID).
 * ```
 *
 * @returns RAW A 16-byte RAW value containing the UUID.
 */
FUNCTION GENERATE-UUID RETURNS RAW (
 ) FORWARD.
