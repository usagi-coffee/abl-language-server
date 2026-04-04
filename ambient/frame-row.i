/**
 * Returns a DECIMAL value that represents the row position of the upper-left corner of a frame within its window.
 *
 * Syntax
 *
 * `FRAME-ROW [ ( frame ) ]`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 *
 * Example
 *
 * ```abl
 * FOR EACH Customer NO-LOCK:
 * DISPLAY Customer WITH FRAME cust-frame 2 COLUMNS
 * TITLE "CUSTOMER INFORMATION".
 * FOR EACH Order OF Customer NO-LOCK:
 * DISPLAY Order.OrderNum Order.OrderDate Order.ShipDate
 * Order.PromiseDate Order.Carrier Order.Instruction Order.PO
 * WITH 2 COLUMNS 1 DOWN OVERLAY TITLE "CUSTOMER'S ORDERS"
 * ROW FRAME-ROW(cust-frame) + 8 COLUMN FRAME-COL(cust-frame) + 1.
 * END.
 * END.
 * ```
 *
 * @param frame The name of the frame whose row position you are trying to determine. If you do not supply a frame name, the FRAME-ROW function uses the default frame for the block that contains the FRAME-ROW function. If the FRAME-ROW function is in a DO block, the function uses the default frame scoped to the block that contains the DO block.
 * @returns DECIMAL value that represents the row position of the upper-left corner of a frame within its window.
 */
FUNCTION FRAME-ROW RETURNS DECIMAL (
 INPUT p-frame AS HANDLE
 ) FORWARD.
