/**
 * Returns a character representation of a DATE, DATETIME, or DATETIME-TZ that conforms to the ISO 8601 standard for date/time representations.
 *
 * Syntax
 *
 * `ISO-DATE ( expression )`
 *
 * Notes
 * - These formats are equivalent to the XML Schema date and dateTime formats.
 * - The following table lists the standard ISO formats for each data type:
 *   - DATE: YYYY-MM-DD
 *   - DATETIME: YYYY-MM-DDTHH:MM:SS.SSS
 *   - DATETIME-TZ: YYYY-MM-DDTHH:MM:SS.SSS+HH:MM
 *
 * @param expression An expression that evaluates to a DATE, DATETIME or DATETIME-TZ.
 * @returns Returns a character string in the standard ISO format of the data type.
 */
FUNCTION ISO-DATE RETURNS CHARACTER (
    INPUT expression AS DATE
  ) FORWARD.

FUNCTION ISO-DATE RETURNS CHARACTER (
    INPUT expression AS DATETIME
  ) FORWARD.

FUNCTION ISO-DATE RETURNS CHARACTER (
    INPUT expression AS DATETIME-TZ
  ) FORWARD.
