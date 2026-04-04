/**
 * Converts a string representation of a handle to a valid handle.
 *
 * Syntax
 *
 * `WIDGET-HANDLE ( handle-string )`
 *
 * Notes
 * - This function is supported only for backward compatibility. Use the HANDLE function instead.
 * - Use this function only to convert a handle previously stored as a string value back to a valid handle.
 * - If you convert an arbitrary string to handle using this function and then reference the new handle, a system error will occur.
 * - If you use the VALID-HANDLE function to validate a handle generated from an arbitrary string value, a system error will occur.
 *
 * @param handle-string A string representation of a handle. Since handles are integer values, the string must contain only numeric characters.
 * @returns A valid handle.
 */
FUNCTION WIDGET-HANDLE RETURNS HANDLE (
    INPUT handle-string AS CHARACTER
  ) FORWARD.
