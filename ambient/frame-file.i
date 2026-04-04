/**
 * Returns the name of the database table that contains the field the cursor is in.
 * The FRAME-FILE function is useful if you want to provide users with context-sensitive help.
 *
 * Syntax
 *
 * `FRAME-FILE`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 * - FRAME-FILE returns a null string if the frame field being entered is not associated with a database field.
 * - If the cursor is not in an enabled input field when the last input statement ends, FRAME-FILE returns a null string.
 * - The FRAME-FILE value is set to blanks at the next PAUSE statement, at the next READKEY statement, or when the AVM pauses automatically.
 *
 * Example
 *
 * ```abl
 * FOR EACH Customer, EACH Order OF Customer:
 * DISPLAY Order.OrderNum WITH CENTERED ROW 2 FRAME onum.
 * UPDATE
 * Customer.CustNum AT 5 Order.CustNum AT 30 SKIP
 * Customer.Name AT 5
 * Customer.City AT 5
 * Customer.State AT 5
 * Customer.PostalCode AT 5
 * WITH ROW 8 CENTERED 1 DOWN NO-LABELS EDITING:
 * MESSAGE "The field" FRAME-FIELD "is from the" FRAME-FILE "file".
 * READKEY.
 * APPLY LASTKEY.
 * END.
 * END.
 * ```
 *
 * @returns CHARACTER The name of the database table that contains the field the cursor is in.
 */
FUNCTION FRAME-FILE RETURNS CHARACTER (
 ) FORWARD.
