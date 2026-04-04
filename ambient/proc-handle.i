/**
 * Returns a value in the appropriate data type (usually INTEGER) that is a unique identifier for a stored
 * procedure.
 *
 * Syntax
 *
 * `PROC-HANDLE`
 *
 * Notes
 * - Progress Software Corporation recommends that you specify a procedure handle for each stored procedure
 * that you run.
 * - You do not have to specify a handle if there is only one active stored procedure and you do not include
 * SQL statements in the ABL application. In the case of ORACLE only, the DataServer passes SQL statements
 * to the ORACLE RDBMS and uses the default system handle in the process.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE iHandle AS INTEGER NO-UNDO.
 * RUN STORED-PROCEDURE pcust iHandle = PROC-HANDLE (10, OUTPUT 0, OUTPUT 0).
 * FOR EACH proc-text-buffer WHERE PROC-HANDLE = iHandle:
 * DISPLAY proc-text.
 * END.
 * CLOSE STORED-PROCEDURE pcust WHERE PROC-HANDLE = iHandle.
 * ```
 *
 * @returns A value in the appropriate data type (usually INTEGER) that is a unique identifier for a stored
 * procedure.
 */
FUNCTION PROC-HANDLE RETURNS INTEGER (
 ) FORWARD.
