/**
 * Returns the value of an aggregate expression that is calculated by an ACCUMULATE
 * or aggregate phrase of a DISPLAY statement.
 *
 * Syntax
 *
 * `ACCUM aggregate-phrase expression`
 *
 * Notes
 * - The expression you use in the ACCUMULATE or DISPLAY statement and the expression
 * you use in the ACCUM function must be in exactly the same form. (For example,
 * "on-hand * cost" and "cost * on-hand" are not in exactly the same form.)
 * - For the AVERAGE, SUB-AVERAGE, TOTAL, and SUB-TOTAL aggregate phrases, expression
 * must be numeric.
 *
 * Example
 *
 * ```abl
 * FOR EACH Order NO-LOCK:
 * FOR EACH OrderLine OF Order NO-LOCK:
 * ACCUMULATE OrderLine.Qty * OrderLine.Price (TOTAL).
 * DISPLAY (ACCUM TOTAL OrderLine.Qty * OrderLine.Price) LABEL "Accum Total".
 * END.
 * END.
 * ```
 *
 * @param aggregate-phrase A phrase that identifies the aggregate value it should return.
 * This is the syntax for aggregate-phrase:
 * `{ AVERAGE | COUNT | MAXIMUM | MINIMUM | TOTAL | SUB-AVERAGE |
 * SUB-COUNT | SUB-MAXIMUM | SUB-MINIMUM | SUB-TOTAL }
 * [ BY break-group ]`
 * @param expression An expression that was used in an earlier ACCUMULATE or DISPLAY statement.
 * @returns The value of an aggregate expression.
 */
FUNCTION ACCUM RETURNS DECIMAL (
 aggregate_phrase AS CHARACTER,
 expression AS DECIMAL
 ) FORWARD.
