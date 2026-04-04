/**
 * Returns a comma-separated list of objects and widget types that respond to a specified event.
 *
 * Syntax
 *
 * `LIST-WIDGETS ( event-name [ , platform ] )`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 * - Some events are valid only on certain platforms.
 * - If you omit the platform parameter, the AVM uses the platform for the current session.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE event-name AS CHARACTER NO-UNDO FORMAT "x(24)" LABEL "Event".
 * DEFINE VARIABLE widget-list AS CHARACTER NO-UNDO LABEL "Widgets"
 *   VIEW-AS SELECTION-LIST INNER-CHARS 24 INNER-LINES 6 SCROLLBAR-VERTICAL.
 * FORM event-name SKIP widget-list WITH FRAME main-frame SIDE-LABELS.
 * REPEAT WITH FRAME main-frame:
 *   DISABLE widget-list.
 *   SET event-name.
 *   widget-list:LIST-ITEMS = LIST-WIDGETS(event-name).
 *   DISPLAY widget-list.
 *   ENABLE widget-list.
 *   PAUSE.
 * END.
 * ```
 *
 * @param event-name A character-string expression that evaluates to an event name.
 * @param platform A character-string value that specifies a display type. Valid values are GUI and TTY.
 * @returns A comma-separated list of objects and widget types that respond to the specified event.
 */
FUNCTION LIST-WIDGETS RETURNS CHARACTER (
    INPUT event-name AS CHARACTER,
    INPUT platform AS CHARACTER
  ) FORWARD.

FUNCTION LIST-WIDGETS RETURNS CHARACTER (
    INPUT event-name AS CHARACTER
  ) FORWARD.
