/**
 * Returns, as a character string, the name of the logical database currently in use or the name of your first
 * connected database.
 *
 * Syntax
 *
 * `DBNAME`
 *
 * Notes
 * - The AVM returns the database name in the same form you used when you connected to the database. If
 * you used a fully qualified pathname, the AVM returns the full directory pathname (such as /usr/acctg/gl
 * on UNIX or C:\acctg\gl in Windows). If you used a name relative to your current working directory, then
 * the AVM returns that name (for example, gl).
 * - Unless you define a format, the database name is displayed in a character field with the default format of
 * x(8).
 * - A database must be connected in order for the DBNAME function to work as described.
 *
 * Example
 *
 * ```abl
 * FORM HEADER "Date:" TO 10 TODAY
 * "Page:" AT 65 pageno SKIP
 * "Database:" TO 10 DBNAME FORMAT "x(60)" SKIP
 * "Userid:" TO 10 USERID WITH NO-BOX NO-LABELS.
 * ```
 *
 * @returns CHARACTER - The name of the logical database currently in use or the name of your first connected database.
 */
FUNCTION DBNAME RETURNS CHARACTER (
 ) FORWARD.
