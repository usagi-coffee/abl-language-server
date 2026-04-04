/**
 * Returns TRUE if, within an EDITING phrase, an APPLY statement results in a GO action.
 * The GO action is deferred until the end of the EDITING phrase.
 * This function is supported only for backward compatibility.
 *
 * Syntax
 *
 * `GO-PENDING`
 *
 * Notes
 * - This function is supported only for backward compatibility.
 * - Does not apply to SpeedScript programming.
 *
 * Example
 *
 * ```abl
 * REPEAT:
 *   PROMPT-FOR Customer.CustNum.
 *   FIND Customer USING Customer.CustNum.
 *   UPDATE
 *     Customer.Name Customer.Address Customer.City Customer.State SKIP
 *     Customer.CreditLimit Customer.Balance WITH 1 COLUMN EDITING:
 *     READKEY.
 *     APPLY LASTKEY.
 *     IF GO-PENDING AND INPUT Customer.Balance >
 *       INPUT Customer.CreditLimit THEN DO:
 *       MESSAGE "The current unpaid balance exceeds the credit limit.".
 *       NEXT.
 *     END.
 *   END. /* EDITING */
 * END.
 * ```
 *
 * @returns TRUE if an APPLY statement results in a GO action that is deferred until the end of the EDITING phrase.
 */
FUNCTION GO-PENDING RETURNS LOGICAL (
  ) FORWARD.
