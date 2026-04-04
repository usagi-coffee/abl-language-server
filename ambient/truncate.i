/**
 * Truncates a decimal expression to a specified number of decimal places, returning a decimal value.
 *
 * Syntax
 *
 * `TRUNCATE ( expression , decimal-places )`
 *
 * Notes
 * - You can use the TRUNCATE function to treat division as integer division. For example, i = TRUNCATE (x / y, 0).
 *
 * Example
 *
 * ```abl
 * Customer.CreditLimit = TRUNCATE((Customer.CreditLimit * 2) / 1000, 0) * 1000.
 * ```
 *
 * @param expression A decimal expression that you want to truncate.
 * @param decimal-places A non-negative integer expression that indicates the number of decimal places for a truncated expression.
 * @returns Returns a decimal value.
 */
FUNCTION TRUNCATE RETURNS DECIMAL (
    INPUT expression AS DECIMAL,
    INPUT decimal-places AS INTEGER
  ) FORWARD.
