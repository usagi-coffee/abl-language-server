/**
 * Returns the return status from a stored procedure. The return status is an INTEGER value that indicates whether
 * a stored procedure failed and why.
 *
 * Syntax
 *
 * `PROC-STATUS`
 *
 * Notes
 * - For descriptions of the possible values for the return status of a non-ABL stored procedure, see the
 * procedure's documentation.
 * - For more information on using this function, see the OpenEdge DataServer Guides (Use the Microsoft SQL
 * Data Server and Use the Oracle Data Server).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE iStat AS INTEGER NO-UNDO.
 * RUN STORED-PROCEDURE pcust (10, OUTPUT 0, OUTPUT 0).
 * FOR EACH proc-text-buffer:
 * END.
 * CLOSE STORED-PROCEDURE pcust iStat = PROC-STATUS.
 * DISPLAY iStat.
 * ```
 *
 * @returns The return status from a stored procedure (INTEGER).
 */
FUNCTION PROC-STATUS RETURNS INTEGER (
 ) FORWARD.
