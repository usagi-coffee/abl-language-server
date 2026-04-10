/**
 * Invokes a user-defined function. The AVM evaluates the name of the function (and the procedure handle, if any) at run time.
 *
 * Syntax
 *
 * `DYNAMIC-FUNCTION( function-name [, param1 [, param2] ... ] ) [ IN proc-handle ]`
 *
 * Notes
 * - ABL cannot check the mode and type of the parameters at compile time, since the AVM does not evaluate function-name until run time.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE funcs AS CHARACTER NO-UNDO EXTENT 5
 * INITIAL ["firstrec","lastrec","nextrec","prevrec","quitting"].
 *
 * FUNCTION dispcust RETURNS LOGICAL:
 * DISPLAY Customer EXCEPT Customer.Comments WITH FRAME x.
 * END.
 *
 * FUNCTION firstrec RETURNS LOGICAL:
 * FIND FIRST Customer.
 * dispcust().
 * RETURN yes.
 * END.
 *
 * /* Main routine */
 * REPEAT WHILE NOT alldone:
 * idx = LOOKUP(action,"f,l,n,p,q").
 * DISPLAY DYNAMIC-FUNCTION(funcs[idx]) LABEL "Record Found?".
 * END.
 * ```
 *
 * @param function-name A CHARACTER expression that returns the name of a user-defined function. The AVM evaluates function-name at run time.
 * @param param1, param2, ... Parameters of the user-defined function. You must supply names of actual data items—actual parameter names—not CHARACTER expressions that return parameter names.
 * @param proc-handle An expression that returns a handle to the procedure that defines the function. The AVM evaluates proc-handle at run time.
 * @returns The return value of the invoked user-defined function.
 */
FUNCTION DYNAMIC-FUNCTION RETURNS LOGICAL (
 function-name AS CHARACTER
 ) FORWARD.

FUNCTION DYNAMIC-FUNCTION RETURNS LOGICAL (
 function-name AS CHARACTER,
 INPUT proc-handle AS HANDLE
 ) FORWARD.
