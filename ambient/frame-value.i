/**
 * During a data entry statement, returns the (character string) value of the input field that the cursor is in to the current input field. At other times, returns the (character string) value of the input field the cursor was last in.
 * Note: Does not apply to SpeedScript programming.
 *
 * Syntax
 *
 * `FRAME-VALUE`
 *
 * Notes
 * - If the cursor is not in an enabled input field when the last input statement ends, FRAME-VALUE returns a null string.
 * - FRAME-VALUE is set to blanks at the next pause (done by a PAUSE statement or automatically by the AVM) or at the next READKEY statement.
 * - FRAME-VALUE returns strings. If you use FRAME-VALUE to return a number, you must convert it prior to numeric comparisons.
 *
 * Example
 *
 * ```abl
 * FOR EACH Customer:
 * DISPLAY Customer.CustNum Customer.Name Customer.Address Customer.City
 * Customer.State Customer.PostalCode
 * WITH 1 COLUMN DOWN COLUMN 20.
 * SET Customer.Address Customer.City Customer.State Customer.PostalCode.
 * END.
 * DISPLAY
 * "You were updating field:" FRAME-FIELD FORMAT "x(20)" SKIP
 * "Of file:" FRAME-FILE SKIP
 * "And it had the value:" FRAME-VALUE FORMAT "x(20)" SKIP(2)
 * "When you pressed the END-ERROR key."
 * WITH FRAME helper NO-LABELS COLUMN 20 ROW 14 NO-BOX.
 * ```
 *
 * @returns CHARACTER - The character string value of the input field the cursor is in or was last in.
 */
FUNCTION FRAME-VALUE RETURNS CHARACTER (
 ) FORWARD.
