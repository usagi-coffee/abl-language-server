/**
 * Returns the current system date.
 *
 * Syntax
 *
 * `TODAY`
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE rptdate AS DATE NO-UNDO.
 * rptdate = TODAY.
 * ```
 *
 * @returns Returns the current system date.
 */
FUNCTION TODAY RETURNS DATE (
  ) FORWARD.
