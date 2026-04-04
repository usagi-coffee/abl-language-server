/**
 * Parses a reference to a member of an ABL r-code library and returns the simple member name.
 *
 * Syntax
 *
 * `MEMBER ( string )`
 *
 * Notes
 * - The string must be in the form path-name<<member-name>>, where path-name is the pathname of a library and member-name is the name of a file within the library.
 * - If the string is not in this form, the function returns the Unknown value (?).
 * - Use with the SEARCH function to determine whether a file is in a library.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE location AS CHARACTER NO-UNDO.
 * DEFINE VARIABLE what-lib AS CHARACTER NO-UNDO.
 * location = SEARCH(myfile).
 * what-lib = LIBRARY(location).
 * IF what-lib <> ? THEN
 *   MESSAGE MEMBER(location) "can be found in library" what-lib.
 * ```
 *
 * @param string A character expression whose value is the pathname of a file in an r-code library.
 * @returns The simple member name from the library reference, or ? if not in library format.
 */
FUNCTION MEMBER RETURNS CHARACTER (
    INPUT string AS CHARACTER
  ) FORWARD.
