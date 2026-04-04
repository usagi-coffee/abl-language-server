/**
 * Takes any data type and returns an INT64 value, if the conversion is possible. This function takes most common data types except for RAW and MEMPTR.
 *
 * Syntax
 *
 * `INT64 ( expression )`
 *
 * Notes
 * - If the value of expression is a CHARACTER, it must be valid for conversion into a number (for example, "1.67" is valid, "1.x3" is not).
 * - If expression is an object reference (CLASS), the result is the ABL-generated ID for the class instance.
 * - If expression is a reference to an instance of an enum, the result is the underlying numeric value of that instance.
 * - If expression is a LOGICAL, the result is 0 if expression is FALSE and the result is 1 if expression is TRUE.
 * - If expression is a DATE, the result is the number of days from 1/1/4713 B.C. to that day. If expression is the Unknown value (?), the result is the Unknown value (?).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE mychar AS CHARACTER NO-UNDO INITIAL "2234.3".
 * DEFINE VARIABLE myint64 AS INT64 NO-UNDO.
 * myint64 = INT64(mychar).
 * ```
 *
 * @param expression A constant, field name, variable name, or expression whose value can be of any data type except for RAW and MEMPTR.
 * @returns An INT64 value representing the converted expression.
 */
FUNCTION INT64 RETURNS INT64 (
    INPUT expression AS CHARACTER
  ) FORWARD.

FUNCTION INT64 RETURNS INT64 (
    INPUT expression AS INTEGER
  ) FORWARD.

FUNCTION INT64 RETURNS INT64 (
    INPUT expression AS INT64
  ) FORWARD.

FUNCTION INT64 RETURNS INT64 (
    INPUT expression AS DECIMAL
  ) FORWARD.

FUNCTION INT64 RETURNS INT64 (
    INPUT expression AS LOGICAL
  ) FORWARD.

FUNCTION INT64 RETURNS INT64 (
    INPUT expression AS DATE
  ) FORWARD.

FUNCTION INT64 RETURNS INT64 (
    INPUT expression AS HANDLE
  ) FORWARD.
