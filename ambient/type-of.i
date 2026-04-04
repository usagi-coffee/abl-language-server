/**
 * Verifies that the class instance to which the specified object reference points is an instance of the specified
 * object type, inherits from the specified super class, or implements the specified interface.
 *
 * Syntax
 *
 * `TYPE-OF ( object-reference , object-type-name )`
 *
 * @param object-reference An object reference to a class instance.
 * @param object-type-name Specifies the type name of a class, a super class, or an interface that might be defined, inherited
 *                         from, or implemented by the object referenced by object-reference.
 * @returns Returns TRUE if the object reference is valid and points to an instance of the specified type, FALSE otherwise.
 */
FUNCTION TYPE-OF RETURNS LOGICAL (
    INPUT object-reference AS Progress.Lang.Object,
    INPUT object-type-name AS CHARACTER
  ) FORWARD.
