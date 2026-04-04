/**
 * Interprets one or more consecutive bits in an integer variable or field as an ABL integer value and returns that value.
 *
 * Syntax
 *
 * `GET-BITS( source , position , numbits )`
 *
 * Notes
 * - This function can return a value greater than 32 bits if source is an INT64.
 *
 * @param source An ABL integer variable.
 * @param position A variable or expression that returns an integer. This parameter designates the position of the lowest-order bit of the bits that are to be interpreted as an integer. Bits are numbered from 1 through the length of an integer; with 1 being the low-order bit. If position is greater than the length of an integer, the AVM returns the Unknown value (?). If position is less than 1, the AVM generates a run-time error.
 * @param numbits The number of bits to examine when generating the return value. If position plus numbits is greater than the length of an integer plus 1, the AVM generates a run-time error.
 * @returns Integer
 */
FUNCTION GET-BITS RETURNS INTEGER (
 INPUT source AS INTEGER,
 INPUT position AS INTEGER,
 INPUT numbits AS INTEGER
 ) FORWARD.
