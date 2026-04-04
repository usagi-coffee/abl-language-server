/**
 * Evaluates a date expression and returns a month INTEGER value from 1 to 12, inclusive.
 *
 * Syntax
 *
 * `MONTH ( date )`
 *
 * `MONTH ( datetime-expression )`
 *
 * Example
 *
 * ```abl
 * FOR EACH Order NO-LOCK:
 * IF (MONTH(Order.PromiseDate) < MONTH(TODAY) OR
 * YEAR(Order.PromiseDate) < YEAR(TODAY)) AND Order.ShipDate = ? THEN
 * DISPLAY Order.OrderNum LABEL "Order Num" Order.PO LABEL "P.O. Num"
 * Order.PromiseDate LABEL "Promised By"
 * Order.OrderDate LABEL "Ordered" terms
 * WITH TITLE "These orders are overdue".
 * END.
 * ```
 *
 * @param date-expression A date expression where you want a month value.
 * @param datetime-expression An expression that evaluates to a DATETIME or DATETIME-TZ. The MONTH function returns the month of the date part of the DATETIME or DATETIME-TZ value.
 * @returns A month INTEGER value from 1 to 12, inclusive.
 */
FUNCTION MONTH RETURNS INTEGER (
    INPUT date-expression AS DATE,
    INPUT datetime-expression AS DATETIME
  ) FORWARD.

FUNCTION MONTH RETURNS INTEGER (
    INPUT date-expression AS DATE
  ) FORWARD.
