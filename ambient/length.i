/**
 * Returns the number of characters, bytes, or columns in a string, RAW expression, or BLOB field.
 *
 * Syntax
 *
 * `LENGTH ( { expression | raw-expression | blob-field } [ , type ] )`
 *
 * Notes
 * - If expression evaluates to INTEGER or LOGICAL, STRING function converts it first. For LOGICAL, YES/TRUE returns 3 and NO/FALSE returns 2.
 * - Valid types are "CHARACTER" (default), "RAW", and "COLUMN". "COLUMN" is not valid for LONGCHAR or CLOB.
 * - If the value is Unknown (?), the function returns Unknown (?).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE short-name AS CHARACTER NO-UNDO FORMAT "x(11)" LABEL "Desc".
 * FOR EACH Item NO-LOCK:
 *   IF LENGTH(Item.ItemName, "COLUMN") > 8 THEN
 *     short-name = SUBSTRING(Item.ItemName,1,8, "COLUMN") + "..." .
 *   ELSE
 *     short-name = Item.ItemName.
 * END.
 * ```
 *
 * @param expression An expression that evaluates to CHARACTER, LONGCHAR, INTEGER, LOGICAL, or CLOB type.
 * @param type A character expression indicating the unit: "CHARACTER", "RAW", or "COLUMN".
 * @returns Returns the number of characters, bytes, or columns as an INTEGER value.
 */
FUNCTION LENGTH RETURNS INTEGER (
    INPUT expression AS CHARACTER,
    INPUT type AS CHARACTER
  ) FORWARD.

FUNCTION LENGTH RETURNS INTEGER (
    INPUT expression AS CHARACTER
  ) FORWARD.

FUNCTION LENGTH RETURNS INTEGER (
    INPUT raw-expression AS RAW,
    INPUT type AS CHARACTER
  ) FORWARD.

FUNCTION LENGTH RETURNS INTEGER (
    INPUT raw-expression AS RAW
  ) FORWARD.
