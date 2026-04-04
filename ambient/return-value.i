/**
 * Provides a character string value returned by the most recently executed RETURN statement.
 *
 * Syntax
 *
 * `RETURN-VALUE`
 *
 * Notes
 * - The returned value has the CHARACTER data type.
 * - When you access RETURN-VALUE, its value represents the value returned by the most recently executed RETURN or THROW.
 * - If it is a RETURN with options that can set the RETURN-VALUE, but does not, the empty string ("") is returned.
 * - If it is a THROW that returns a Progress.Lang.AppError object, the value of the object's ReturnValue property is returned.
 * - To reliably access a RETURN-VALUE setting returned from a block or called routine, check RETURN-VALUE as soon as you can after the block or called routine terminates.
 * - RETURN-VALUE does not return the value of a non-VOID method of a class or user-defined function invocation.
 *
 * Example
 *
 * ```abl
 * RUN my-procedure.
 * IF RETURN-VALUE <> "" THEN
 *     MESSAGE "Return value: " RETURN-VALUE.
 * ```
 *
 * @returns A character string value returned by the most recently executed RETURN statement.
 */
FUNCTION RETURN-VALUE RETURNS CHARACTER (
  ) FORWARD.
