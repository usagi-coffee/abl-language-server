/**
 * Checks whether a frame field has been modified during the last INSERT,
 * PROMPT-FOR, SET, or UPDATE statement for that field, and returns a TRUE
 * or FALSE result.
 *
 * Note: Does not apply to SpeedScript programming.
 *
 * Syntax
 *
 * `[ FRAME frame ] field ENTERED`
 *
 * Notes
 * - If you type blanks in a field where data has never been displayed, the
 * ENTERED function returns FALSE, a SET or ASSIGN statement does not update
 * the underlying field or variable. Also, if the AVM has marked a field as
 * entered, and the PROMPT-FOR statement prompts for the field again and you
 * do not enter any data, the AVM no longer considers the field entered.
 * - If you have changed the field's window value since the last INSERT,
 * PROMPT-FOR, SET, or UPDATE statement on that field, the ENTERED function
 * returns FALSE. For example, if you use the DISPLAY statement to change the
 * value of the field, ENTERED no longer returns TRUE.
 * - Before referencing a widget with the ENTERED function, you must scope the
 * frame that contains that widget.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE new-max NO-UNDO LIKE Customer.CreditLimit.
 * FOR EACH Customer:
 * DISPLAY Customer.CustNum Customer.Name Customer.CreditLimit
 * LABEL "Current credit limit"
 * WITH FRAME a 1 DOWN ROW 1.
 * SET new-max LABEL "New credit limit"
 * WITH SIDE-LABELS NO-BOX ROW 10 FRAME b.
 * IF new-max ENTERED THEN DO:
 * IF new-max <> Customer.CreditLimit THEN DO:
 * DISPLAY "Changing Credit Limit of" Customer.Name SKIP
 * "from" Customer.CreditLimit "to" new-max
 * WITH FRAME c ROW 15 NO-LABELS.
 * Customer.CreditLimit = new-max.
 * NEXT.
 * END.
 * END.
 * DISPLAY "No Change In Credit Limit" WITH FRAME d ROW 15.
 * END.
 * ```
 *
 * @param field The name of the frame field you are checking.
 * @param frame The frame name (optional).
 * @returns LOGICAL TRUE if the field has been modified, FALSE otherwise.
 */
FUNCTION ENTERED RETURNS LOGICAL (
 field AS HANDLE
 ) FORWARD.
