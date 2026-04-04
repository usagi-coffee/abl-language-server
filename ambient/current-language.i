/**
 * Returns the current value of the CURRENT-LANGUAGE variable.
 *
 * Syntax
 *
 * `CURRENT-LANGUAGE`
 *
 * Notes
 * - An r-code file may contain several text segments each associated with a different language. The setting of the CURRENT-LANGUAGE variable determines from which r-code text segment the AVM reads character-string constants.
 * - If the value of CURRENT-LANGUAGE is a quoted question mark ("?"), the AVM reads character-strings from the default text segment.
 * - The value of CURRENT-LANGUAGE might be a comma-separated list of language names. If so, the AVM searches r-code for a text segment that matches the first language in the list. If that segment is not found, then it searches for a text segment for the next entry in the list until a segment is found.
 * - You can initialize the CURRENT-LANGUAGE variable with the Language (-lng) parameter.
 * - If a procedure changes the value of CURRENT-LANGUAGE, calls from the procedure to the CURRENT-LANGUAGE function return the name of the new language, but the procedure continues to use the character strings of the original language.
 * - If the procedure then runs another procedure, when the called procedure gets control, calls from the called procedure to the CURRENT-LANGUAGE function return the name of the new language, and the called procedure uses the character strings of the new language.
 * - When the called procedure finishes and control returns to the original procedure, calls from the original procedure to the CURRENT-LANGUAGE function return the name of the new language, but the original procedure continues to use the character strings of the original language.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE cur-lang AS CHARACTER NO-UNDO.
 * cur-lang = CURRENT-LANGUAGE.
 * IF cur-lang = "?" THEN
 * MESSAGE "Your current language is not set.".
 * ELSE
 * MESSAGE "Your current language is" cur-lang.
 * ```
 *
 * @returns CHARACTER - The current value of the CURRENT-LANGUAGE variable.
 */
FUNCTION CURRENT-LANGUAGE RETURNS CHARACTER (
 ) FORWARD.
