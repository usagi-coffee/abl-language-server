/**
 * Converts a value of any data type into a character value.
 *
 * Syntax
 *
 * `STRING ( source [ , format ] )`
 *
 * Notes
 * - The STRING function is double-byte enabled. The source argument can contain double-byte data.
 * - If source is an integer and format begins HH:MM or HH:MM:SS, STRING formats the source as a time.
 * - If source is a RAW value, you must specify an appropriate format to return the character string representation.
 * - When source is a DATETIME or DATETIME-TZ expression, the STRING function converts the expression to a character value in the specified format.
 * - The STRING function converts a DATE, and the date part of a DATETIME or DATETIME-TZ, using the format specified by the DATE-FORMAT attribute or the Date Format (-d) startup parameter.
 * - You can use the STRING function to convert an object reference for a class instance to a character value.
 * - When applied to an enum instance, the STRING function implicitly calls the ToString( ) method, which returns the name of the enumeration member.
 *
 * Example
 *
 * ```abl
 * DISPLAY STRING(TIME,"HH:MM AM").
 * DISPLAY STRING(Customer.CustNum, ">>>9").
 * ```
 *
 * @param source An expression of any data type that you want to convert to a character value.
 * @param format The format you want to use for the new character value. This format must be appropriate to the data type of source.
 * @returns A character value representing the converted source expression.
 */
FUNCTION STRING RETURNS CHARACTER (
    INPUT source AS CHARACTER,
    INPUT format AS CHARACTER
  ) FORWARD.

FUNCTION STRING RETURNS CHARACTER (
    INPUT source AS CHARACTER
  ) FORWARD.

/**
 * Returns the null-terminated character string at the specified memory location.
 *
 * Syntax
 *
 * `GET-STRING ( source , position [ , numbytes ] )`
 *
 * Notes
 * - If source is the Unknown value (?), GET-STRING returns the Unknown value (?).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE mptr AS MEMPTR NO-UNDO.
 * SET-SIZE(mptr) = 100.
 * PUT-STRING(mptr, 1) = "Hello World".
 * MESSAGE GET-STRING(mptr, 1).
 * ```
 *
 * @param source A function or variable that returns a RAW or MEMPTR value.
 * @param position An integer value greater than 0 that indicates the byte position where you want to find the information.
 * @param numbytes An integer value greater than 0 that indicates how many bytes to convert. If not specified or is -1, returns all bytes until a NULL value is encountered.
 * @returns A CHARACTER value containing the null-terminated string or specified number of bytes.
 */
FUNCTION GET-STRING RETURNS CHARACTER (
    INPUT source AS RAW,
    INPUT position AS INTEGER,
    INPUT numbytes AS INTEGER
  ) FORWARD.

FUNCTION GET-STRING RETURNS CHARACTER (
    INPUT source AS RAW,
    INPUT position AS INTEGER
  ) FORWARD.

FUNCTION GET-STRING RETURNS CHARACTER (
    INPUT source AS MEMPTR,
    INPUT position AS INTEGER,
    INPUT numbytes AS INTEGER
  ) FORWARD.

FUNCTION GET-STRING RETURNS CHARACTER (
    INPUT source AS MEMPTR,
    INPUT position AS INTEGER
  ) FORWARD.

/**
 * Extracts a portion of a character string from a field or variable.
 *
 * Syntax
 *
 * `SUBSTRING ( source , position [ , length [ , type ] ] )`
 *
 * Notes
 * - If position is less than 1, the result begins at the first character of source.
 * - If position is greater than the length of source, the result is the empty string ("").
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE str AS CHARACTER NO-UNDO INITIAL "Hello World".
 * MESSAGE SUBSTRING(str, 1, 5).
 * MESSAGE SUBSTRING(str, 7).
 * ```
 *
 * @param source A CHARACTER or LONGCHAR expression from which you want to extract characters or bytes.
 * @param position An integer expression that indicates the position of the first character to extract.
 * @param length An integer expression that indicates the number of characters to extract. If not specified or -1, uses the remainder of the string.
 * @param type A CHARACTER expression that directs ABL to interpret position and length as character units, bytes, or columns. Valid types: "CHARACTER", "FIXED", "COLUMN", and "RAW". Default is "CHARACTER".
 * @returns A CHARACTER or LONGCHAR value containing the extracted substring.
 */
FUNCTION SUBSTRING RETURNS CHARACTER (
    INPUT source AS CHARACTER,
    INPUT position AS INTEGER,
    INPUT length AS INTEGER,
    INPUT type AS CHARACTER
  ) FORWARD.

FUNCTION SUBSTRING RETURNS CHARACTER (
    INPUT source AS CHARACTER,
    INPUT position AS INTEGER,
    INPUT length AS INTEGER
  ) FORWARD.

FUNCTION SUBSTRING RETURNS CHARACTER (
    INPUT source AS CHARACTER,
    INPUT position AS INTEGER
  ) FORWARD.
