/**
 * Converts any lowercase characters in a CHARACTER or LONGCHAR expression to uppercase characters,
 * and returns the result. The data type of the returned value matches the data type of the expression passed to
 * the function.
 *
 * Syntax
 *
 * `CAPS ( expression )`
 *
 * Notes
 * - The CAPS function returns uppercase characters relative to the settings of the Internal Code Page
 * (-cpinternal) and Case Table (-cpcase) startup parameters. For more information on these parameters,
 * see Startup Command and Parameter Reference.
 * - The CAPS function is double-byte enabled. The specified expression can yield a string containing double-byte
 * characters; however, the CAPS function changes only single-byte characters in the string.
 *
 * Example
 *
 * ```abl
 * Customer.State = CAPS(Customer.State).
 * ```
 *
 * @param expression A constant, field name, variable name, or expression that results in a CHARACTER or LONGCHAR value.
 * @returns CHARACTER or LONGCHAR value with uppercase characters.
 */
FUNCTION CAPS RETURNS CHARACTER (
 expression AS CHARACTER
 ) FORWARD.
