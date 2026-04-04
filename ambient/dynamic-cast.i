/**
 * Returns a new object reference to the same class instance as an existing object reference, but with a
 * different data type. This different data type is cast from the object type of the original object reference
 * according to another object type specified by a run-time expression. The object types must be related,
 * where one is a class type and the other is subclass of that class type or where one is an interface type
 * and the other is a class that implements the interface of that type. When you cast an object reference,
 * ABL treats it as if it referenced an instance of the object type to which it is cast. The underlying class
 * hierarchy of the object instance does not change.
 *
 * Syntax
 *
 * `DYNAMIC-CAST( object-reference , expression )`
 *
 * Notes
 * - You typically cast an object reference down a class hierarchy—that is, from a super class to a derived class
 * within a class hierarchy, or from an interface to a class that implements that interface. However, you do not
 * always need to explicitly cast an object reference. Because a derived class contains all the super classes
 * in its inherited class hierarchy, ABL implicitly casts any object reference up within its class hierarchy, and
 * because a class that implements an interface implements all of the methods specified for the interface, ABL
 * implicitly casts any object reference from an implementing class to any interface that the class implements.
 * - At run time, ABL verifies that the object type specified by expression is within the class hierarchy of the
 * specified object reference. Therefore, if you access a class member on the cast object reference that exists
 * for the cast data type, but the referenced object does not actually define the accessed class member, the
 * AVM raises a run-time ERROR.
 * - A .NET generic type can be part of a cast. For example, you can cast from a System.Object to a
 * "System.Collections.Generic.List<SHORT>", because all .NET classes, including generic classes,
 * derive from the .NET root class. However, note that you cannot cast from a
 * "System.Collections.Generic.List<System.Object>" to a
 * "System.Collections.Generic.List<System.Windows.Forms.Button>". You cannot assign a
 * "List<Button>" reference to an object reference defined as a "List<Object>", because, even though
 * the type parameters are compatible, the two objects as a whole are not equivalent and have no inheritance
 * relationship. Therefore, a cast between these two objects cannot work either.
 * - You can also use the CAST function to perform all casting operations at compile time. The primary reason
 * for using the DYNAMIC-CAST function is to cast object references based on run-time conditions that
 * determine the object type to use for the cast.
 * - You can use the DYNAMIC-CAST function to cast a parameter in a parameter list for a method using the
 * following syntax, where expression evaluates to a subclass of the object-reference type:
 * method-name( INPUT DYNAMIC-CAST( object-reference, expression ), ... ).
 * - You can use the DYNAMIC-CAST function to cast a temp-table field, which is defined as a
 * Progress.Lang.Object, to use as an object of another class type.
 * - You cannot use the DYNAMIC-CAST function to cast an object reference to a subclass and invoke a method
 * defined in that subclass using the following syntax:
 * DYNAMIC-CAST( object-reference, object-type-name ):method-name( parameters ).
 * Similarly, you cannot use this syntax to invoke a method on a class that implements the referenced interface
 * from which you cast the specified class.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE RCustObj AS CLASS acme.myObjs.CustObj.
 * DEFINE TEMP-TABLE mytt NO-UNDO
 * FIELD CustObj AS Progress.Lang.Object.
 * RCustObj = DYNAMIC-CAST(mytt.CustObj, "acme.myObjs.CustObj").
 * ```
 *
 * @param object-reference An object reference defined with the object type to be cast.
 * @param expression A character expression that evaluates to the fully qualified type name for the ABL
 * or .NET class or interface type to which the object reference is cast.
 * @returns A new object reference to the same class instance but with a different data type.
 */
FUNCTION DYNAMIC-CAST RETURNS Progress.Lang.Object (
 object-reference AS Progress.Lang.Object,
 expression AS CHARACTER
 ) FORWARD.
