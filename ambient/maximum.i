/**
 * Compares two or more values and returns the largest value.
 *
 * Syntax
 *
 * `MAXIMUM ( expression1 , expression2 [ , expression3 ] ... )`
 *
 * Notes
 * - When comparing character values, if at least one of the character fields is defined as case sensitive, then MAXIMUM treats all of the values as case sensitive for the sake of the comparisons. If none of the values is case sensitive, MAXIMUM treats lowercase letters as if they were uppercase letters.
 * - You cannot compare data of different DATE, DATETIME, and DATETIME-TZ data types to each other using MAXIMUM. You must first convert different date and datetime data types to the same data type before doing a comparison between them.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE cred-lim2 AS DECIMAL NO-UNDO FORMAT ">>,>>9.99".
 * FOR EACH Customer NO-LOCK:
 *   cred-lim2 = IF Customer.CreditLimit < 20000 THEN
 *     Customer.CreditLimit + 10000 ELSE 30000.
 *   DISPLAY Customer.CreditLimit cred-lim2
 *     MAXIMUM(cred-lim2, Customer.CreditLimit)
 *     LABEL "Maximum of these two values".
 * END.
 * ```
 *
 * @param expression-1 A constant, field name, variable name, or expression.
 * @param expression-2 A constant, field name, variable name, or expression.
 * @param expression-3 A constant, field name, variable name, or expression.
 * @returns Returns the largest value. If there is a mixture of decimal and integer data types, decimal type is returned.
 */
FUNCTION MAXIMUM RETURNS DECIMAL (
    INPUT expression-1 AS DECIMAL,
    INPUT expression-2 AS DECIMAL,
    INPUT expression-3 AS DECIMAL
  ) FORWARD.

FUNCTION MAXIMUM RETURNS DECIMAL (
    INPUT expression-1 AS DECIMAL,
    INPUT expression-2 AS DECIMAL
  ) FORWARD.
