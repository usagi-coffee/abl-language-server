/**
 * Returns the current value of the ABL PROMSGS variable.
 *
 * Syntax
 *
 * `PROMSGS`
 *
 * Notes
 * - The PROMSGS function returns the name of the current message file.
 * - If the default message file (promsgs) is in use, it returns "promsgs".
 *
 * Example
 *
 * ```abl
 * IF PROMSGS = "promsgs" THEN
 * MESSAGE "Using default promsgs file.".
 * ELSE
 * MESSAGE "Using" PROMSGS.
 * ```
 *
 * @returns CHARACTER - The current value of the ABL PROMSGS variable.
 */
FUNCTION PROMSGS RETURNS CHARACTER (
 ) FORWARD.
