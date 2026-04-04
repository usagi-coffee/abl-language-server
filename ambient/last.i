/**
 * Returns a TRUE value if the current iteration of a DO, FOR EACH, or REPEAT . . . BREAK block is the last iteration of that block.
 *
 * Syntax
 *
 * `LAST ( break-group )`
 *
 * Example
 *
 * ```abl
 * FOR EACH Item NO-LOCK BY Item.OnHand * Item.Price DESCENDING:
 *   DISPLAY Item.ItemNum Item.OnHand * Item.Price (TOTAL) LABEL "Value-oh"
 *   WITH USE-TEXT.
 * END.
 * FOR EACH Item NO-LOCK BREAK BY Item.OnHand * Item.Price DESCENDING:
 *   FORM Item.ItemNum value-oh AS DECIMAL LABEL "Value-oh"
 *   WITH COLUMN 40 USE-TEXT.
 *   DISPLAY Item.ItemNum Item.OnHand * Item.Price @ value-oh.
 *   ACCUMULATE Item.OnHand * Item.Price (TOTAL).
 *   IF LAST(Item.OnHand * Item.Price) THEN DO:
 *     UNDERLINE value-oh.
 *     DISPLAY ACCUM TOTAL Item.OnHand * Item.Price @ value-oh.
 *   END.
 * END.
 * ```
 *
 * @param break-group The name of a field or expression you named in the block header with the BREAK BY option.
 * @returns Returns a TRUE value if the current iteration is the last iteration of the block.
 */
FUNCTION LAST RETURNS LOGICAL (
    INPUT break-group AS CHARACTER
  ) FORWARD.
