/**
 * Returns the unsigned 16-bit value at the specified memory location as an INTEGER value.
 *
 * Syntax
 *
 * `GET-UNSIGNED-SHORT ( source , position )`
 *
 * Notes
 * - This function supports byte-swapping only if source is a MEMPTR data type. The function will first examine
 * the byte-order setting of the MEMPTR and then swap the bytes appropriately before interpreting them. The
 * AVM does not swap the bytes in the MEMPTR's memory, but does the byte-swap as it creates the return value.
 * - For more information on using the MEMPTR and RAW data types, see OpenEdge Programming Interfaces.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * FOR EACH Customer:
 * ix = GET-UNSIGNED-SHORT(RAW(Customer.Name), 1).
 * IF ix = 83 THEN
 * DISPLAY Customer.Name.
 * END.
 * ```
 *
 * @param source A function or variable that returns a RAW or MEMPTR value. If source is the Unknown value (?), GET-UNSIGNED-SHORT returns the Unknown value (?).
 * @param position An integer value greater than 0 that indicates the byte position where you want to find the information. If position is greater than the length of source, the AVM returns the Unknown value (?). If position is less than 1, the AVM generates a run-time error.
 * @returns INTEGER
 */
FUNCTION GET-UNSIGNED-SHORT RETURNS INTEGER (
 source AS RAW,
 position AS INTEGER
 ) FORWARD.

FUNCTION GET-UNSIGNED-SHORT RETURNS INTEGER (
 source AS MEMPTR,
 position AS INTEGER
 ) FORWARD.
