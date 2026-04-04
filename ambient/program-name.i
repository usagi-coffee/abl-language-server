/**
 * Returns the name of the calling program.
 *
 * Syntax
 *
 * `PROGRAM-NAME ( n )`
 *
 * Notes
 * - If n is 1, the name of the current program is returned. If n is 2, the name of the calling program is returned. If there is no calling program then you have reached the top of the call stack and the AVM returns the Unknown value (?).
 * - If you execute a procedure directly from the Procedure Editor or the AppBuilder, then PROGRAM-NAME(1) returns the name of a temporary file rather than the name of the actual procedure file.
 * - If the procedure you reference is an internal procedure, then PROGRAM-NAME returns a string with the following form: "internal-procedure-name source-file-name"
 * - If the procedure you reference is a user interface trigger associated with a widget, then PROGRAM-NAME returns a string with the following form: "USER-INTERFACE-TRIGGER source-file-name"
 * - If the procedure you reference is a user interface trigger that uses the ANYWHERE keyword, then PROGRAM-NAME returns a string with the following form: "SYSTEM-TRIGGER source-file-name"
 * - If the procedure you reference is a session database trigger, then PROGRAM-NAME returns a string with the following form: "type-TRIGGER source-file-name" where type is either ASSIGN, CREATE, DELETE, FIND, or WRITE.
 * - If the call stack contains a method reference, then PROGRAM-NAME returns a string with the following form: "method-name class-file-name" where class-file-name is the name of the class definition (.cls) file in which method-name is implemented.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE level AS INTEGER NO-UNDO INITIAL 1.
 * REPEAT WHILE PROGRAM-NAME(level) <> ?:
 *     DISPLAY LEVEL PROGRAM-NAME(level) FORMAT "x(30)".
 *     level = level + 1.
 * END.
 * ```
 *
 * @param n The numeric argument. If n is 1, the name of the current program is returned. If n is 2, the name of the calling program is returned.
 * @returns The name of the program at the specified level in the call stack, or the Unknown value (?) if there is no calling program.
 */
FUNCTION PROGRAM-NAME RETURNS CHARACTER (
    INPUT n AS INTEGER
  ) FORWARD.
