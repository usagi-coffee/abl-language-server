/**
 * Verifies that a handle is valid.
 *
 * Syntax
 *
 * `VALID-HANDLE ( handle )`
 *
 * Notes
 * - A handle becomes invalid if the associated widget or procedure is deleted or is out of scope.
 * - This function is useful when walking through a list of widgets or persistent procedures using the PREV-SIBLING or NEXT-SIBLING attributes.
 * - If a handle is valid, it can still point to an obsolete object.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE mywin AS HANDLE NO-UNDO.
 * CREATE WINDOW mywin ASSIGN
 *   VISIBLE = TRUE
 *   TITLE = "Second Window".
 * IF VALID-HANDLE(mywin) THEN
 *   DELETE WIDGET mywin.
 * ```
 *
 * @param handle An expression that evaluates to a value of type HANDLE.
 * @returns Returns TRUE if the handle represents an object that is currently valid, otherwise returns FALSE.
 */
FUNCTION VALID-HANDLE RETURNS LOGICAL (
    INPUT handle AS HANDLE
  ) FORWARD.
