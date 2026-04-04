/**
 * Returns a character string (CHARACTER or LONGCHAR) entry from a list based on an integer position.
 * The data type of the returned value matches the data type of the list element.
 *
 * Syntax
 *
 * `ENTRY ( element , list [ , character ] )`
 *
 * Notes
 * - The ENTRY function is double-byte enabled. It can return an entry that contains double-byte
 * characters from a specified list and the character delimiter can be a double-byte character.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE daynam AS CHARACTER NO-UNDO INITIAL
 * "Sunday,Monday,Tuesday,Wednesday,Thursday,Friday,Saturday".
 * DISPLAY ENTRY(WEEKDAY(TODAY), daynam) FORMAT "x(9)" LABEL "is a" WITH SIDE-LABELS.
 * ```
 *
 * @param element An integer value that corresponds to the position of a character string in a list
 * of values. If the value of element does not correspond to an entry in the list,
 * the AVM raises the ERROR condition. If the value of element is the Unknown value (?),
 * ENTRY returns the Unknown value (?). If element is less than or equal to 0, or is
 * larger than the number of elements in list, ENTRY returns an error.
 * @param list A list of character strings separated with a character delimiter. The list can be a
 * variable of type CHARACTER or LONGCHAR. If the value of list is the Unknown value (?),
 * ENTRY returns the Unknown value (?).
 * @param character A delimiter you define for the list. The default is a comma. This allows the
 * ENTRY function to operate on non-comma-separated lists. If you use an alphabetic
 * character, this delimiter is case sensitive.
 * @returns CHARACTER or LONGCHAR - The character string at the specified position in the list.
 */
FUNCTION ENTRY RETURNS CHARACTER (
 INPUT element AS INTEGER,
 INPUT list AS CHARACTER,
 INPUT character AS CHARACTER
 ) FORWARD.

FUNCTION ENTRY RETURNS CHARACTER (
 INPUT element AS INTEGER,
 INPUT list AS CHARACTER
 ) FORWARD.

FUNCTION ENTRY RETURNS LONGCHAR (
 INPUT element AS INTEGER,
 INPUT list AS LONGCHAR,
 INPUT character AS CHARACTER
 ) FORWARD.

FUNCTION ENTRY RETURNS LONGCHAR (
 INPUT element AS INTEGER,
 INPUT list AS LONGCHAR
 ) FORWARD.
