/**
 * Returns the number of lines you can use to display frames. This value omits the space
 * used by the message area and status area.
 *
 * Syntax
 *
 * `SCREEN-LINES`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE nbrdown AS INTEGER NO-UNDO.
 * nbrdown = IF SCREEN-LINES > 21 THEN 7 ELSE 6.
 * FOR EACH Customer NO-LOCK WITH nbrdown DOWN:
 *   DISPLAY Customer.CustNum Customer.Name Customer.Address Customer.City
 *     Customer.State Customer.Country.
 * END.
 * ```
 *
 * @returns Returns, as an INTEGER value, the number of lines you can use to display frames.
 */
FUNCTION SCREEN-LINES RETURNS INTEGER (
  ) FORWARD.
