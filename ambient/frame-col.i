/**
 * Returns a DECIMAL value that is the column position of the left corner of a frame within its window.
 *
 * Syntax
 *
 * `FRAME-COL [ ( frame ) ]`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 * - The FRAME-COL function returns a value of 0 if the frame you specify is not in view when the AVM evaluates the function.
 * - To convert the decimal value returned by FRAME-COL to an integer value, use the INTEGER function.
 *
 * Example
 *
 * ```abl
 * FOR EACH Customer NO-LOCK:
 * DISPLAY Customer WITH FRAME cust-frame 2 COLUMNS
 * TITLE "CUSTOMER INFORMATION".
 * FOR EACH Order OF Customer NO-LOCK:
 * DISPLAY Order.OrderNum Order.OrderDate Order.ShipDate Order.PromiseDate
 * Order.Carrier Order.Instructions Order.PO
 * WITH 2 COLUMNS 1 DOWN OVERLAY TITLE "CUSTOMER'S ORDERS"
 * ROW FRAME-ROW(cust-frame) + 8 COLUMN FRAME-COL(cust-frame) + 1.
 * END.
 * END.
 * ```
 *
 * @param frame The name of the frame whose column position you are trying to determine. If you do not supply a frame name, the FRAME-COL function uses the default frame for the block it is in.
 * @returns DECIMAL value that is the column position of the left corner of a frame within its window
 */
FUNCTION FRAME-COL RETURNS DECIMAL (
 INPUT frame-name AS CHARACTER
 ) FORWARD.
