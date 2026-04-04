/**
 * Returns the value of a class property. The function allows you to refer to a class property by providing a character
 * expression for the property name, along with an object reference, THIS-OBJECT system reference, or the
 * name of a class. The compiler supports both ABL user-defined types and .NET types for the object reference
 * or class name.
 *
 * Syntax
 *
 * `[ return-value = ] DYNAMIC-PROPERTY(
 * { object-reference | class-type-name } , property-name [ , index ] )`
 *
 * Notes
 * - The DYNAMIC-PROPERTY function also works with .NET indexed properties. Normally, ABL allows you to refer to
 * a .NET indexed property without using the property name, but for dynamic access you must use the property name,
 * typically Item indexed property.
 *
 * @param object-reference Reference to an ABL or .NET class instance that exposes the specified property as an instance member.
 * @param class-type-name Name of an ABL or .NET class type that defines the specified property as a static member. This is a CHARACTER expression that the AVM evaluates to the type name of a class at run time.
 * @param property-name CHARACTER expression that evaluates to the property name at run time.
 * @param index Integer expression for the index of the specified element. Use index to set or retrieve an individual array element.
 * @returns Data element that is assigned the value returned when you execute the property's GET accessor.
 */
FUNCTION DYNAMIC-PROPERTY RETURNS CHARACTER (
 object-reference AS Progress.Lang.Object,
 property-name AS CHARACTER
 ) FORWARD.

FUNCTION DYNAMIC-PROPERTY RETURNS CHARACTER (
 class-type-name AS CHARACTER,
 property-name AS CHARACTER
 ) FORWARD.

FUNCTION DYNAMIC-PROPERTY RETURNS CHARACTER (
 object-reference AS Progress.Lang.Object,
 property-name AS CHARACTER,
 index AS INTEGER
 ) FORWARD.

FUNCTION DYNAMIC-PROPERTY RETURNS CHARACTER (
 class-type-name AS CHARACTER,
 property-name AS CHARACTER,
 index AS INTEGER
 ) FORWARD.
