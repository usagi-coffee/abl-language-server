/**
 * Determines whether a connected database is audit-enabled.
 *
 * Syntax
 *
 * `AUDIT-ENABLED([ integer-expression | logical-name | alias ])`
 *
 * Notes
 * - If you specify a connected database, the AVM queries that database and returns TRUE if it is audit-enabled. If you do not specify a database, the AVM queries all connected databases and returns TRUE if any one of the connected databases is audit-enabled.
 * - You can reference the AUDIT-ENABLED function within a preprocessor &IF expression (such as, &IF AUDIT-ENABLED … &ENDIF). For more information, see the &IF, &THEN, &ELSEIF, &ELSE, and &ENDIF preprocessor directives reference entry.
 *
 * @param sequence_number The sequence number of a connected database to query
 * @param logical_name The logical name or alias of a connected database to query
 * @returns TRUE if the database is audit-enabled, otherwise the Unknown value (?)
 */
FUNCTION AUDIT-ENABLED RETURNS LOGICAL (
 INPUT sequence_number AS INTEGER
 ) FORWARD.
FUNCTION AUDIT-ENABLED RETURNS LOGICAL (
 INPUT logical_name AS CHARACTER
 ) FORWARD.
FUNCTION AUDIT-ENABLED RETURNS LOGICAL (
 ) FORWARD.
