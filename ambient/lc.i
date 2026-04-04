/**
 * Converts any uppercase characters in a CHARACTER or LONGCHAR expression to lowercase characters.
 *
 * Syntax
 *
 * `LC ( expression )`
 *
 * Notes
 * - The LC function returns lowercase characters relative to the settings of the Internal Code Page (-cpinternal) and Case Table (-cpcase) startup parameters.
 * - The LC function is double-byte enabled. The specified expression can yield a string containing double-byte characters; however, the LC function changes only single-byte characters in the string.
 *
 * Example
 *
 * ```abl
 * Customer.SalesRep = CAPS(SUBSTRING(Customer.SalesRep, 1, 1)) +
 *   LC(SUBSTRING(Customer.SalesRep, 2)).
 * ```
 *
 * @param expression A constant, field name, variable name, or expression that results in a CHARACTER or LONGCHAR value.
 * @returns The result with uppercase characters converted to lowercase. The data type matches the input expression.
 */
FUNCTION LC RETURNS CHARACTER (
    INPUT expression AS CHARACTER
  ) FORWARD.

FUNCTION LC RETURNS LONGCHAR (
    INPUT expression AS LONGCHAR
  ) FORWARD.
