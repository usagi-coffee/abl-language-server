/**
 * GET-FLOAT function
 *
 * Returns the 4-byte floating-point value at the specified memory location as a DECIMAL value.
 *
 * @param source A function or variable that returns a RAW or MEMPTR value. If source is the Unknown value (?),
 * GET-FLOAT returns the Unknown value (?).
 * @param position An integer value greater than 0 that indicates the byte position where you want to find the information.
 * If position is greater than the length of source, the AVM returns the Unknown value (?). If
 * position is less than 1, the AVM generates a run-time error.
 * @returns DECIMAL value
 *
 * @notes
 * - This function supports byte-swapping only if source is a MEMPTR data type. The function will first examine
 *   the byte-order setting of the MEMPTR and then swap the bytes appropriately before interpreting them. The
 *   AVM does not swap the bytes in the MEMPTR's memory, but does the byte-swap as it creates the return value.
 * - For more information on using the MEMPTR and RAW data types, see OpenEdge Programming Interfaces.
 */
FUNCTION GET-FLOAT RETURNS DECIMAL (source AS RAW, position AS INTEGER) FORWARD.
FUNCTION GET-FLOAT RETURNS DECIMAL (source AS MEMPTR, position AS INTEGER) FORWARD.
