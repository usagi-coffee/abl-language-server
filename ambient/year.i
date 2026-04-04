/**
 * Evaluates a date expression and returns the year value of that date, including the century, as an INTEGER value.
 *
 * Syntax
 *
 * `YEAR ( date )`
 *
 * `YEAR ( datetime-expression )`
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE outfmt AS CHARACTER NO-UNDO.
 * DEFINE VARIABLE orddate AS CHARACTER NO-UNDO
 *   LABEL "Order Date" FORMAT "x(10)".
 * FOR EACH Order NO-LOCK:
 *   ASSIGN
 *     outfmt = IF YEAR(Order.OrderDate) >= 2000 THEN
 *       "99/99/9999" ELSE "99/99/99"
 *     orddate = STRING(Order.OrderDate, outfmt).
 *   DISPLAY Order.OrderNum orddate Order.Terms.
 * END.
 * ```
 *
 * @param date A date expression for which you want to determine the year.
 * @param datetime-expression An expression that evaluates to a DATETIME or DATETIME-TZ.
 * @returns Returns the year value of the date, including the century, as an INTEGER.
 */
FUNCTION YEAR RETURNS INTEGER (
    INPUT date AS DATE
  ) FORWARD.

FUNCTION YEAR RETURNS INTEGER (
    INPUT datetime-expression AS DATETIME
  ) FORWARD.

FUNCTION YEAR RETURNS INTEGER (
    INPUT datetime-expression AS DATETIME-TZ
  ) FORWARD.
