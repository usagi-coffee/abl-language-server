/**
 * Returns the normalized form of a character string based on the specified Unicode normalization form.
 *
 * Syntax
 *
 * `NORMALIZE ( string , normalization-form )`
 *
 * Notes
 * - The source string may be of type CHARACTER or LONGCHAR.
 * - If the string is a CHARACTER value, -cpinternal must be set to UTF-8.
 * - If the string is a LONGCHAR value, its code page can be any form of Unicode (for example, UTF-8, UTF-16, or UTF-32).
 * - The normalization-form must evaluate to one of: NFD, NFC, NFKD, NFKC, or NONE.
 * - NFD: Canonical Decomposition
 * - NFC: Canonical Decomposition, followed by Canonical Composition
 * - NFKD: Compatibility Decomposition
 * - NFKC: Compatibility Decomposition, followed by Canonical Composition
 * - NONE: Returns the source string unchanged
 *
 * @param source-string The source string to normalize.
 * @param normalization-form A character expression that evaluates to a Unicode normalization form.
 * @returns Returns the normalized form of the source string with the same data type as the input.
 */
FUNCTION NORMALIZE RETURNS CHARACTER (
    INPUT source-string AS CHARACTER,
    INPUT normalization-form AS CHARACTER
  ) FORWARD.

FUNCTION NORMALIZE RETURNS LONGCHAR (
    INPUT source-string AS LONGCHAR,
    INPUT normalization-form AS CHARACTER
  ) FORWARD.
