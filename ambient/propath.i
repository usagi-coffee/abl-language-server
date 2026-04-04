/**
 * Returns the current value of the PROPATH environment variable.
 *
 * Syntax
 *
 * `PROPATH`
 *
 * Notes
 * - ABL stores the PROPATH as a comma-separated list of directories. (ABL strips the operating-specific separation characters (a colon (:) on UNIX; a semicolon (;) in Windows) and replaces them with commas.)
 * - The default format for PROPATH is x(70).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE ix AS INTEGER NO-UNDO.
 * DISPLAY PROPATH.
 * REPEAT ix = 1 TO NUM-ENTRIES(PROPATH):
 *   DISPLAY ENTRY(ix, PROPATH) FORMAT "x(30)".
 * END.
 * ```
 *
 * @returns Returns the current value of the PROPATH environment variable as a comma-separated list of directories.
 */
FUNCTION PROPATH RETURNS CHARACTER (
  ) FORWARD.
