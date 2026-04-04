/**
 * Returns a TRUE value if the current iteration of a DO, FOR EACH, or REPEAT . . . BREAK block
 * is the first iteration for a new break group, and modifies all three block types.
 *
 * Syntax
 *
 * `FIRST-OF ( break-group )`
 *
 * break-group
 * The name of a field or expression you name in the block header with the BREAK BY option.
 *
 * Notes
 * - When you calculate in a block use the BREAK option to tell the AVM to calculate when
 * the value of certain expressions changes. The AVM uses default formatting to display
 * the results of these calculations. To control the formatting, use the FIRST-OF function
 * to determine the start of a break group and then change the formatting.
 *
 * Example
 *
 * ```abl
 * FOR EACH Item BREAK BY Item.CatPage:
 * IF FIRST-OF(Item.CatPage) THEN CLEAR ALL.
 * DISPLAY Item.CatPage Item.ItemNum Item.ItemName.
 * END.
 * ```
 *
 * @param break-group The name of a field or expression you name in the block header with the BREAK BY option.
 * @returns LOGICAL TRUE if the current iteration is the first for a new break group.
 */
FUNCTION FIRST-OF RETURNS LOGICAL (
 break-group AS HANDLE
 ) FORWARD.
