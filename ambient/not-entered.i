/**
 * Returns TRUE if a frame field was not modified during the last INSERT, PROMPT-FOR, SET, or UPDATE statement.
 *
 * Syntax
 *
 * `[ FRAME frame ] field NOT ENTERED`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 * - If you use a field or variable referenced with NOT ENTERED in more than one frame, then the AVM uses the value in the frame most recently introduced in the procedure. To make sure you are using the appropriate frame, use the FRAME option with the NOT ENTERED function to reference a particular frame.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE new-max NO-UNDO LIKE Customer.CreditLimit.
 * FOR EACH Customer:
 *   DISPLAY Customer.CustNum Customer.Name Customer.CreditLimit
 *     LABEL "current max credit"
 *     WITH FRAME a 1 DOWN ROW 1.
 *   SET new-max LABEL "new max credit"
 *     WITH SIDE-LABELS NO-BOX ROW 10 FRAME b.
 *   IF new-max NOT ENTERED OR new-max = Customer.CreditLimit THEN DO:
 *     DISPLAY "No Change In credit-limit" WITH FRAME d ROW 15.
 *     NEXT.
 *   END.
 * END.
 * ```
 *
 * @param field-name The name of the field or variable you are checking.
 * @param frame-name The frame name that contains the field.
 * @returns Returns TRUE if the field was not modified; otherwise FALSE.
 */
FUNCTION NOT-ENTERED RETURNS LOGICAL (
    INPUT field-name AS CHARACTER,
    INPUT frame-name AS CHARACTER
  ) FORWARD.

FUNCTION NOT-ENTERED RETURNS LOGICAL (
    INPUT field-name AS CHARACTER
  ) FORWARD.
