/**
 * Searches the directories and libraries defined in the PROPATH environment variable for a file.
 *
 * Syntax
 *
 * `SEARCH ( opsys-file )`
 *
 * Notes
 * - The SEARCH function is double-byte enabled. You can specify a filename with the opsys-file argument that contains double-byte characters.
 * - Use the SEARCH function to ensure that procedures that get input from external data files are independent of specific directory paths.
 * - Typically, the PROPATH includes a nil entry representing the current working directory. If the SEARCH function finds the file when searching this entry, it returns only the simple name of the file rather than the full pathname.
 * - If you provide a fully qualified pathname, SEARCH checks if the file exists. In this case, SEARCH does not search directories on the PROPATH.
 * - When you search for a file that is in a library, SEARCH returns the file's pathname in the form path-name<<member-name>>.
 * - If an application repeatedly runs a procedure, you can improve performance by using the SEARCH function once to build a full pathname for that procedure.
 * - In Windows, you can specify URL pathnames on the PROPATH.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE fullname AS CHARACTER NO-UNDO FORMAT "x(55)".
 * DEFINE VARIABLE filename AS CHARACTER NO-UNDO FORMAT "x(20)".
 * REPEAT:
 *   UPDATE filename HELP "Try entering 'help.r' or 'dict.r'"
 *     WITH FRAME a SIDE-LABELS CENTERED.
 *   fullname = SEARCH(filename).
 *   IF fullname = ? THEN
 *     DISPLAY "UNABLE TO FIND FILE " filename
 *       WITH FRAME b ROW 6 CENTERED NO-LABELS.
 *   ELSE
 *     DISPLAY "Fully Qualified Path Name Of:" filename SKIP(2)
 *       "is:" fullname WITH FRAME c ROW 6 NO-LABELS CENTERED.
 * END.
 * ```
 *
 * @param opsys-file A character expression whose value is the name of the file you want to find.
 * @returns Returns the full pathname of the file unless it is found in your current working directory. If SEARCH does not find the file, it returns the Unknown value (?).
 */
FUNCTION SEARCH RETURNS CHARACTER (
    INPUT opsys-file AS CHARACTER
  ) FORWARD.
