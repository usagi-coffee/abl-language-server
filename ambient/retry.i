/**
 * Returns a TRUE value if the current block is being reprocessed after a previous UNDO, RETRY.
 *
 * Syntax
 *
 * `RETRY`
 *
 * Notes
 * - Using the RETRY function in a block turns off the default error processing, which results in no infinite loop protection for the block.
 *
 * Example
 *
 * ```abl
 * DEFINE VAR badState AS CHAR NO-UNDO.
 * FOR EACH Customer:
 *   IF RETRY THEN
 *   DO:
 *     /* So user can see the bad value they typed */
 *     Customer.State = badState.
 *   END.
 *   UPDATE City State PostalCode.
 *   IF NOT CAN-FIND(State WHERE State.State = Customer.State) THEN DO:
 *     MESSAGE "This is not a valid state"
 *       VIEW-AS ALERT-BOX INFORMATION BUTTONS OK.
 *     badState = Customer.State.
 *     UNDO, RETRY.
 *   END.
 * END.
 * ```
 *
 * @returns Returns TRUE if the current block is being reprocessed after a previous UNDO, RETRY; otherwise returns FALSE.
 */
FUNCTION RETRY RETURNS LOGICAL (
  ) FORWARD.
