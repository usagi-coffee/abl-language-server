/**
 * Returns a logical value indicating whether you can query a specified attribute or method for a specified widget.
 *
 * Syntax
 *
 * `CAN-QUERY ( handle , attribute-name )`
 *
 * Notes
 * - For SpeedScript, use with buffer-field, buffer-object, buffer, and query-object handles.
 *
 * Example
 *
 * ```abl
 * queryable = CAN-QUERY(temp-handle, attribute).
 * ```
 *
 * @param handle An expression that evaluates to a handle. The handle must refer to a valid widget.
 * @param attribute-name An expression that evaluates to a character-string value. The contents of the string must be an attribute or method name.
 * @returns A logical value indicating whether the attribute can be queried.
 */
FUNCTION CAN-QUERY RETURNS LOGICAL (
 INPUT handle AS HANDLE,
 INPUT attribute-name AS CHARACTER
 ) FORWARD.
