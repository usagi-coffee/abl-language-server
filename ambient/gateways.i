/**
 * The GATEWAYS function has been replaced by the DATASERVERS function,
 * which is exactly equivalent. This function is supported only for
 * backward compatibility.
 *
 * Syntax
 *
 * `GATEWAYS`
 *
 * Notes
 * - Does not apply to SpeedScript programming.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE db-types AS CHARACTER NO-UNDO VIEW-AS SELECTION-LIST
 * INNER-CHARS 20 INNER-LINES 3 LABEL "DataServers".
 * FORM db-types.
 * db-types:LIST-ITEMS = GATEWAYS.
 * UPDATE db-types.
 * ```
 *
 * @returns A character string containing a comma-separated list of database types.
 */
FUNCTION GATEWAYS RETURNS CHARACTER (
 ) FORWARD.
