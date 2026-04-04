/**
 * Returns the object reference for the Progress.Lang.Class instance associated with the specified class or
 * interface type.
 *
 * Syntax
 *
 * `GET-CLASS ( object-type-name )`
 *
 * Notes
 * - Unlike the GetClass() method of Progress.Lang.Class, GET-CLASS takes a type name as its argument instead
 * of a CHARACTER expression that holds a type name. As a result, USING statements can be applied to the
 * argument, and the compiler checks the type name at compile time.
 *
 * Example
 *
 * ```abl
 * USING Progress.Lang.*.
 * USING acme.myObjs.*.
 * DEFINE VARIABLE lFinal AS LOGICAL.
 * lFinal = GET-CLASS(CustObj):IsFinal().
 * MESSAGE "Is the class FINAL? " lFinal VIEW-AS ALERT-BOX.
 * ```
 *
 * @param object-type-name Specifies the type name of an ABL or .NET class or interface type, using the syntax
 * described in the Type-name syntax reference entry. With an appropriate USING statement,
 * you can also specify an unqualified class or interface name alone.
 * @returns The object reference for the Progress.Lang.Class instance associated with the specified class or
 * interface type.
 */
FUNCTION GET-CLASS RETURNS Progress.Lang.Class (
 object-type-name AS CHARACTER
 ) FORWARD.
