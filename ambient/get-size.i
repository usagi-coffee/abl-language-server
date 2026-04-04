/**
 * Description
 * Returns, as an INT64 value, the allocated byte size of the memory region associated with the specified MEMPTR variable.
 *
 * Syntax
 * GET-SIZE ( memptr-var )
 *
 * Notes
 * - To return a memory size greater than 0, the MEMPTR variable must be fully initialized, not just pre-initialized by a DLL or UNIX shared library routine.
 * - MEMPTR structures are initialized using the SET-SIZE statement.
 * - For more information on using the MEMPTR data type, see OpenEdge Programming Interfaces.
 *
 * Example
 * DEFINE VARIABLE bitmapinfo AS MEMPTR NO-UNDO.
 * DEFINE VARIABLE bitmapinfoheader AS MEMPTR NO-UNDO.
 * DEFINE VARIABLE RGBcolors AS MEMPTR NO-UNDO.
 *
 * SET-SIZE(bitmapinfo) = 4 + 4.
 * SET-SIZE(bitmapinfoheader) = 4 + 4 + 4 + 2 + 2 + 4 + 4 + 4 + 4 + 4 + 4.
 * SET-SIZE(RGBcolors) = 16 * 4.
 *
 * DISPLAY
 * GET-SIZE(bitmapinfo) LABEL "Bitmap info structure size" COLON 30 SKIP
 * GET-SIZE(bitmapinfoheader) LABEL "Bitmap header structure size" COLON 30 SKIP
 * GET-SIZE(RGBcolors) LABEL "Bitmap colors array size" COLON 30
 * WITH SIDE-LABELS.
 *
 * @param memptr-var A MEMPTR variable. If the variable is uninitialized (has no associated memory region), the function returns 0.
 * @returns INT64
 */
FUNCTION GET-SIZE RETURNS INT64 (INPUT memptr-var AS MEMPTR) FORWARD.
