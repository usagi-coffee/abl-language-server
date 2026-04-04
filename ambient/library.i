/**
 * Parses a character string in the form path-name<<member-name>>, where path-name is the pathname of
 * an ABL r-code library and member-name is the name of a file within the library, and returns the pathname of
 * the library.
 *
 * Syntax
 *
 * `LIBRARY ( string )`
 *
 * Notes
 * - You can improve the performance of an application by using the SEARCH and LIBRARY functions for any files
 *   you want to execute that you specify with a relative pathname.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE what-lib AS CHARACTER NO-UNDO.
 * DEFINE VARIABLE location AS CHARACTER NO-UNDO.
 * DEFINE VARIABLE myfile AS CHARACTER NO-UNDO FORMAT "x(16)" LABEL "R-code File".
 * SET myfile.
 * location = SEARCH(myfile).
 * IF location = ? THEN DO:
 *   MESSAGE "Can't find" myfile.
 *   LEAVE.
 * END.
 * what-lib = LIBRARY(location).
 * IF what-lib <> ? THEN
 *   MESSAGE myfile "can be found in library" what-lib.
 * ELSE
 *   MESSAGE myfile "is not in a library but is in" location.
 * ```
 *
 * @param string The pathname of a file in a library.
 * @returns The pathname of the library, or the Unknown value (?) if the string is not in the form path-name<<member-name>>.
 */
FUNCTION LIBRARY RETURNS CHARACTER (
    INPUT string AS CHARACTER
  ) FORWARD.
