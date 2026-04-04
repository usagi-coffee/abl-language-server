/**
 * Returns an instance of the enumeration type for the specified enumeration member.
 *
 * Syntax
 *
 * `DYNAMIC-ENUM( enum-type-name, enum-member )`
 *
 * Notes
 * - The function generates a runtime error if any specified character or numeric value does not correspond to a defined member.
 *
 * Example
 *
 * ```abl
 * myDirection = DYNAMIC-ENUM("Direction", cNeededDirection).
 * ```
 *
 * @param enum-type-name The name of an ABL or .NET enumeration type that defines the member. This is a CHARACTER expression that the AVM evaluates to the type name of an enumeration type at runtime.
 * @param enum-member Specifies an enum member in one of two ways: A CHARACTER expression that evaluates to the member name at runtime, or an INTEGER or INT64 expression that evaluates to the underlying numeric value of the member at runtime.
 * @returns An instance of the enumeration type for the specified enumeration member.
 */
FUNCTION DYNAMIC-ENUM RETURNS Progress.Lang.Enum (
 enum-type-name AS CHARACTER,
 enum-member AS CHARACTER
 ) FORWARD.

FUNCTION DYNAMIC-ENUM RETURNS Progress.Lang.Enum (
 enum-type-name AS CHARACTER,
 enum-member AS INT64
 ) FORWARD.
