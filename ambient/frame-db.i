/**
 * Returns the logical database name of the database that contains any field in which the user-interface cursor is entered.
 * Note: Does not apply to SpeedScript programming.
 *
 * Syntax
 *
 * `FRAME-DB`
 *
 * The function requires no arguments. If the cursor is in a field that is not a database field, this function returns no value for the field.
 *
 * Notes
 * - If the cursor is not in an enabled input field when the last input statement is executed, or the input field is not associated with a database field, FRAME-DB returns an empty string.
 * - Use this syntax to find the name of a schema holder for a non-OpenEdge database: SDBNAME ( FRAME-DB )
 *
 * Example
 *
 * ```abl
 * FOR EACH Customer NO-LOCK:
 * UPDATE Customer.CustNum Customer.Name Customer.Address Customer.Address2
 * Customer.City Customer.State Customer.PostalCode
 * WITH 1 DOWN 1 COLUMN CENTERED EDITING:
 * DISPLAY "You are editing field: " FRAME-FIELD SKIP
 * " of file: " FRAME-FILE SKIP
 * " in database: " FRAME-DB
 * WITH FRAME a ROW 15 NO-LABELS CENTERED.
 * READKEY.
 * APPLY LASTKEY.
 * END.
 * END.
 * ```
 *
 * @returns CHARACTER - The logical database name of the database containing the field where the cursor is entered. Returns an empty string if the cursor is not in an enabled input field or if the field is not associated with a database field.
 */
FUNCTION FRAME-DB RETURNS CHARACTER (
 ) FORWARD.
