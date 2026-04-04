/**
 * Verifies that an object reference points to a valid ABL or .NET class-based object instance or an ABL handle-based object.
 *
 * Syntax
 *
 * `VALID-OBJECT ( handle )`
 * `VALID-OBJECT ( object-reference )`
 *
 * Notes
 * - If the object reference represents an object that is currently valid, the function returns TRUE.
 * - If the object reference is no longer valid (for example, it was garbage collected or explicitly deleted), the function returns FALSE.
 * - When used with a handle-based object, VALID-OBJECT works in the same way as the VALID-HANDLE function.
 *
 * @param handle A handle to an ABL handle-based object. Must be a variable of type HANDLE containing a valid handle.
 * @param object-reference An object reference defined for an ABL or .NET object type.
 * @returns Returns TRUE if the object reference is valid, FALSE otherwise.
 */
FUNCTION VALID-OBJECT RETURNS LOGICAL (
    INPUT handle AS HANDLE
  ) FORWARD.

FUNCTION VALID-OBJECT RETURNS LOGICAL (
    INPUT object-reference AS Progress.Lang.Object
  ) FORWARD.
