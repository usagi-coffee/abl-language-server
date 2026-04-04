/**
 * Returns an INTEGER value that represents a combination of a red, green, and blue color value.
 *
 * Syntax
 *
 * `RGB-VALUE ( redval , greenval , blueval )`
 *
 * Notes
 * - The RGB-VALUE function is generally most useful when it is used with ActiveX Controls.
 *
 * Example
 *
 * ```abl
 * DEFINE VARIABLE hdlControl AS COM-HANDLE NO-UNDO.
 * hdlControl:BackColor = RGB(128, 0, 256).
 * ```
 *
 * @param redval The red color value (0-255).
 * @param greenval The green color value (0-255).
 * @param blueval The blue color value (0-255).
 * @returns An INTEGER value that represents a combination of red, green, and blue color values.
 */
FUNCTION RGB-VALUE RETURNS INTEGER (
    INPUT redval AS INTEGER,
    INPUT greenval AS INTEGER,
    INPUT blueval AS INTEGER
  ) FORWARD.
