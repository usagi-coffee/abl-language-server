/**
 * Invokes a class-based method whose name is specified by a run-time expression, but whose parameters are defined at compile time.
 *
 * Syntax
 *
 * `[ return-value = ] DYNAMIC-INVOKE( {class-type-name | object-reference}, method-name [ , parameter [ , parameter ] ... ] )`
 *
 * Notes
 * - The Invoke( ) method of the Progress.Lang.Class class provides similar functionality to the DYNAMIC-INVOKE function. The difference is that DYNAMIC-INVOKE has a fixed, compile-time parameter list and, therefore, does not require the creation of a ParameterList object at run time.
 * - DYNAMIC-INVOKE must be executed as a stand-alone statement (without assigning it to a return-value) if the method it invokes is VOID.
 *
 * Example
 *
 * ```abl
 * class MyObject:
 * method public decimal MultiplyTheValue(dValue as decimal):
 * return dValue * 2.
 * end.
 * end.
 * var decimal retValue.
 * var MyObject rObj = new MyObject().
 * retValue = dynamic-invoke(rObj, "MultiplyTheValue", 12.3).
 * ```
 *
 * @param class-type-name The name of an ABL or .NET class type that defines the specified method as a static member.
 * @param object-reference Specifies a reference to an ABL or .NET class instance that defines the specified method as an instance member.
 * @param method-name A CHARACTER expression that evaluates to the method name.
 * @param parameter Specifies zero or more parameters passed to the method.
 * @returns An optional data element that is assigned the return value from the invoked, non-void method.
 */
FUNCTION DYNAMIC-INVOKE RETURNS CHARACTER (
 class-type-name AS CHARACTER,
 method-name AS CHARACTER
 ) FORWARD.

FUNCTION DYNAMIC-INVOKE RETURNS CHARACTER (
 object-reference AS Progress.Lang.Object,
 method-name AS CHARACTER
 ) FORWARD.

FUNCTION DYNAMIC-INVOKE RETURNS CHARACTER (
 class-type-name AS CHARACTER,
 method-name AS CHARACTER,
 parameter AS CHARACTER
 ) FORWARD.

FUNCTION DYNAMIC-INVOKE RETURNS CHARACTER (
 object-reference AS Progress.Lang.Object,
 method-name AS CHARACTER,
 parameter AS CHARACTER
 ) FORWARD.
