/**
 * Returns a comma-delimited list of the code pages listed in convmap.cp or
 * specified by the Conversion Map (-convmap) startup parameter for the current ABL session.
 *
 * Syntax
 *
 * `GET-CODEPAGES`
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE code-page-list AS CHARACTER NO-UNDO.
 * code-page-list = GET-CODEPAGES.
 * ```
 *
 * @returns CHARACTER - A comma-delimited list of code pages available in memory for the current ABL session.
 */
FUNCTION GET-CODEPAGES RETURNS CHARACTER (
 ) FORWARD.
