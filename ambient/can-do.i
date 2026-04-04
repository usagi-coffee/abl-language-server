/**
 * Checks a user ID against a list of one or more user ID matching patterns that can be used to indicate what
 * users have access to a given application function. The function returns TRUE if the specified user ID has access
 * according to the list. Thus, you can implement run-time authorization checking for any procedure or class in
 * your application.
 *
 * Syntax
 *
 * `CAN-DO ( id-pattern-list [, userid] )`
 *
 * Notes
 * - If id-pattern-list contains contradictory values, the first occurrence of a value in the list applies. For
 * example, CAN-DO("abc,!abc*") evaluates to TRUE, because the user ID, "abc", appears before
 * "!abc" in id-pattern-list.
 * - If id-pattern-list is exhausted without a match, CAN-DO returns a value of FALSE. Therefore, "!abc"
 * restricts "abc" and everyone else, including the blank user ID (""). To restrict "abc" only and allow
 * everyone else, use "!abc,*".
 * - A userid comparison against id-pattern-list is not case sensitive.
 * - If a user is logged into a UNIX system as root, the AVM allows access to the procedure even if access is
 * denied by the id-pattern-list. You must specifically deny root access by adding "!root" to the
 * id-pattern-list.
 * - In addition to the examples shown above, you can use the CAN-DO function to compare a userid other
 * than that of the current user against the list of values in id-pattern-list.
 * - You can pass user IDs to the CAN-DO function, other than the default, that have been set using command-line
 * database connections, the CONNECT statement, the SECURITY-POLICY:SET-CLIENT( ) method, the SET-DB-CLIENT
 * function, or the SETUSERID function.
 * - You can use the CAN-DO function to match a user ID against run-time table and field permissions stored in
 * an OpenEdge RDBMS by accessing the user ID patterns stored in the _Can-* fields of the _File
 * metaschema table.
 * - ABL raises an error if you omit userid and one of the following conditions exists:
 * * There is no database connected.
 * * More than one database is currently connected.
 * - By default, CAN-DO treats "@" as a delimiter for the domain name in a fully qualified user ID. However, you
 * can change this functionality so that CAN-DO treats "@" as a regular character by using the
 * CAN-DO-DOMAIN-SUPPORT attribute of the SECURITY-POLICY handle or the -nocandodomain startup parameter.
 * - Using the # character as an argument to the CAN-DO function may lead to incorrect results. # should only
 * be used to distinguish GRANTable versus non-GRANTable permissions in SQL89.
 *
 * Example
 *
 * ```abl
 * DO FOR permission:
 * FIND permission WHERE Activity = "custedit".
 * IF NOT CAN-DO(permission.Can-Run, USERID) THEN DO:
 * MESSAGE "You are not authorized to run this procedure".
 * RETURN.
 * END.
 * END.
 * ```
 *
 * @param id-pattern-list A constant, field name, variable name, or expression that evaluates to a list of one or more user ID patterns.
 * @param userid A character expression that evaluates to a user ID value. If omitted, defaults to USERID function.
 * @returns LOGICAL - TRUE if the specified user ID has access according to the list, FALSE otherwise.
 */
FUNCTION CAN-DO RETURNS LOGICAL (
 INPUT id-pattern-list AS CHARACTER,
 INPUT userid AS CHARACTER
 ) FORWARD.

FUNCTION CAN-DO RETURNS LOGICAL (
 INPUT id-pattern-list AS CHARACTER
 ) FORWARD.
