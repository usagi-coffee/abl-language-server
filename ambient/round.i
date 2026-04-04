/**
 * Rounds a decimal expression to a specified number of places after the decimal point.
 *
 * Syntax
 *
 * `ROUND ( expression , precision )`
 *
 * Example
 *
 * ```abl
 * Customer.CreditLimit = ROUND((Customer.CreditLimit * 1.1) / 100, 0) * 100.
 * ```
 *
 * @param expression A decimal expression.
 * @param precision A non-negative integer expression whose value is the number of places you want in the decimal result.
 * @returns A decimal value rounded to the specified precision.
 */
FUNCTION ROUND RETURNS DECIMAL (
    INPUT expression AS DECIMAL,
    INPUT precision AS INTEGER
  ) FORWARD.
