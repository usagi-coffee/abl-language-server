/**
 * Returns a comma-separated list of available drives.
 *
 * Syntax
 *
 * `OS-DRIVES`
 *
 * Notes
 * - Windows only. On platforms other than Windows, OS-DRIVES compiles and executes, but returns the empty string ("").
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE drives AS CHARACTER NO-UNDO LABEL "Select a Drive"
 *   VIEW-AS SELECTION-LIST INNER-CHARS 3 INNER-LINES 5.
 * DEFINE FRAME f drives.
 * drives:LIST-ITEMS = OS-DRIVES.
 * UPDATE drives WITH FRAME f.
 * MESSAGE "Files will be written to drive" INPUT drives:SCREEN-VALUE.
 * ```
 *
 * @returns A comma-separated list of available drives.
 */
FUNCTION OS-DRIVES RETURNS CHARACTER (
  ) FORWARD.
