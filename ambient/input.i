/**
 * References the value of a field in a frame.
 *
 * Syntax
 *
 * `INPUT [ FRAME frame ] field`
 *
 * Notes
 * - If you use a field referenced with INPUT in more than one frame, ABL uses the value in the frame most recently introduced. Use FRAME option to reference a particular frame.
 * - If INPUT is used for a character field whose format contains fill characters, the value does not contain the fill characters.
 *
 * Example
 *
 * ```abl
 * FOR EACH Customer:
 *     DISPLAY Customer.CustNum Customer.Name Customer.CreditLimit
 *         LABEL "Current credit limit"
 *         WITH FRAME a 1 DOWN ROW 1.
 *     PROMPT-FOR Customer.CreditLimit LABEL "New credit limit"
 *         WITH SIDE-LABELS NO-BOX ROW 10 FRAME b.
 *     IF INPUT FRAME b Customer.CreditLimit <> Customer.CreditLimit THEN DO:
 *         Customer.CreditLimit = INPUT FRAME b Customer.CreditLimit.
 *     END.
 * END.
 * ```
 *
 * @param frame-name The name of the frame that contains the field.
 * @param field-name The name of a field or variable whose value is stored in the screen buffer.
 * @returns The value of the specified field from the screen buffer.
 */
FUNCTION INPUT RETURNS CHARACTER (
    INPUT frame-name AS CHARACTER,
    INPUT field-name AS CHARACTER
  ) FORWARD.

FUNCTION INPUT RETURNS CHARACTER (
    INPUT field-name AS CHARACTER
  ) FORWARD.

FUNCTION INPUT RETURNS INTEGER (
    INPUT frame-name AS CHARACTER,
    INPUT field-name AS CHARACTER
  ) FORWARD.

FUNCTION INPUT RETURNS INTEGER (
    INPUT field-name AS CHARACTER
  ) FORWARD.

FUNCTION INPUT RETURNS DECIMAL (
    INPUT frame-name AS CHARACTER,
    INPUT field-name AS CHARACTER
  ) FORWARD.

FUNCTION INPUT RETURNS DECIMAL (
    INPUT field-name AS CHARACTER
  ) FORWARD.

FUNCTION INPUT RETURNS DATE (
    INPUT frame-name AS CHARACTER,
    INPUT field-name AS CHARACTER
  ) FORWARD.

FUNCTION INPUT RETURNS DATE (
    INPUT field-name AS CHARACTER
  ) FORWARD.

FUNCTION INPUT RETURNS LOGICAL (
    INPUT frame-name AS CHARACTER,
    INPUT field-name AS CHARACTER
  ) FORWARD.

FUNCTION INPUT RETURNS LOGICAL (
    INPUT field-name AS CHARACTER
  ) FORWARD.
