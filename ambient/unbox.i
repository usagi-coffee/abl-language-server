/**
 * Unboxes a .NET System.Object or array object and returns a value of a corresponding ABL primitive or
 * array type.
 *
 * Syntax
 *
 * `UNBOX ( object-reference )`
 *
 * Notes
 * - You must use the UNBOX function if you want to reference an appropriate System.Object property or
 *   method return value in an ABL primitive expression.
 * - For any direct assignment of a .NET object or object array to a compatible ABL primitive value or array,
 *   use of the UNBOX function is optional, because ABL automatically unboxes the underlying .NET object or
 *   array object type to its matching ABL primitive or array type.
 * - You must specify the -clrnetcore startup parameter to use the UNBOX function on Linux.
 *
 * Example
 *
 * ```abl
 * USING System.Data.* FROM ASSEMBLY.
 * DEFINE VARIABLE dataTable1 AS DataTable.
 * DEFINE VARIABLE dcCustNum AS DataColumn.
 * DEFINE VARIABLE row1 AS DataRow.
 * DEFINE VARIABLE iVal AS INTEGER.
 * dataTable1 = NEW DataTable( INPUT "Customer" ).
 * dcCustNum = NEW DataColumn( INPUT "CustNum" ).
 * dataTable1:COLUMNS:ADD( INPUT dcCustNum ).
 * row1 = dataTable1:NewRow( ).
 * row1:Item["CustNum"] = 5.
 * iVal = UNBOX( row1:Item["CustNum"] ) MODULO 2.
 * ```
 *
 * @param object-reference Specifies an object reference to a boxed .NET primitive value (System.Object) or to a
 *   one-dimensional .NET array object.
 * @returns A value of the ABL primitive type that implicitly maps to the boxed .NET type, or an ABL array
 *   containing elements from the .NET array.
 */
FUNCTION UNBOX RETURNS HANDLE (
    INPUT object-reference AS HANDLE
  ) FORWARD.

FUNCTION UNBOX RETURNS RAW (
    INPUT object-reference AS RAW
  ) FORWARD.

FUNCTION UNBOX RETURNS CHARACTER (
    INPUT object-reference AS CHARACTER
  ) FORWARD.

FUNCTION UNBOX RETURNS INTEGER (
    INPUT object-reference AS INTEGER
  ) FORWARD.

FUNCTION UNBOX RETURNS DECIMAL (
    INPUT object-reference AS DECIMAL
  ) FORWARD.

FUNCTION UNBOX RETURNS LOGICAL (
    INPUT object-reference AS LOGICAL
  ) FORWARD.

FUNCTION UNBOX RETURNS DATE (
    INPUT object-reference AS DATE
  ) FORWARD.

FUNCTION UNBOX RETURNS DATETIME (
    INPUT object-reference AS DATETIME
  ) FORWARD.

FUNCTION UNBOX RETURNS DATETIME-TZ (
    INPUT object-reference AS DATETIME-TZ
  ) FORWARD.

FUNCTION UNBOX RETURNS INT64 (
    INPUT object-reference AS INT64
  ) FORWARD.

FUNCTION UNBOX RETURNS MEMPTR (
    INPUT object-reference AS MEMPTR
  ) FORWARD.

FUNCTION UNBOX RETURNS LONGCHAR (
    INPUT object-reference AS LONGCHAR
  ) FORWARD.

FUNCTION UNBOX RETURNS RECID (
    INPUT object-reference AS RECID
  ) FORWARD.

FUNCTION UNBOX RETURNS ROWID (
    INPUT object-reference AS ROWID
  ) FORWARD.

FUNCTION UNBOX RETURNS COM-HANDLE (
    INPUT object-reference AS COM-HANDLE
  ) FORWARD.

FUNCTION UNBOX RETURNS PROGRESS.Lang.Object (
    INPUT object-reference AS PROGRESS.Lang.Object
  ) FORWARD.
