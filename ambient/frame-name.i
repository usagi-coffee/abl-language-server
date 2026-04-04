/**
 * Returns the name of the frame that the cursor is in to a field that is enabled for input.
 *
 * Syntax
 *
 * `FRAME-NAME`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 * - The FRAME-NAME function returns an empty string for a frame that has not been named (the default frame). It also returns an empty string if the cursor is in a field that is not enabled for input.
 * - When using the FRAME-NAME function, you must place it logically following the Frame phrase where it is named.
 * - FRAME-NAME is especially useful for context-sensitive help.
 *
 * Example
 *
 * ```abl
 * FOR EACH Customer, EACH Order OF Customer:
 * DISPLAY Order.OrderNum WITH CENTERED ROW 2 FRAME onum.
 * UPDATE Customer.CustNum AT 5 Customer.Name AT 30 SKIP
 * WITH FRAME custfrm WITH CENTERED 1 DOWN EDITING:
 * DISPLAY " You are currently editing a frame called " FRAME-NAME
 * WITH FRAME d1 WITH 1 DOWN CENTERED.
 * READKEY.
 * APPLY LASTKEY.
 * END.
 * END.
 * ```
 *
 * @returns CHARACTER
 */
FUNCTION FRAME-NAME RETURNS CHARACTER FORWARD.
