/**
 * Returns a comma-separated list of attributes and methods that are supported for an object or widget.
 *
 * Syntax
 *
 * `LIST-QUERY-ATTRS ( handle )`
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE qattr-list AS CHARACTER NO-UNDO.
 * qattr-list = LIST-QUERY-ATTRS(FOCUS).
 * ```
 *
 * @param handle A handle to a valid object or widget.
 * @returns A comma-separated list of the attributes and methods that are supported for the object or widget.
 */
FUNCTION LIST-QUERY-ATTRS RETURNS CHARACTER (
    INPUT handle AS HANDLE
  ) FORWARD.
