/**
 * Runs the super procedure version of the current user-defined function.
 *
 * Syntax
 *
 * `SUPER [ ( parameter [, parameter ] ... ) ]`
 *
 * Notes
 * - This language element must appear within a user-defined function, but can appear anywhere within the user-defined function.
 * - The parameters passed to SUPER must have the same signature (number of parameters, and type and mode of each) as the parameters of the current user-defined function.
 * - If a user-defined function cannot be located in any super procedure, the AVM generates an error message.
 * - To run the super version of an internal procedure, use the RUN SUPER statement.
 *
 * Example
 *
 * ```abl
 * FUNCTION getCustomerName RETURNS CHARACTER (INPUT custNum AS INTEGER):
 *   RETURN SUPER(custNum).
 * END FUNCTION.
 * ```
 *
 * @returns The return value from the super procedure's version of the function.
 */
FUNCTION SUPER RETURNS CHARACTER (
  ) FORWARD.
