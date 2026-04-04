/**
 * During a data entry statement, returns the name of the input field the cursor is in. At other times, returns the
 * name of the input field the cursor was last in.
 *
 * The FRAME-FIELD function is particularly useful if you want to provide the user with help for the input field
 * being used.
 *
 * Syntax
 *
 * `FRAME-FIELD`
 *
 * Notes
 * - If the current or last input field is an array, FRAME-FIELD returns the name of the field but does not indicate
 * the array element that the input field represents. To display the array element, use the FRAME-INDEX function.
 * - If the cursor was not in an enabled input field when the last input statement ended, FRAME-FIELD returns
 * an empty string.
 *
 * Example
 *
 * ```abl
 * FOR EACH Customer:
 * UPDATE Customer.CustNum Customer.Name
 * WITH 1 DOWN 1 COLUMN CENTERED EDITING:
 * DISPLAY "You are editing field:" FRAME-FIELD SKIP
 * WITH FRAME a ROW 15 NO-LABELS CENTERED.
 * READKEY.
 * APPLY LASTKEY.
 * END.
 * END.
 * ```
 *
 * @returns CHARACTER - The name of the input field the cursor is in (or was last in).
 */
FUNCTION FRAME-FIELD RETURNS CHARACTER (
 ) FORWARD.
