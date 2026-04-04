/**
 * Returns a new object reference to the same class instance as an existing object reference, but with a
 * different data type. This different data type is cast from the object type of the original object reference
 * according to another specified object type. The two object types must be related, where one is a class type
 * and the other is a subclass of that class type or where one is an interface type and the other is a class
 * that implements the interface of that type.
 * When you cast an object reference, ABL treats it as if it referenced an instance of the object type to which
 * it is cast. The underlying class hierarchy of the object instance does not change.
 *
 * Syntax
 *
 * `CAST( object-reference, object-type-name ).`
 *
 * Notes
 * - You typically cast an object reference down a class hierarchy—that is, from a super class to a derived class
 * within a class hierarchy, or from an interface to a class that implements that interface.
 * - At compile time, ABL verifies that the specified object type is within the class hierarchy of the specified
 * object reference. At run time, the AVM checks the validity of the cast operation.
 * - A .NET generic type can be part of a cast. However, you cannot cast between generic types where the type
 * parameters have no inheritance relationship.
 * - You can also use the DYNAMIC-CAST function to cast object references to object types determined at run time.
 * - You can use the CAST function to cast a parameter in a parameter list for a method.
 * - You can use the CAST function to cast a temp-table field, which is defined as a Progress.Lang.Object,
 * to use as an object of another class type.
 * - You can use the CAST function to cast an object reference to a subclass and invoke a method defined in
 * that subclass using the following syntax: CAST( object-reference, object-type-name ):method-name( parameters ).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE rCustObj AS CLASS acme.myObjs.CustObj.
 * DEFINE TEMP-TABLE mytt FIELD CustObj AS Progress.Lang.Object.
 * rCustObj = CAST(mytt.CustObj, acme.myObjs.CustObj).
 * ```
 *
 * @param object-reference An object reference defined with the object type to be cast.
 * @param object-type-name Specifies the type name of an ABL or .NET class or interface type to which the object reference is cast.
 * @returns A new object reference to the same class instance as the existing object reference, but with a different data type.
 */
FUNCTION CAST RETURNS Progress.Lang.Object (
 object-reference AS Progress.Lang.Object,
 object-type-name AS CHARACTER
 ) FORWARD.
