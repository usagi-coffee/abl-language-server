/**
 * Compares two or more values and returns the smallest.
 *
 * Syntax
 *
 * `MINIMUM ( expression , expression [, expression ]... )`
 *
 * Notes
 * - When comparing character values, if at least one of the character fields is defined as case sensitive, then MINIMUM treats all of the values as case sensitive for the sake of the comparisons. If none of the values is case sensitive, MINIMUM treats lowercase letters as if they were uppercase letters.
 * - You cannot compare data of different DATE, DATETIME, and DATETIME-TZ data types to each other using MINIMUM. You must first convert different date and datetime data types to the same data type before doing a comparison between them.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE want NO-UNDO LIKE on-hand LABEL "How many do you want?".
 * DEFINE VARIABLE ans AS LOGICAL NO-UNDO.
 * REPEAT:
 *   PROMPT-FOR Item.ItemNum want.
 *   FIND Item NO-LOCK USING Item.ItemNum.
 *   ans = FALSE.
 *   IF MINIMUM(INPUT want, Item.OnHand) = INPUT want THEN DO:
 *     MESSAGE "We have enough" Item.ItemName "in stock.".
 *     MESSAGE "Any other items to check?" UPDATE ans.
 *     IF NOT ans THEN LEAVE.
 *   END.
 *   ELSE DO:
 *     MESSAGE "We only have" Item.OnHand Item.ItemName "in stock.".
 *     MESSAGE "Any other items to check?"
 *     UPDATE ans.
 *     IF NOT ans THEN LEAVE.
 *   END.
 * END.
 * ```
 *
 * @param expression1 A constant, field name, variable name, or expression.
 * @param expression2 A constant, field name, variable name, or expression.
 * @param expression3 A constant, field name, variable name, or expression.
 * @returns The smallest of the specified values. If there is a mixture of decimal and integer data types, decimal type is returned.
 */
FUNCTION MINIMUM RETURNS INTEGER (
    INPUT expression1 AS INTEGER,
    INPUT expression2 AS INTEGER,
    INPUT expression3 AS INTEGER
  ) FORWARD.

FUNCTION MINIMUM RETURNS INTEGER (
    INPUT expression1 AS INTEGER,
    INPUT expression2 AS INTEGER
  ) FORWARD.
