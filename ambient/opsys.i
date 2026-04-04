/**
 * Identifies the operating system being used, so that a single version of a procedure can work differently under
 * different operating systems. Returns the value of that operating system. Valid values are "UNIX" and "WIN32".
 * Note: For both 32-bit and 64-bit Windows operating systems, OPSYS returns the value "WIN32".
 *
 * Syntax
 *
 * `OPSYS`
 *
 * Notes
 * - ABL supports an override option that enables applications that need to return the value of MS-DOS for all
 * Microsoft operating systems to do so. For example, if you do not want the value WIN32 returned when the
 * Windows operating system is recognized, you can override this return value by defining the Opsys key in the
 * Startup section of the current environment, which may be in the registry or in an initialization file. If the
 * Opsys key is located, the OPSYS function returns the value associated with the Opsys key on all platforms.
 *
 * Example
 *
 * ```abl
 * IF OPSYS = "UNIX" THEN UNIX ls.
 * ELSE IF OPSYS = "WIN32" THEN DOS dir.
 * ELSE MESSAGE OPSYS "is an unsupported operating system".
 * ```
 *
 * @returns CHARACTER The operating system name ("UNIX" or "WIN32").
 */
FUNCTION OPSYS RETURNS CHARACTER (
 ) FORWARD.
