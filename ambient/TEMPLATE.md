## TEMPLATE FOR ABL FUNCTION FILES

Create files with this EXACT format:

```abl
/**
 * [Description from reference - keep it concise]
 *
 * Syntax
 *
 * `[FUNCTION-NAME ( param1 [, param2 ] )]`
 *
 * Notes
 * - [Note 1 if any]
 * - [Note 2 if any]
 *
 * Example
 *
 * ```abl
 * [Simple example if available]
 * ```
 *
 * @param param1 [Description]
 * @param param2 [Description]
 * @returns [Description of return value]
 */
FUNCTION FUNCTION-NAME RETURNS RETURN-TYPE (
    INPUT param1 AS TYPE,
    INPUT param2 AS TYPE
  ) FORWARD.

FUNCTION FUNCTION-NAME RETURNS RETURN-TYPE (
    INPUT param1 AS TYPE
  ) FORWARD.
```

## IMPORTANT RULES:

1. **JSDoc block**: Start with `/**` and end with `*/`
2. **Description**: Goes right after `/**` on the same line or next line
3. **Syntax section**: Use backticks around the syntax
4. **Notes section**: Only include if there are notes. Use `- ` for bullet points
5. **Example section**: Only include if there's an example. Use ```abl code blocks
6. **@param tags**: One per parameter
7. **@returns tag**: Describe the return value
8. **Function declarations**: 
   - FULL signature first (with all parameters)
   - Then overloads without comments
   - Use `INPUT` keyword for parameters
   - End with `FORWARD.`
9. **REMOVE**: All "on page X" references, all "See also" sections, all page numbers
10. **Parameter naming**: Use lowercase with hyphens (e.g., `source-string`, `start-position`)

## EXAMPLE - INDEX function:

```abl
/**
 * Returns the first position of target in source.
 *
 * Syntax
 *
 * `INDEX ( source , target [ , starting ] )`
 *
 * Notes
 * - If either operand is case sensitive, then the search is case sensitive.
 * - If the target string is null, the result is 0.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE pos AS INTEGER NO-UNDO.
 * pos = INDEX("banana", "na").
 * ```
 *
 * @param source-string The string to search in.
 * @param target-string The substring to search for.
 * @param start-position The 1-based position to start searching from.
 * @returns Returns a value that indicates the position of the target string within the source string.
 */
FUNCTION INDEX RETURNS INTEGER (
    INPUT source-string AS CHARACTER,
    INPUT target-string AS CHARACTER,
    INPUT start-position AS INTEGER
  ) FORWARD.

FUNCTION INDEX RETURNS INTEGER (
    INPUT source-string AS CHARACTER,
    INPUT target-string AS CHARACTER
  ) FORWARD.
```
