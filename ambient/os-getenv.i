/**
 * Returns a string that contains the value of the desired environment variable
 * in the environment in which the ABL session is running.
 *
 * Syntax
 *
 * `OS-GETENV ( environment-variable )`
 *
 * Notes
 * - If the environment variable is not defined, this statement returns the Unknown value (?).
 * - This function returns the value of an environment variable defined before the ABL session started, not a variable defined during the session.
 * - Since environment variables are case sensitive in some environments, make sure that the name you supply is the correct case.
 * - For security reasons some operating systems may not return certain system environment variable values. In these cases the Unknown value (?) is returned.
 * - When the AVM retrieves an environment variable from the operating system, it assumes that the value is encoded using the same encoding as the internal code page setting (-cpinternal). No conversion is done when reading the value into the AVM's memory.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE pathname AS CHARACTER NO-UNDO.
 * pathname = OS-GETENV("DLC") + "/" + "report.txt".
 * ```
 *
 * @param environment-variable The name of the environment variable whose value you want to find.
 * @returns A string containing the value of the environment variable, or Unknown value (?) if not defined.
 */
FUNCTION OS-GETENV RETURNS CHARACTER (
    INPUT environment-variable AS CHARACTER
  ) FORWARD.
