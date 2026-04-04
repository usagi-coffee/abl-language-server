/**
 * Takes no arguments; returns the number of connected databases as an INTEGER value.
 *
 * Syntax
 *
 * `NUM-DBS`
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * REPEAT ix = 1 TO NUM-DBS:
 *     DISPLAY LDBNAME(ix) DBRESTRICTIONS(ix) FORMAT "x(40)".
 * END.
 * ```
 *
 * @returns Returns the number of connected databases as an INTEGER value.
 */
FUNCTION NUM-DBS RETURNS INTEGER (
  ) FORWARD.
