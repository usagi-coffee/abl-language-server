/**
 * Returns the unsigned 1 byte value at the specified memory location as an INTEGER value.
 *
 * Syntax
 *
 * `GET-BYTE ( source , position )`
 *
 * Notes
 * - For more information on using the MEMPTR and RAW data types, see OpenEdge Programming Interfaces.
 * - You can use the alternative keyword GETBYTE instead of GET-BYTE.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * FOR EACH Customer:
 * ix = GET-BYTE(RAW(Customer.Name), 1).
 * IF ix = 83 THEN
 * DISPLAY Customer.Name.
 * END.
 * ```
 *
 * @param source A function or variable that returns a RAW or MEMPTR value. If source is the Unknown value (?), GET-BYTE returns the Unknown value (?).
 * @param position An integer value greater than 0 that indicates the byte position where you want to find the information. If position is greater than the length of source, the AVM returns the Unknown value (?). If position is less than 1, the AVM generates a run-time error.
 * @returns INTEGER
 */
FUNCTION GET-BYTE RETURNS INTEGER (
 source AS RAW,
 position AS INTEGER
 ) FORWARD.

FUNCTION GET-BYTE RETURNS INTEGER (
 source AS MEMPTR,
 position AS INTEGER
 ) FORWARD.
