/**
 * Returns the specified number of bytes, from the specified location, into a RAW or MEMPTR variable.
 *
 * Syntax
 *
 * `GET-BYTES( source , position , numbytes )`
 *
 * @param source An expression that evaluates to a RAW or MEMPTR value indicating the source location. If source is the Unknown value (?), GET-BYTES returns the Unknown value (?).
 * @param position An integer value greater than 0 indicating the byte position of the first byte to get. If position is greater than the length of source, the AVM returns the Unknown value (?). If position is less than 1, the AVM generates a run-time error.
 * @param numbytes An integer value greater than 0 indicating how many bytes to return as a RAW value. If position plus numbytes is greater than the size of source, the AVM returns the Unknown value (?).
 * @returns RAW
 */
FUNCTION GET-BYTES RETURNS RAW (
 source AS RAW,
 position AS INTEGER,
 numbytes AS INTEGER
 ) FORWARD.
