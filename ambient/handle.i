/**
 * Converts a string representation of a handle to a valid handle. If the string does not correlate to a valid handle
 * in the session, the function returns 0.
 *
 * Syntax
 *
 * `HANDLE ( handle-string )`
 *
 * Caution: Use this function only to convert a handle previously stored as a string value back to a valid handle.
 * If you convert an arbitrary string to handle using this function and then reference the new handle, a system
 * error will occur. If you use the VALID-HANDLE function to validate a handle generated from an arbitrary string
 * value, a system error will occur.
 *
 * Notes
 * - The HANDLE function can convert the string representation of procedure, system, and widget handles.
 * - For SpeedScript, the only valid use is to convert the handle of a QUERY object that you create using the
 * CREATE WIDGET statement.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE whand AS HANDLE NO-UNDO.
 * DEFINE VARIABLE chand AS CHARACTER NO-UNDO.
 * CREATE FRAME whand.
 * chand = STRING(whand).
 * DELETE WIDGET whand.
 * whand = HANDLE(chand).
 * MESSAGE VALID-HANDLE(whand) VIEW-AS ALERT-BOX INFORMATION BUTTONS OK.
 * ```
 *
 * @param handle-string A string representation of a handle. Since handles are integer values, the string must contain only numeric characters.
 * @returns A valid handle, or 0 if the string does not correlate to a valid handle in the session.
 */
FUNCTION HANDLE RETURNS HANDLE (
 handle-string AS CHARACTER
 ) FORWARD.
