/**
 * Converts any data type into the LOGICAL data type.
 *
 * Syntax
 *
 * `LOGICAL ( expression [ , char-expression-format ] )`
 *
 * Notes
 * - If the value of expression is the Unknown value (?), the LOGICAL function returns the Unknown value (?).
 * - If expression is of type DECIMAL, INTEGER, INT64, DATE, DATETIME, DATETIME-TZ, or HANDLE, the function returns TRUE if the value of expression is nonzero. If the value of expression is 0, it returns FALSE. The second argument is ignored if present.
 * - If expression is of type LONGCHAR or CHARACTER, it returns TRUE or FALSE depending on the value in the expression and the format used. Whether or not char-expression-format is given, the case-insensitive values TRUE, FALSE, YES, NO, abbreviated to 1 character, are always accepted.
 * - If char-expression-format is given, it is validated. If it is not valid, an error message appears and the Unknown value (?) is returned.
 * - Data types such as RAW, MEMPTR, LVARBINARY, and so on return the Unknown value (?), but this is not considered an error.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE mychar AS CHARACTER NO-UNDO.
 * DEFINE VARIABLE v-log AS LOGICAL NO-UNDO.
 * mychar = "si".
 * v-log = LOGICAL(mychar, "si/no").
 * ```
 *
 * @param expression An expression in the data type that you want to convert to logical.
 * @param char-expression-format A character expression that evaluates to a valid logical format, such as "si/no", or "da/nyet". This argument is ignored unless expression is of CHARACTER type.
 * @returns Returns the LOGICAL value of the expression.
 */
FUNCTION LOGICAL RETURNS LOGICAL (
    INPUT expression AS CHARACTER,
    INPUT char-expression-format AS CHARACTER
  ) FORWARD.

FUNCTION LOGICAL RETURNS LOGICAL (
    INPUT expression AS CHARACTER
  ) FORWARD.

FUNCTION LOGICAL RETURNS LOGICAL (
    INPUT expression AS DECIMAL
  ) FORWARD.

FUNCTION LOGICAL RETURNS LOGICAL (
    INPUT expression AS INTEGER
  ) FORWARD.

FUNCTION LOGICAL RETURNS LOGICAL (
    INPUT expression AS INT64
  ) FORWARD.

FUNCTION LOGICAL RETURNS LOGICAL (
    INPUT expression AS DATE
  ) FORWARD.

FUNCTION LOGICAL RETURNS LOGICAL (
    INPUT expression AS DATETIME
  ) FORWARD.

FUNCTION LOGICAL RETURNS LOGICAL (
    INPUT expression AS DATETIME-TZ
  ) FORWARD.

FUNCTION LOGICAL RETURNS LOGICAL (
    INPUT expression AS HANDLE
  ) FORWARD.
