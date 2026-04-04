/**
 * Returns a list of database types your OpenEdge product supports from where
 * it is executed. The DATASERVERS function takes no arguments.
 *
 * Syntax
 *
 * `DATASERVERS`
 *
 * Notes
 * - The DATASERVERS function returns a character string containing a comma-separated
 * list of database types. For example: "PROGRESS,MSS,ORACLE"
 * - Which indicates licensed support for the OpenEdge database, DataServer for
 * Microsoft SQL Server, and DataServer for Oracle.
 * - You can use the returned string with the LOOKUP function to determine whether
 * a particular type of database is supported.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE db-types AS CHARACTER NO-UNDO VIEW-AS SELECTION-LIST
 * INNER-CHARS 20 INNER-LINES 3 LABEL "DataServers".
 * FORM db-types.
 * db-types:LIST-ITEMS = DATASERVERS.
 * UPDATE db-types.
 * ```
 *
 * @returns CHARACTER - A comma-separated list of database types supported by the
 * OpenEdge product
 */
FUNCTION DATASERVERS RETURNS CHARACTER (
 ) FORWARD.
