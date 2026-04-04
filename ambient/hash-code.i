/**
 * Returns a hash code for one or more arguments.
 *
 * Syntax
 *
 * `HASH-CODE ( arg [, arg ]… )`
 *
 * Notes
 * - The function accepts up to 10 arguments.
 * - This function is intended for use with hash-based collections only.
 * - The hash code returned is consistent within a given ABL session.
 * - Do not store or pass the hash code between sessions.
 * - The order of the arguments can change the resulting hash code value.
 * - BLOB/CLOB types are not supported.
 *
 * Example
 *
 * ```abl
 * VAR CHAR FirstName.
 * VAR CHAR LastName.
 * VAR INT hashval.
 * hashval = HASH-CODE(FirstName, LastName).
 * ```
 *
 * @param arg1 The first argument (any data type allowed when defining a variable).
 * @param arg2 The second argument (any data type allowed when defining a variable).
 * @returns A hash code as an INTEGER value.
 */
FUNCTION HASH-CODE RETURNS INTEGER (
    INPUT arg1 AS CHARACTER,
    INPUT arg2 AS CHARACTER
  ) FORWARD.

FUNCTION HASH-CODE RETURNS INTEGER (
    INPUT arg1 AS CHARACTER
  ) FORWARD.
