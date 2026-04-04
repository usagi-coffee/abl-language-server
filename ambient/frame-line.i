/**
 * Returns an INTEGER value that represents the current logical line number in a down frame.
 * Note: Does not apply to SpeedScript programming.
 *
 * Syntax
 *
 * `FRAME-LINE [ ( frame ) ]`
 *
 * Notes
 * - If there is a down pending for a frame, the FRAME-LINE function returns a value equal to FRAME-LINE + 1.
 * - The FRAME-LINE function counts an underline row as a logical line. A logical line corresponds to one iteration in a down frame and can contain more than one physical line.
 * - The FRAME-LINE function returns a value of 0 if the frame is not in view when the function is evaluated.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ans AS LOGICAL NO-UNDO
 * LABEL "Do you want to delete this customer ?".
 *
 * FOR EACH Customer WITH 10 DOWN:
 * UPDATE Customer.CustNum Customer.Name Customer.CreditLimit.
 * UPDATE ans WITH ROW FRAME-ROW + 3 + FRAME-LINE + 5
 * COLUMN 10 SIDE-LABELS OVERLAY FRAME del-frame.
 * END.
 * ```
 *
 * @param frame The frame name that you are trying to determine a line number for. If you do not supply a frame name, the FRAME-LINE function uses the default frame for the block that contains the FRAME-LINE function.
 * @returns INTEGER value that represents the current logical line number in a down frame.
 */
FUNCTION FRAME-LINE RETURNS INTEGER (
 INPUT p-frame AS HANDLE
 ) FORWARD.
