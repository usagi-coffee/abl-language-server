/**
 * Returns the current INT64 value of a sequence defined in the Data Dictionary.
 *
 * Syntax
 *
 * `CURRENT-VALUE ( sequence [, logical-dbname ] [, tenant-id ] )`
 *
 * Notes
 * - The current value of a sequence can be one of the following:
 * - The initial value specified in the Data Dictionary
 * - The last value set with either the CURRENT-VALUE statement or the NEXT-VALUE function
 * - The Unknown value (?) if the sequence has exceeded its minimum or maximum and is not cycling
 * - If sequence is a multi-tenant sequence in the database, each regular tenant has their own current value
 * of the sequence. So, the same values are returned for each tenant that invokes this function. If the sequence
 * is shared in a multi-tenant database, the values returned by this function are unique across all tenants in
 * the database.
 * - Sequence values are stored in the database in which they are defined, and persist between each invocation
 * of the CURRENT-VALUE statement or NEXT-VALUE function.
 * - You cannot invoke the CURRENT-VALUE function from within a WHERE clause. Doing so generates a
 * compiler error. To use a result from the CURRENT-VALUE function in a WHERE clause, assign the result
 * to a variable, then use the variable in the WHERE clause.
 * - You can use any combination of the NEXT-VALUE function, CURRENT-VALUE function, CURRENT-VALUE
 * statement, and their dynamic versions. Use the dynamic version when you don't know what the database
 * name or sequence name is at compile time.
 * - Be careful when accessing a database sequence with an alias that points to a different database than the
 * one used when the alias was defined. If you supply an alias name to the CURRENT-VALUE function or the
 * NEXT-VALUE function, only the database used to define the alias is referenced. In this case, it is preferable
 * to use the DYNAMIC-CURRENT-VALUE function and DYNAMIC-NEXT-VALUE function instead of the
 * CURRENT-VALUE function and NEXT-VALUE function, respectively.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE cur-cust NO-UNDO LIKE Customer.CustNum.
 * cur-cust = CURRENT-VALUE(NextCustNum).
 * IF CAN-FIND(FIRST Order WHERE Order.CustNum = cur-cust) THEN
 * FOR EACH Order NO-LOCK WHERE Order.CustNum = cur-cust:
 * DISPLAY Order.
 * END.
 * ```
 *
 * @param sequence An identifier that specifies the name of a sequence defined in the Data Dictionary.
 * @param logical-dbname An identifier that specifies the logical name of the database in which the sequence is defined. The database must be connected. You can omit this parameter if the sequence name is unambiguous. If a sequence with this name exists in more than one connected database, then you must specify logical-dbname.
 * @param tenant-id An integer expression that evaluates to the tenant ID of a regular tenant, including the default tenant (0). This option applies only to a multi-tenant sequence specified by sequence and is intended for access primarily by a super-tenant user.
 * @returns The current INT64 value of the sequence.
 */
FUNCTION CURRENT-VALUE RETURNS INT64 (
 sequence AS CHARACTER
 ) FORWARD.

FUNCTION CURRENT-VALUE RETURNS INT64 (
 sequence AS CHARACTER,
 logical-dbname AS CHARACTER
 ) FORWARD.

FUNCTION CURRENT-VALUE RETURNS INT64 (
 sequence AS CHARACTER,
 logical-dbname AS CHARACTER,
 tenant-id AS INT64
 ) FORWARD.
