/**
 * Returns one of the following character values which identifies the ABL product that is running: Full, Query or
 * Run-time. Can also return COMPILE if you use the Developer's Toolkit, or COMPILE-ENCRYPT if you use the
 * run-time Compiler.
 *
 * Syntax
 *
 * `PROGRESS`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 *
 * Example
 *
 * ```abl
 * IF PROGRESS EQ "FULL" THEN
 * MESSAGE "Running Full Editor".
 * ELSE IF PROGRESS EQ "QUERY" THEN
 * MESSAGE "Running Query Editor".
 * ELSE IF PROGRESS EQ "RUN-TIME" THEN
 * MESSAGE "Running Runtime".
 * ```
 *
 * @returns One of the following character values: "Full", "Query", "Run-time", "COMPILE", or "COMPILE-ENCRYPT".
 */
FUNCTION PROGRESS RETURNS CHARACTER (
 ) FORWARD.
