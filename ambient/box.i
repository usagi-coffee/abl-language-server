/**
 * Returns an object reference to a .NET System.Object that contains (boxes) a .NET mapping of an ABL
 * value. At run time, this mapping depends on the kind of ABL value passed to the function. If you pass an ABL
 * primitive value, the function returns a corresponding .NET mapped object type. If you pass an ABL array of
 * .NET-compatible elements, the function returns a corresponding .NET array object type.
 *
 * Syntax
 *
 * `BOX ( ABL-expression [, AS-data-type-expression] )`
 *
 * Notes
 * - For many direct assignments of a System.Object to an ABL primitive value or .NET-compatible ABL
 * array, use of the BOX function is optional, because ABL automatically boxes the assigned ABL value into
 * its default matching .NET object type. However, one such assignment for which you must use the BOX
 * function is when you want to box an ABL primitive value (or primitive array) as a valid .NET mapped data
 * type (or array of mapped types) other than the default match, for example, when boxing an ABL INTEGER
 * (or INTEGER EXTENT) as a .NET System.Byte (or "System.Byte[]").
 * - If you pass a compatible ABL value or array to an INPUT parameter of a .NET method, ABL automatically
 * boxes the ABL value into the matching .NET System.Object or array object. For an ABL primitive (or
 * primitive array) value, this automatic boxing also allows you to explicitly specify the .NET data type mapping
 * if you use the AS data type option on the ABL argument that you pass to the INPUT parameter. This automatic
 * boxing does not occur for an ABL method, procedure, or user-defined function passing the same parameters.
 * In this case, you can do an initial direct assignment or use the BOX function to explicitly do the necessary
 * conversion.
 *
 * Example
 *
 * ```abl
 * USING System.Data.* FROM ASSEMBLY.
 * DEFINE VARIABLE dataTable1 AS DataTable NO-UNDO.
 * DEFINE VARIABLE dcCustNum AS DataColumn NO-UNDO.
 * DEFINE VARIABLE dcName AS DataColumn NO-UNDO.
 * DEFINE VARIABLE dcBusType AS DataColumn NO-UNDO.
 * DEFINE VARIABLE row1 AS DataRow NO-UNDO.
 * dataTable1 = NEW DataTable(INPUT "Customer").
 * dcCustNum = NEW DataColumn(INPUT "CustNum").
 * dcName = NEW DataColumn(INPUT "Name").
 * dcBusType = NEW DataColumn(INPUT "BusType").
 * dataTable1:Columns:Add(INPUT dcCustNum).
 * dataTable1:Columns:Add(INPUT dcName).
 * dataTable1:Columns:Add(INPUT dcBusType).
 * row1 = dataTable1:NewRow( ).
 * row1:Item["CustNum"] = 1.
 * row1:Item["Name"] = "Mr Jones".
 * row1:Item["BusType"] = BOX( 236, "UNSIGNED-BYTE").
 * ```
 *
 * @param ABL-expression Specifies an expression with a value in a .NET-compatible ABL data type (CHARACTER,
 * DATE, DATETIME, DATETIME-TZ, DECIMAL, INT64, INTEGER, LOGICAL, LONGCHAR, or a .NET-compatible ABL array).
 * @param AS-data-type-expression A character expression equal to a keyword (AS data type) that matches the explicit
 * .NET mapped data type into which you want to box the specified ABL-expression.
 * @returns An object reference to a .NET System.Object that contains (boxes) a .NET mapping of an ABL value.
 */
FUNCTION BOX RETURNS System.Object (
 ABL-expression AS System.Object,
 AS-data-type-expression AS CHARACTER
 ) FORWARD.
