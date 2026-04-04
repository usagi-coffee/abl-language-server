/**
 * Returns the current INT64 value of a sequence defined in the specified database.
 *
 * Syntax
 *
 * `DYNAMIC-CURRENT-VALUE( sequence-expression , logical-dbname-expression [ , tenant-id ] )`
 *
 * Notes
 * - If logical-dbname-expression contains the Unknown value (?), the AVM generates a run-time error.
 * - The user must have CAN-READ privileges on the _Sequence table to use the DYNAMIC-CURRENT-VALUE function.
 * - The current value of a sequence can be one of the following:
 * - The initial value specified in the Data Dictionary.
 * - The last value set with either the DYNAMIC-CURRENT-VALUE statement or the DYNAMIC-NEXT-VALUE function.
 * - The Unknown value (?) if the sequence has exceeded its minimum or maximum and is not cycling.
 * - If sequence-expression is a multi-tenant sequence in the database, each regular tenant has their own current value of the sequence. So, the same values are returned for each tenant that invokes this function. If the sequence is shared in a multi-tenant database, the values returned by this function are unique across all tenants in the database.
 * - Sequence values are stored in the database in which they are defined, and persist between each invocation of the DYNAMIC-CURRENT-VALUE statement or DYNAMIC-NEXT-VALUE function.
 * - You cannot invoke the DYNAMIC-CURRENT-VALUE function from within a WHERE clause. Doing so generates a compiler error. To use a result from the DYNAMIC-CURRENT-VALUE function in a WHERE clause, assign the result to a variable, then use the variable in the WHERE clause.
 * - You can use any combination of the DYNAMIC-NEXT-VALUE function, DYNAMIC-CURRENT-VALUE function, DYNAMIC-CURRENT-VALUE statement, and their non-dynamic versions.
 *
 * @param sequence-expression A character expression that evaluates to the name of a sequence.
 * @param logical-dbname-expression A character expression that evaluates to the name of a connected database in which the sequence is defined.
 * @param tenant-id An integer expression that evaluates to the tenant ID of a regular tenant, including the default tenant (0). This option applies only to a multi-tenant sequence specified by sequence-expression and is intended for access primarily by a super-tenant user.
 * @returns The current INT64 value of a sequence.
 */
FUNCTION DYNAMIC-CURRENT-VALUE RETURNS INT64 (
 sequence-expression AS CHARACTER,
 logical-dbname-expression AS CHARACTER
 ) FORWARD.

FUNCTION DYNAMIC-CURRENT-VALUE RETURNS INT64 (
 sequence-expression AS CHARACTER,
 logical-dbname-expression AS CHARACTER,
 tenant-id AS INTEGER
 ) FORWARD.
