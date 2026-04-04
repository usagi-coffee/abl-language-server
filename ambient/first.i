/**
 * Returns a TRUE value if the current iteration of a DO, FOR EACH, or REPEAT . . . BREAK block is the first
 * iteration of that block.
 *
 * Syntax
 *
 * `FIRST ( break-group )`
 *
 * @param break-group The name of a field or expression you name in the block header with the BREAK BY option.
 * @returns TRUE if the current iteration is the first iteration of that block.
 */
FUNCTION FIRST RETURNS LOGICAL (
 break-group AS CHARACTER
 ) FORWARD.
