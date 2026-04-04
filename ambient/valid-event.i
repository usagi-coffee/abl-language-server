/**
 * Verifies whether a specified event is valid for a specified widget.
 *
 * Syntax
 *
 * `VALID-EVENT ( handle , event-name [ , platform ] )`
 *
 * Notes
 * - For each type of widget, only certain events are valid.
 * - Does not apply to SpeedScript programming.
 *
 * @param handle The handle of a valid widget.
 * @param event-name A character-string expression that evaluates to the name of an event.
 * @param platform A character-string expression that evaluates to the name of a platform type: GUI or TTY.
 * @returns Returns TRUE if the event is valid for the widget; otherwise returns FALSE.
 */
FUNCTION VALID-EVENT RETURNS LOGICAL (
    INPUT handle AS HANDLE,
    INPUT event-name AS CHARACTER,
    INPUT platform AS CHARACTER
  ) FORWARD.

FUNCTION VALID-EVENT RETURNS LOGICAL (
    INPUT handle AS HANDLE,
    INPUT event-name AS CHARACTER
  ) FORWARD.
