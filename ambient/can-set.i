/**
 * Returns a logical value indicating whether you can set a specified attribute for a specified widget.
 *
 * Syntax
 *
 * `CAN-SET ( handle , attribute-name )`
 *
 * Notes
 * - For a field-level widget, the CAN-SET function always returns TRUE for the FRAME attribute; however, you can set the frame attribute only if the widget is dynamic. Therefore, before setting the FRAME attribute for a widget, you can test that the operation is valid with a statement similar to the following: IF CAN-SET(my-handle, "FRAME") AND my-handle:DYNAMIC THEN my-handle:FRAME = frame-handle.
 * - For SpeedScript, use with buffer-field, buffer-object, buffer, and query-object handles.
 *
 * @param handle An expression that evaluates to a handle. The handle must refer to a valid widget.
 * @param attribute-name An expression that evaluates to a character-string value. The contents of the string must be an attribute name.
 * @returns A logical value indicating whether you can set a specified attribute for a specified widget.
 */
FUNCTION CAN-SET RETURNS LOGICAL (
 handle AS HANDLE,
 attribute-name AS CHARACTER
 ) FORWARD.
