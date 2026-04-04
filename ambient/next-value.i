/**
 * Returns the next INT64 value of a static sequence, incremented by the positive or negative value defined in
 * the Data Dictionary.
 *
 * Syntax
 *
 * `NEXT-VALUE ( sequence [ , logical-dbname ] [ , tenant-id ] )`
 *
 * Notes
 * - If sequence is a cycling sequence, and the NEXT-VALUE function increments the sequence beyond its upper limit (for positive increments) or decrements the sequence beyond its lower limit (for negative increments), the function sets and returns the initial value defined for the sequence.
 * - If sequence is a terminating sequence, and the NEXT-VALUE function attempts to increment the sequence beyond its upper limit (for positive increments) or decrement the sequence beyond its lower limit (for negative increments), the function returns the Unknown value (?) and leaves the current sequence value unchanged.
 * - If sequence is a multi-tenant sequence in the database, each regular tenant has their own current value of the sequence.
 * - You cannot invoke the NEXT-VALUE function from within a WHERE clause. Doing so generates a compiler error.
 *
 * Example
 *
 * ```abl
 * TRIGGER PROCEDURE FOR Create OF Item.
 * /* Automatically assign a unique item number using NextItemNum seq */
 * ASSIGN Item.ItemNum = NEXT-VALUE(NextItemNum).
 * ```
 *
 * @param sequence An identifier that specifies the name of a sequence defined in the Data Dictionary.
 * @param logical-dbname An identifier that specifies the logical name of the database in which the sequence is defined.
 * @param tenant-id An integer expression that evaluates to the tenant ID of a regular tenant, including the default tenant (0).
 * @returns Returns the next INT64 value of a static sequence.
 */
FUNCTION NEXT-VALUE RETURNS INT64 (
    INPUT sequence AS CHARACTER,
    INPUT logical-dbname AS CHARACTER,
    INPUT tenant-id AS INTEGER
  ) FORWARD.

FUNCTION NEXT-VALUE RETURNS INT64 (
    INPUT sequence AS CHARACTER,
    INPUT logical-dbname AS CHARACTER
  ) FORWARD.

FUNCTION NEXT-VALUE RETURNS INT64 (
    INPUT sequence AS CHARACTER
  ) FORWARD.
