/**
 * Converts the specified data type to CHARACTER and encloses the results in quotes when necessary.
 * Intended for use in QUERY-PREPARE where a character predicate must be created from a concatenated
 * list of string variables to form a WHERE clause.
 *
 * Syntax
 *
 * `QUOTER ( expression [, quote-char [, null-string]] )`
 *
 * Notes
 * - Does not return Unknown value (?) if expression is unknown. Instead returns unquoted question-mark by default, or the third argument if present.
 * - For non-character data types (DECIMAL, INT64, INTEGER, DATE, DATETIME, DATETIME-TZ), converts to character in EXPORT-like format.
 * - For CHARACTER data, quotes that are part of the string are doubled.
 * - Can be used with object references to get unique object identifier as quoted string.
 * - DATE types always have the 4-digit year.
 * - Data types with no DISPLAY format like MEMPTR and LVARBINARY return the Unknown value (?).
 * - If expression is of type RAW, it is converted to base 64.
 * - When applied to an enum instance, returns the enumeration member name as a string.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE mychar AS CHARACTER NO-UNDO INITIAL "Lift Line Skiing".
 * qhandle:QUERY-PREPARE("FOR EACH Customer WHERE Customer.Name = " + QUOTER(mychar)).
 * ```
 *
 * @param expression The expression to convert to character and enclose with quotes.
 * @param quote-char Either a single or double quote character. The default is double quote.
 * @param null-string The string for unknown values. The default is an unquoted question mark.
 * @returns Returns a quoted character string representation of the expression.
 */
FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS CHARACTER,
    INPUT quote-char AS CHARACTER,
    INPUT null-string AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS CHARACTER,
    INPUT quote-char AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS DECIMAL,
    INPUT quote-char AS CHARACTER,
    INPUT null-string AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS DECIMAL,
    INPUT quote-char AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS DECIMAL
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS INTEGER,
    INPUT quote-char AS CHARACTER,
    INPUT null-string AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS INTEGER,
    INPUT quote-char AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS INTEGER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS INT64,
    INPUT quote-char AS CHARACTER,
    INPUT null-string AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS INT64,
    INPUT quote-char AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS INT64
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS DATE,
    INPUT quote-char AS CHARACTER,
    INPUT null-string AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS DATE,
    INPUT quote-char AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS DATE
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS DATETIME,
    INPUT quote-char AS CHARACTER,
    INPUT null-string AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS DATETIME,
    INPUT quote-char AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS DATETIME
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS DATETIME-TZ,
    INPUT quote-char AS CHARACTER,
    INPUT null-string AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS DATETIME-TZ,
    INPUT quote-char AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS DATETIME-TZ
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS LOGICAL,
    INPUT quote-char AS CHARACTER,
    INPUT null-string AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS LOGICAL,
    INPUT quote-char AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS LOGICAL
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS RAW,
    INPUT quote-char AS CHARACTER,
    INPUT null-string AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS RAW,
    INPUT quote-char AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS RAW
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS MEMPTR,
    INPUT quote-char AS CHARACTER,
    INPUT null-string AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS MEMPTR,
    INPUT quote-char AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS MEMPTR
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS LVARBINARY,
    INPUT quote-char AS CHARACTER,
    INPUT null-string AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS LVARBINARY,
    INPUT quote-char AS CHARACTER
  ) FORWARD.

FUNCTION QUOTER RETURNS CHARACTER (
    INPUT expression AS LVARBINARY
  ) FORWARD.
