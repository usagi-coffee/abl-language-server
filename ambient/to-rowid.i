/**
 * Converts a string representation of a ROWID to a valid ROWID value.
 *
 * Syntax
 *
 * `TO-ROWID ( rowid-string )`
 *
 * Notes
 * - The rowid-string must be in the form "0xhex-digits", where hex-digits is any string of characters from 0 through 9 and A through F.
 * - There is no guarantee that the returned ROWID value corresponds to an existing record in your database.
 *
 * Example
 *
 * ```abl
 * FIND Customer WHERE ROWID(Customer) = TO-ROWID("0x1234").
 * ```
 *
 * @param rowid-string A string representation of a ROWID.
 * @returns A valid ROWID value.
 */
FUNCTION TO-ROWID RETURNS ROWID (
    INPUT rowid-string AS CHARACTER
  ) FORWARD.
