/**
 * Returns the keyboard label (such as F1) of the key that performs a specified ABL function (such as GO).
 *
 * Syntax
 *
 * `KBLABEL ( key-function )`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 * - If you reassign a new function key for the key function with the ON statement, the KBLABEL function returns the new key.
 *
 * Example
 *
 * ```abl
 * STATUS INPUT "Enter data, then press " + KBLABEL("GO").
 * FOR EACH Customer:
 *   UPDATE Customer.CustNum Customer.Name.
 * END.
 * ```
 *
 * @param key-function An expression whose value is the name of the special ABL key function. If key-function is a constant, enclose it in quotation marks ("").
 * @returns Returns a character string indicating the keyboard label of the key that performs the specified ABL function.
 */
FUNCTION KBLABEL RETURNS CHARACTER (
    INPUT key-function AS CHARACTER
  ) FORWARD.
