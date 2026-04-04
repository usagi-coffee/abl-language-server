/**
 * Returns the unsigned 32-bit value at the specified memory location as an INT64. This is analogous to the
 * GET-UNSIGNED-SHORT function, except with a 32-bit value.
 *
 * Syntax
 *
 * `GET-UNSIGNED-LONG ( source , position )`
 *
 * Notes
 * - When returning the value from GET-UNSIGNED-LONG() to an INTEGER, if the value exceeds the maximum
 *   value of an INTEGER, the AVM generates a run-time error.
 * - This function supports byte-swapping only if source is a MEMPTR data type. The function will first examine
 *   the byte-order setting of the MEMPTR and then swap the bytes appropriately before interpreting them. The
 *   AVM does not swap the bytes in the MEMPTR's memory, but does the byte-swap as it creates the return value.
 * - For more information on using the MEMPTR and RAW data types, see OpenEdge Programming Interfaces.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INT64 NO-UNDO.
 * FOR EACH Customer:
 *   ix = GET-UNSIGNED-LONG(RAW(Customer.Name), 1).
 *   IF ix = 83 THEN
 *     DISPLAY Customer.Name.
 * END.
 * ```
 *
 * @param source A function or variable that returns a RAW or MEMPTR value. If source is the Unknown value (?), GET-UNSIGNED-LONG returns the Unknown value (?).
 * @param position An INTEGER or INT64 value greater than 0 that indicates the byte position where you want to find the information. If position is greater than the length of source, the AVM returns the Unknown value (?). If position is less than 1, the AVM generates a run-time error.
 * @returns INT64
 */
FUNCTION GET-UNSIGNED-LONG RETURNS INT64 (
  source AS RAW,
  position AS INTEGER
  ) FORWARD.

FUNCTION GET-UNSIGNED-LONG RETURNS INT64 (
  source AS MEMPTR,
  position AS INTEGER
  ) FORWARD.

FUNCTION GET-UNSIGNED-LONG RETURNS INT64 (
  source AS RAW,
  position AS INT64
  ) FORWARD.

FUNCTION GET-UNSIGNED-LONG RETURNS INT64 (
  source AS MEMPTR,
  position AS INT64
  ) FORWARD.
