/**
 * Identifies the bit value of the AVM process you are using at run time and returns this value as an INTEGER.
 * Valid values are 32 for a 32-bit AVM and 64 for 64-bit AVM.
 *
 * Syntax
 *
 * `PROCESS-ARCHITECTURE`
 *
 * Notes
 * - This function has a corresponding preprocessor. {&PROCESS-ARCHITECTURE} expands to either "32" or "64".
 * - For example, an application that uses a 32-bit Windows API through the DLL external procedure interface
 * may need to pass different parameters to a function when running a 64-bit GUI client than when running
 * a 32-bit GUI client. Using PROCESS-ARCHITECTURE, you can decide the parameters to pass.
 *
 * Example
 *
 * ```abl
 * IF PROCESS-ARCHITECTURE = 64 THEN
 * MESSAGE "64-bit process running on 64-bit Windows".
 * ELSE
 * MESSAGE "32-bit process".
 * ```
 *
 * @returns INTEGER value of 32 for a 32-bit AVM or 64 for a 64-bit AVM.
 */
FUNCTION PROCESS-ARCHITECTURE RETURNS INTEGER (
 )
 FORWARD.
