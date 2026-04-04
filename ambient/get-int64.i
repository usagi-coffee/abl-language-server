/**
 * GET-INT64 function
 *
 * @description Returns the signed 64-bit value at the specified memory location as an INT64 value.
 *
 * @syntax GET-INT64 ( source , position )
 *
 * @param source A function or variable that returns a RAW or MEMPTR value. If source is the Unknown value (?), GET-INT64 returns the Unknown value (?).
 * @param position An integer value greater than 0 that indicates the byte position where you want to find the information. If position is greater than the length of source, the AVM returns the Unknown value (?). If position is less than 1, the AVM generates a run-time error.
 *
 * @returns INT64 - The signed 64-bit value at the specified memory location.
 *
 * @example
 *
 * r-getint64.p
 *
 * DEFINE VARIABLE myint64 AS INT64 NO-UNDO INITIAL 7888999000.
 * DEFINE VARIABLE myint AS INTEGER NO-UNDO INITIAL 2345.
 * DEFINE VARIABLE myraw AS RAW NO-UNDO.
 * DEFINE VARIABLE result64 AS INT64 NO-UNDO.
 * DEFINE VARIABLE result1 AS INTEGER NO-UNDO.
 * DEFINE VARIABLE mymem AS MEMPTR NO-UNDO.
 *
 * ASSIGN
 *     PUT-LONG(myraw,1) = myint
 *     result1 = GET-LONG(myraw,1)
 *     PUT-INT64(myraw,1) = myint64
 *     result64 = GET-INT64(myraw,1).
 * MESSAGE "raw version " result1 result64.
 *
 * ASSIGN
 *     result1 = ?
 *     result64 = ?
 *     SET-SIZE(mymem) = 40
 *     PUT-LONG(mymem,1) = myint
 *     result1 = GET-LONG(mymem,1).
 *
 * PUT-INT64(mymem,1) = myint64
 * result64 = GET-INT64(mymem,1).
 * MESSAGE "memptr version " result1 result64.
 * result1 = get-int64(mymem,1) NO-ERROR.
 * MESSAGE "store getint64 in int gives" ERROR-STATUS:GET-MESSAGE(1).
 * MESSAGE "doing put-long of in64" myint64.
 * PUT-LONG(mymem,1) = myint64 NO-ERROR.
 * MESSAGE "storeint64 with put-long gives" ERROR-STATUS:GET-MESSAGE(1).
 * SET-SIZE(mymem) = 0.
 */
FUNCTION GET-INT64 RETURNS INT64 (source AS RAW, position AS INTEGER) FORWARD.
FUNCTION GET-INT64 RETURNS INT64 (source AS MEMPTR, position AS INTEGER) FORWARD.
