/**
 * Returns a comma-separated list of attributes that can be set for an object or widget.
 *
 * Syntax
 *
 * `LIST-SET-ATTRS ( handle )`
 *
 * Notes
 * - When used with the SECURITY-POLICY handle, this function does not return the XCODE-SESSION-KEY writeable attribute because this attribute contains an encryption key for encrypted source code.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE wattr-list AS CHARACTER NO-UNDO.
 * wattr-list = LIST-SET-ATTRS(FOCUS).
 * ```
 *
 * @param handle A handle to a valid object or widget.
 * @returns A comma-separated list of the attributes that can be set for that object or widget.
 */
FUNCTION LIST-SET-ATTRS RETURNS CHARACTER (
    INPUT handle AS HANDLE
  ) FORWARD.
