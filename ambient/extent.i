/**
 * This function returns the size (extent) of an array field or variable as an INTEGER value.
 * More specifically, it returns:
 * - The number of elements for a field or variable defined as a determinate array
 * - The Unknown value (?) for a variable defined as an indeterminate array with no size
 * - The size that has been set for a variable defined as an indeterminate array
 * - Zero for a field or variable that is not an array
 *
 * Syntax
 *
 * `EXTENT ( array )`
 *
 * Notes
 * - The EXTENT function corresponds to the EXTENT attribute.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE int_value AS INTEGER NO-UNDO EXTENT 3 INITIAL [1, 2, 3].
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * DEFINE VARIABLE tot AS INTEGER NO-UNDO LABEL "The total is".
 * DO ix = 1 TO EXTENT(int_value):
 * tot = tot + int_value[ix].
 * END.
 * DISPLAY tot.
 * ```
 *
 * @param array Any array field or variable.
 * @returns The size (extent) of an array field or variable as an INTEGER value.
 */
FUNCTION EXTENT RETURNS INTEGER (
 array AS CHARACTER
 ) FORWARD.
