/**
 * Converts an expression of any data type, with the exception of BLOB, CLOB, and RAW, to a 32-bit integer value of data type INTEGER, rounding that value if necessary.
 *
 * Syntax
 *
 * `INTEGER ( expression )`
 *
 * Notes
 * - If the value of expression is a CHARACTER, it must be valid for conversion into a number (for example, "1.67" is valid, "1.x3" is not).
 * - If expression is an object reference (CLASS), the result is the ABL-generated ID for the class instance.
 * - If expression is a reference to an instance of an enum, the result is the underlying numeric value of that instance. Because the underlying numeric values of enum members are of type INT64, the function may raise an error if the value cannot be represented as an INTEGER.
 * - If expression is a LOGICAL, the result is 0 if expression is FALSE and the result is 1 if expression is TRUE.
 * - If expression is a DATE, the result is the number of days from 1/1/4713 B.C. to that day.
 * - If expression is the Unknown value (?), the result is the Unknown value (?).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE street-number AS INTEGER NO-UNDO LABEL "Street Number".
 * FOR EACH Customer NO-LOCK:
 *   ASSIGN street-number = INTEGER(ENTRY(1, Customer.Address, " ")) NO-ERROR.
 *   IF ERROR-STATUS:ERROR THEN
 *     MESSAGE "Could not get street number of" Customer.Address.
 *   ELSE
 *     DISPLAY Customer.CustNum Customer.Address street-number.
 * END.
 * ```
 *
 * @param expression A constant, field name, variable name, or expression.
 * @returns A 32-bit integer value of data type INTEGER.
 */
FUNCTION INTEGER RETURNS INTEGER (
    INPUT expression AS CHARACTER
  ) FORWARD.

FUNCTION INTEGER RETURNS INTEGER (
    INPUT expression AS CLASS
  ) FORWARD.

FUNCTION INTEGER RETURNS INTEGER (
    INPUT expression AS LOGICAL
  ) FORWARD.

FUNCTION INTEGER RETURNS INTEGER (
    INPUT expression AS DATE
  ) FORWARD.

FUNCTION INTEGER RETURNS INTEGER (
    INPUT expression AS INTEGER
  ) FORWARD.

FUNCTION INTEGER RETURNS INTEGER (
    INPUT expression AS DECIMAL
  ) FORWARD.

FUNCTION INTEGER RETURNS INTEGER (
    INPUT expression AS INT64
  ) FORWARD.
