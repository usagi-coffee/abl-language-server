/**
 * Returns the version of ABL, or release of OpenEdge, you are running.
 *
 * Syntax
 *
 * `PROVERSION [ ( mode ) ]`
 *
 * Notes
 * - If mode is 0 or not specified, returns limited version information (major, minor, and version tag if applicable).
 * - If mode is 1, returns full version information (major, minor, service pack, hot fix, build number, and version tag if applicable).
 * - If mode is other than 0 or 1, a run-time error message is returned.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ver AS CHARACTER NO-UNDO.
 * ver = PROVERSION(1).
 * ```
 *
 * @param mode An optional integer expression that identifies how much version information is returned.
 * @returns A character value containing the version information.
 */
FUNCTION PROVERSION RETURNS CHARACTER (
    INPUT mode AS INTEGER
  ) FORWARD.

FUNCTION PROVERSION RETURNS CHARACTER (
  ) FORWARD.
