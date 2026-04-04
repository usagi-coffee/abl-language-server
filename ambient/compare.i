/**
 * The COMPARE function compares two strings and lets you perform a raw compare,
 * use a particular collation, and turn case sensitivity on and off. COMPARE
 * returns a LOGICAL value representing the result of the logical expression, where
 * the comparison rules are defined by the combination of the operator, the
 * comparison strength, and the collation.
 *
 * Syntax
 *
 * `COMPARE ( string1 , relational-operator , string2 , strength [ , collation ] )`
 *
 * Notes
 * - If either or both strings evaluate to the Unknown value (?), COMPARE returns the value
 * indicated in the following table:
 * - LT (or <): Only one string = FALSE, Both strings = FALSE
 * - LE (or <=): Only one string = FALSE, Both strings = TRUE
 * - EQ (or =): Only one string = FALSE, Both strings = TRUE
 * - GE (or >=): Only one string = FALSE, Both strings = TRUE
 * - GT (or >): Only one string = FALSE, Both strings = FALSE
 * - NE (or <>): Only one string = TRUE, Both strings = FALSE
 * - BEGINS: Only one string = FALSE, Both strings = TRUE
 * - MATCHES: Only one string = FALSE, Both strings = TRUE
 * - COMPARE returns the Unknown value (?) if one of the following occurs:
 * - relational-operator does not evaluate to a valid value.
 * - strength does not evaluate to a valid value.
 * - collation does not evaluate to a collation table residing in the convmap.cp file.
 * - collation evaluates to a collation table that is not defined for the code page
 * corresponding to the -cpinternal startup parameter.
 * - LONGCHAR variable values are converted to -cpinternal for comparison and must
 * convert without error, or the AVM returns an error.
 * - With BEGINS, the language-sensitive rules are used only when strength is not RAW or CAPS.
 * - With MATCHES, CASE-SENSITIVE is treated as RAW, CASE-INSENSITIVE is treated as CAPS,
 * and the collation is never used.
 *
 * @param string1 A CHARACTER or LONGCHAR expression that evaluates to the first string to be compared.
 * @param relational-operator A CHARACTER expression that evaluates to one of the relational operators: LT (or <), LE (or <=), EQ (or =), GE (or >=), GT (or >), NE (or <>), "BEGINS", and "MATCHES".
 * @param string2 A CHARACTER expression that evaluates to the second string to be compared.
 * @param strength A CHARACTER expression that evaluates to the ABL comparison strength (RAW, CASE-SENSITIVE, CASE-INSENSITIVE, CAPS) or ICU comparison strength (PRIMARY, SECONDARY, TERTIARY, QUATERNARY).
 * @param collation A CHARACTER expression that evaluates to the name of an ABL collation table or ICU collation.
 * @returns A LOGICAL value representing the result of the comparison, or the Unknown value (?) if an error occurs.
 */
FUNCTION COMPARE RETURNS LOGICAL (
 INPUT string1 AS CHARACTER,
 INPUT relational-operator AS CHARACTER,
 INPUT string2 AS CHARACTER,
 INPUT strength AS CHARACTER,
 INPUT collation AS CHARACTER
 ) FORWARD.
