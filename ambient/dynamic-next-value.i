/**
 * Returns the next INT64 value of a sequence, incremented by the positive or negative value defined in the
 * specified database.
 *
 * Syntax
 *
 * `DYNAMIC-NEXT-VALUE( sequence-expression, logical-dbname-expression [, tenant-id ] )`
 *
 * Notes
 * - If logical-dbname-expression contains the Unknown value (?), the AVM raises a run-time error.
 * - If sequence-expression is a cycling sequence, and the DYNAMIC-NEXT-VALUE function increments
 * the sequence beyond its upper limit (for positive increments) or decrements the sequence beyond its lower
 * limit (for negative increments), the function sets and returns the initial value defined for the sequence.
 * - If sequence-expression is a terminating sequence, and the DYNAMIC-NEXT-VALUE function attempts
 * to increment the sequence beyond its upper limit (for positive increments) or decrement the sequence
 * beyond its lower limit (for negative increments), the function returns the Unknown value (?) and leaves the
 * current sequence value unchanged. Once a sequence terminates, DYNAMIC-NEXT-VALUE continues to
 * return the Unknown value (?) for the specified sequence until it is reset to a new value with the
 * DYNAMIC-CURRENT-VALUE statement, or its definition is changed to a cycling sequence. After changing
 * the sequence definition to cycle, the first use of DYNAMIC-NEXT-VALUE for the sequence sets and returns
 * its initial value.
 * - If sequence-expression is a multi-tenant sequence in the database, each regular tenant has their own
 * current value of the sequence. So, the same values are returned for each tenant that invokes this function.
 * If the sequence is shared in a multi-tenant database, the values returned by this function are unique across
 * all tenants in the database.
 * - The value of a sequence set by the DYNAMIC-NEXT-VALUE function persists in the database until the
 * next DYNAMIC-CURRENT-VALUE statement or DYNAMIC-NEXT-VALUE function is invoked for the
 * sequence, or until the sequence is deleted from the database.
 * - You cannot invoke the DYNAMIC-NEXT-VALUE function from within a WHERE clause. Doing so generates
 * a compiler error because the value returned by the DYNAMIC-NEXT-VALUE function can result in ambiguous
 * expressions. To use a result from the DYNAMIC-NEXT-VALUE function in a WHERE clause, assign the
 * result to a variable and use the variable in the WHERE clause instead.
 * - You can use any combination of the DYNAMIC-NEXT-VALUE function, DYNAMIC-CURRENT-VALUE
 * function, DYNAMIC-CURRENT-VALUE statement, and their non-dynamic versions.
 *
 * @param sequence-expression A character expression that evaluates to the name of a sequence.
 * @param logical-dbname-expression A character expression that evaluates to the name of a connected database in which the sequence is defined.
 * @param tenant-id An integer expression that evaluates to the tenant ID of a regular tenant, including the default tenant (0).
 * @returns The next INT64 value of the sequence.
 */
FUNCTION DYNAMIC-NEXT-VALUE RETURNS INT64 (
 sequence-expression AS CHARACTER,
 logical-dbname-expression AS CHARACTER
 ) FORWARD.

FUNCTION DYNAMIC-NEXT-VALUE RETURNS INT64 (
 sequence-expression AS CHARACTER,
 logical-dbname-expression AS CHARACTER,
 tenant-id AS INTEGER
 ) FORWARD.
