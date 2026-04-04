/**
 * Returns an INTEGER value that represents the number of iterations in a frame.
 *
 * Syntax
 *
 * `FRAME-DOWN [ ( frame ) ]`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 * - The FRAME-DOWN function returns a value of 0 if used with a single frame or if the frame is not in view when the function is evaluated.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ans AS LOGICAL NO-UNDO.
 * REPEAT:
 * FIND NEXT Customer NO-LOCK.
 * DISPLAY Customer.CustNum Customer.Name.
 * IF FRAME-LINE = FRAME-DOWN THEN DO:
 * MESSAGE "Do you want to see the next page?" UPDATE ans.
 * IF NOT ans THEN LEAVE.
 * END.
 * END.
 * ```
 *
 * @param frame The name of the frame whose number down you are trying to determine. If you do not supply a frame name, the FRAME-DOWN function uses the default frame for the block it is in. If the FRAME-DOWN function is in a DO block, the function uses the default frame scoped to the block containing the DO block.
 * @returns INTEGER value that represents the number of iterations in a frame
 */
FUNCTION FRAME-DOWN RETURNS INTEGER (
 frame AS CHARACTER
 ) FORWARD.

/**
 * Returns an INTEGER value that represents the number of iterations in a frame.
 *
 * Syntax
 *
 * `FRAME-DOWN`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 * - The FRAME-DOWN function returns a value of 0 if used with a single frame or if the frame is not in view when the function is evaluated.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ans AS LOGICAL NO-UNDO.
 * REPEAT:
 * FIND NEXT Customer NO-LOCK.
 * DISPLAY Customer.CustNum Customer.Name.
 * IF FRAME-LINE = FRAME-DOWN THEN DO:
 * MESSAGE "Do you want to see the next page?" UPDATE ans.
 * IF NOT ans THEN LEAVE.
 * END.
 * END.
 * ```
 *
 * @returns INTEGER value that represents the number of iterations in a frame
 */
FUNCTION FRAME-DOWN RETURNS INTEGER (
 ) FORWARD.
