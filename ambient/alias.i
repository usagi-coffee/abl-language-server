/**
 * The ALIAS function returns the alias corresponding to the integer value of expression.
 *
 * Syntax
 *
 * `ALIAS ( integer-expression )`
 *
 * Notes
 * - If there are, for example, three currently defined aliases, the functions ALIAS(1), ALIAS(2), and ALIAS(3) return them. If the ALIAS function cannot find a defined alias, it returns the Unknown value (?). For example, building on the previous example of three defined aliases, the functions ALIAS(4), ALIAS(5), and ALIAS(6) return the Unknown value (?) because they cannot find a defined alias.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * REPEAT ix = 1 TO NUM-ALIASES:
 * DISPLAY ALIAS(ix) LABEL "Alias"
 * LDBNAME(ALIAS(ix)) LABEL "Logical Database".
 * END.
 * ```
 *
 * @param integer-expression The integer value corresponding to an alias
 * @returns The alias corresponding to the integer value of expression
 */
FUNCTION ALIAS RETURNS CHARACTER (
 integer-expression AS INTEGER
 ) FORWARD.
