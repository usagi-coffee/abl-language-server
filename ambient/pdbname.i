/**
 * Returns the physical name of a currently connected database.
 *
 * Syntax
 *
 * `PDBNAME ( integer-expression | logical-name | alias )`
 *
 * Notes
 * - If the parameter is an integer greater than the number of connected databases, returns the Unknown value (?).
 * - If the parameter is a logical name or alias that does not match a connected database, returns the Unknown value (?).
 * - The old DBNAME function has been retained for compatibility and is equivalent to PDBNAME(1).
 *
 * Example
 *
 * ```abl
 * MESSAGE "The current DICTDB is" PDBNAME("DICTDB") + ".db".
 * ```
 *
 * @param integer-expression The 1-based index of a connected database.
 * @param logical-name-or-alias A quoted character string or character expression representing the logical name or alias of a connected database.
 * @returns Returns the physical name of the specified database, or the Unknown value (?) if not found.
 */
FUNCTION PDBNAME RETURNS CHARACTER (
    INPUT integer-expression AS INTEGER
  ) FORWARD.

FUNCTION PDBNAME RETURNS CHARACTER (
    INPUT logical-name-or-alias AS CHARACTER
  ) FORWARD.
