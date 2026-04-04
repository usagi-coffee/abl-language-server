/**
 * Converts a universally unique identifier (UUID) value into a globally unique identifier (GUID) value.
 *
 * Syntax
 *
 * `GUID [ ( UUID ) ]`
 *
 * Notes
 * - Returns a GUID as a 36-character string value consisting of 32 hexadecimal digits and 4 hyphens.
 * - If the specified UUID is not exactly 16 bytes in length, returns the Unknown value (?).
 * - If UUID is not specified, the AVM generates a UUID and then converts it into a GUID.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE MyUUID AS RAW NO-UNDO.
 * DEFINE VARIABLE vGUID AS CHARACTER NO-UNDO.
 * ASSIGN
 *   MyUUID = GENERATE-UUID
 *   vGUID = GUID(MyUUID).
 * ```
 *
 * @param uuid An optional 16-byte raw UUID value to be converted.
 * @returns A 36-character GUID string value.
 */
FUNCTION GUID RETURNS CHARACTER (
    INPUT uuid AS RAW
  ) FORWARD.

FUNCTION GUID RETURNS CHARACTER (
  ) FORWARD.
