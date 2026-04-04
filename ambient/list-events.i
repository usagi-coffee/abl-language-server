/**
 * Returns a comma-separated list of the valid events for a specified object or widget.
 *
 * Syntax
 *
 * `LIST-EVENTS ( handle [ , platform ] )`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 * - Some events are valid only on certain platforms.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE event-list AS CHARACTER NO-UNDO
 *     VIEW-AS SELECTION-LIST INNER-CHARS 20 INNER-LINES 5 SCROLLBAR-VERTICAL.
 * event-list:LIST-ITEMS = LIST-EVENTS(FOCUS).
 * ```
 *
 * @param handle A handle to a valid object or widget.
 * @param platform A character-string value that specifies a display type. Valid values are GUI and TTY.
 * @returns A comma-separated list of the valid events for the specified object or widget.
 */
FUNCTION LIST-EVENTS RETURNS CHARACTER (
    INPUT handle AS HANDLE,
    INPUT platform AS CHARACTER
  ) FORWARD.

FUNCTION LIST-EVENTS RETURNS CHARACTER (
    INPUT handle AS HANDLE
  ) FORWARD.
