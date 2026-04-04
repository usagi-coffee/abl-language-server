/**
 * Returns an INTEGER error code indicating whether an execution error occurred during the last OS-APPEND, OS-COPY, OS-CREATE-DIR, OS-DELETE, OS-RENAME or SAVE CACHE statement.
 *
 * Syntax
 *
 * `OS-ERROR`
 *
 * Notes
 * - Returns 0 if no error occurred.
 * - Use this function immediately following an OS-APPEND, OS-COPY, OS-CREATE-DIR, OS-DELETE, OS-RENAME, or SAVE CACHE statement to determine whether an error occurred.
 * - The next use of one of these statements overwrites the previous error code.
 * - Error codes include: 0 (No error), 1 (Not owner), 2 (No such file or directory), 4 (Interrupted system call), 5 (I/O error), 6 (Bad file number), 11 (No more processes), 12 (Not enough core memory), 13 (Permission denied), 14 (Bad address), 17 (File exists), 19 (No such device), 20 (Not a directory), 21 (Is a directory), 23 (File table overflow), 24 (Too many open files), 27 (File too large), 28 (No space left on device), 39 (Directory not empty), 49 (Unmapped error).
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE err-status AS INTEGER NO-UNDO.
 * DEFINE VARIABLE filename AS CHARACTER NO-UNDO FORMAT "x(40)" LABEL "Enter a file to delete".
 * UPDATE filename.
 * OS-DELETE VALUE(filename).
 * err-status = OS-ERROR.
 * IF err-status <> 0 THEN
 *   CASE err-status:
 *     WHEN 1 THEN MESSAGE "You are not the owner of this file or directory.".
 *     WHEN 2 THEN MESSAGE "The file or directory you want to delete does not exist.".
 *     OTHERWISE DISPLAY "OS Error #" + STRING(OS-ERROR,"99") FORMAT "x(13)" WITH FRAME b.
 *   END CASE.
 * ```
 *
 * @returns An INTEGER value that indicates whether an execution error occurred (0 = no error).
 */
FUNCTION OS-ERROR RETURNS INTEGER ( ) FORWARD.
