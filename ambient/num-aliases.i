/**
 * Returns an INTEGER value that represents the number of aliases defined.
 *
 * Syntax
 *
 * `NUM-ALIASES`
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * DISPLAY NUM-ALIASES LABEL "Number of Defined Aliases:".
 * REPEAT ix = 1 TO NUM-ALIASES:
 *   DISPLAY ALIAS(ix) LABEL "Aliases"
 *     LDBNAME(ALIAS(ix)) LABEL "Logical Database".
 * END.
 * ```
 *
 * @returns An INTEGER value that represents the number of aliases defined.
 */
FUNCTION NUM-ALIASES RETURNS INTEGER (
  ) FORWARD.
