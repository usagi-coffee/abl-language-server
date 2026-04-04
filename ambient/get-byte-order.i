/**
 * Returns an INTEGER value indicating the byte order setting of a MEMPTR variable. This will be either the
 * value provided by the last execution of SET-BYTE-ORDER with this MEMPTR variable, or HOST-BYTE-ORDER
 * if SET-BYTE-ORDER has not been executed.
 *
 * Syntax
 *
 * `GET-BYTE-ORDER( memptr )`
 *
 * Notes
 * - GET-BYTE-ORDER never affects data currently in the MEMPTR. That is, it does not actually re-order the data.
 *
 * @param memptr An expression that returns a MEMPTR.
 * @returns INTEGER value indicating the byte order setting.
 */
FUNCTION GET-BYTE-ORDER RETURNS INTEGER (
 MEMPTR memptr
 ) FORWARD.
