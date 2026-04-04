/**
 * Converts an expression of any data type, with the exception of BLOB, CLOB, and RAW, to a DECIMAL value.
 *
 * Syntax
 *
 * `DECIMAL ( expression )`
 *
 * Notes
 * - If expression is a CHARACTER, then it must be valid for conversion into a number. (For example, 1.67 is valid but 1.x3 is not valid.)
 * - If expression is LOGICAL, then the result is 0 if expression is FALSE and 1 if expression is TRUE.
 * - If expression is a DATE, then the result is the number of days from 1/1/4713 B.C. to that date.
 * - If the value of expression is the Unknown value (?), then the result is also the Unknown value (?).
 *
 * Example
 *
 * ```abl
 * Customer.CreditLimit = DECIMAL(new-max).
 * ```
 *
 * @param expression The expression to convert to DECIMAL.
 * @returns A DECIMAL value.
 */
FUNCTION DECIMAL RETURNS DECIMAL (
 INPUT expression AS CHARACTER
 ) FORWARD.
