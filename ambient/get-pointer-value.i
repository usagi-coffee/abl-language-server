/**
 * Function: GET-POINTER-VALUE
 * Description:
 * Returns, as an INT64 value, the address of (or pointer to) the memory region associated with the specified
 * MEMPTR variable. The returned value is based on whether the platform supports 64-bit pointers or 32-bit
 * pointers. On a 32-bit platform, the value never gets bigger than 2GB.
 * Note: Does not apply to SpeedScript programming.
 *
 * Syntax:
 *   GET-POINTER-VALUE ( memptr-var )
 *
 * @param memptr-var A reference to a variable defined as MEMPTR. If the variable is uninitialized (has no associated memory region), the function returns 0.
 * @returns INT64 value representing the address of the memory region associated with the specified MEMPTR variable.
 *
 * Example:
 *   This function is particularly useful when building a structure in an MEMPTR region that references other
 *   MEMPTR regions. It allows you to obtain the pointer to one MEMPTR region and store it in the structure you
 *   create in another MEMPTR region.
 *
 *   Note: The following example only works on a 32-bit machine because it leaves space for 4 bytes for the pointer
 *   values. On a 64-bit machine, you would have to allocate 8 bytes of space.
 *
 *   DEFINE VARIABLE bitmapinfo AS MEMPTR NO-UNDO.
 *   DEFINE VARIABLE bitmapinfoheader AS MEMPTR NO-UNDO.
 *   DEFINE VARIABLE RGBcolors AS MEMPTR NO-UNDO.
 *
 *   SET-SIZE(bitmapinfo) = 4 + 4.
 *   SET-SIZE(bitmapinfoheader) = 4 + 4 + 4 + 2 + 2 + 4 + 4 + 4 + 4 + 4 + 4.
 *   SET-SIZE(RGBcolors) = 16 * 4.
 *
 *   PUT-LONG(bitmapinfo,1) = GET-POINTER-VALUE(bitmapinfoheader).
 *   PUT-LONG(bitmapinfo,5) = GET-POINTER-VALUE(RGBcolors).
 *
 * Notes:
 *   - MEMPTR structures are initialized using the SET-SIZE statement.
 *   - For more information on using the MEMPTR data type, see OpenEdge Programming Interfaces.
 */

FUNCTION GET-POINTER-VALUE RETURNS INT64 FORWARD.
