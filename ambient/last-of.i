/**
 * Returns a TRUE value if the current iteration of a DO, FOR EACH, or REPEAT . . . BREAK block is the last iteration for a particular value of a break group.
 *
 * Syntax
 *
 * `LAST-OF ( break-group )`
 *
 * Example
 *
 * ```abl
 * FOR EACH Item NO-LOCK BREAK BY Item.CatPage:
 *   ACCUMULATE Item.OnHand * Item.Price (TOTAL BY Item.CatPage).
 *   IF LAST-OF(Item.CatPage) THEN
 *     DISPLAY Item.CatPage (ACCUM TOTAL BY Item.CatPage
 *       Item.OnHand * Item.Price) LABEL "Value-oh".
 * END.
 * ```
 *
 * @param break-group The name of a field or expression you named in the block header with the BREAK BY option.
 * @returns Returns TRUE if the current iteration is the last iteration for a particular value of a break group.
 */
FUNCTION LAST-OF RETURNS LOGICAL (
    INPUT break-group AS CHARACTER
  ) FORWARD.
