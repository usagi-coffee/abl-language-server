/**
 * Returns the status of a preprocessor name or include file argument name as an INTEGER value.
 * You can use the DEFINED function only within a preprocessor &IF expression.
 *
 * Syntax
 *
 * `DEFINED ( name )`
 *
 * Notes
 * - This function returns a value of 1 if the argument was a name defined with the &GLOBAL-DEFINE directive;
 * a value of 2 if the argument was passed as an include file parameter; and a value of 3 if the argument was a
 * name defined with the &SCOPED-DEFINE directive. If the argument was not defined and was not an include
 * file parameter, then this function returns a value of 0. The value returned refers to the definition that is current
 * at the point of the call.
 *
 * Example
 *
 * ```abl
 * &IF DEFINED(MAX-EXPENSE) &THEN
 * MESSAGE "MAX-EXPENSE is defined".
 * &ENDIF
 * ```
 *
 * @param name Preprocessor name or include file argument name whose status you want to check.
 * @returns INTEGER value indicating how the name was defined (1 = &GLOBAL-DEFINE, 2 = include file parameter, 3 = &SCOPED-DEFINE, 0 = not defined)
 */
FUNCTION DEFINED RETURNS INTEGER (
 INPUT name AS CHARACTER
 ) FORWARD.
