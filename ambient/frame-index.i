/**
 * During a data entry statement, returns the subscript of the array element of the input field that the cursor is in
 * as an INTEGER value. At other times, returns the subscript of the array element the cursor was in.
 * The FRAME-INDEX function is particularly useful if you want to provide the user with help for the input array
 * element being edited.
 *
 * Syntax
 *
 * `FRAME-INDEX`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 * - If the cursor is not in an enabled input field when the last input statement is executed, FRAME-INDEX
 * returns a 0. For example, FRAME-INDEX returns 0 if the user presses END-ERROR on the previous input
 * statement.
 * - The FRAME-INDEX value is set to 0 at the next pause (done by a PAUSE statement or automatically by
 * the AVM) or at the next READKEY statement.
 * - If you are updating a subset of an array-for example, TEXT(array-field[i TO 12]), and you use a
 * variable subscript (i), then FRAME-INDEX returns 0. If you use a constant subscript,
 * TEXT(array-field[6 TO 12]), then FRAME-INDEX returns the correct value.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE menu AS CHARACTER NO-UNDO EXTENT 3.
 * DO WHILE TRUE:
 * DISPLAY
 * "1. Display Customer Data" @ menu[1] SKIP
 * "2. Display Order Data" @ menu[2] SKIP
 * "3. Exit" @ menu[3] SKIP
 * WITH FRAME choices NO-LABELS.
 * CHOOSE FIELD menu AUTO-RETURN WITH FRAME choices
 * TITLE "Demonstration Menu" WITH CENTERED ROW 10.
 * HIDE FRAME choices.
 * IF FRAME-INDEX EQ 1 THEN
 * MESSAGE "You picked option 1.".
 * ELSE IF FRAME-INDEX EQ 2 THEN
 * MESSAGE "You picked option 2.".
 * ELSE IF FRAME-INDEX EQ 3 THEN LEAVE.
 * END.
 * ```
 *
 * @returns INTEGER - The subscript of the array element of the input field that the cursor is in
 */
FUNCTION FRAME-INDEX RETURNS INTEGER (
 ) FORWARD.
