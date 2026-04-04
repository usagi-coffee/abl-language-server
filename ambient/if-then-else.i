/**
 * Evaluates and returns one of two expressions, depending on the value of a specified condition.
 *
 * Syntax
 *
 * `IF condition THEN expression1 ELSE expression2`
 *
 * Example
 *
 * ```abl
 * FOR EACH Customer NO-LOCK BY IF Customer.Balance > 10000 THEN 1
 * ELSE (IF Customer.Balance > 1000 THEN 2 ELSE 3) BY Customer.SalesRep:
 *   DISPLAY Customer.SalesRep Customer.Balance Customer.Name.
 * END.
 * ```
 *
 * @param condition An expression whose value is logical (TRUE or FALSE).
 * @param expression1 A constant, field name, variable name, or expression. If the condition is TRUE, then the function returns this value.
 * @param expression2 A constant, field name, variable name, or expression whose value is of a data type that is compatible with the data type of expression1. If the condition is FALSE or the Unknown value (?), then the function returns this value.
 * @returns Returns expression1 if condition is TRUE, otherwise returns expression2.
 */
FUNCTION IF-THEN-ELSE RETURNS CHARACTER (
    INPUT condition AS LOGICAL,
    INPUT expression1 AS CHARACTER,
    INPUT expression2 AS CHARACTER
  ) FORWARD.
