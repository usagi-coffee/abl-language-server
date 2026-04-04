/**
 * GET-STRING function
 *
 * ## Description
 * Returns the null-terminated character string at the specified memory location as a CHARACTER value (not
 * including the null terminator) or the number of bytes specified starting from the specified memory location as
 * a CHARACTER value.
 *
 * ## Syntax
 * GET-STRING ( source , position [, numbytes ] )
 *
 * ## Notes
 * For more information on using the MEMPTR and RAW data types, see OpenEdge Programming Interfaces.
 *
 * ## Example
 * For examples of how to use the GET-STRING function, see the GET-BYTE function.
 *
 * @param source A function or variable that returns a RAW or MEMPTR value. If source is the Unknown value (?), GET-STRING returns the Unknown value (?).
 * @param position An integer value greater than 0 that indicates the byte position where you want to find the information. If position is greater than the length of source, the AVM returns the Unknown value (?). If position is less than 1, the AVM generates a run-time error.
 * @param numbytes An integer value greater than 0 that indicates how many bytes to convert into the CHARACTER value that is returned. If numbytes is not specified, or is -1, GET-STRING returns all bytes until it encounters a NULL value.
 * @returns CHARACTER - The null-terminated character string at the specified memory location, or the number of bytes specified.
 */
FUNCTION GET-STRING RETURNS CHARACTER (source AS RAW, position AS INTEGER) FORWARD.
FUNCTION GET-STRING RETURNS CHARACTER (source AS RAW, position AS INTEGER, numbytes AS INTEGER) FORWARD.
FUNCTION GET-STRING RETURNS CHARACTER (source AS MEMPTR, position AS INTEGER) FORWARD.
FUNCTION GET-STRING RETURNS CHARACTER (source AS MEMPTR, position AS INTEGER, numbytes AS INTEGER) FORWARD.
