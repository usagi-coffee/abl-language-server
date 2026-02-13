// We should be able to go to files for relative paths
{includes/include.i}

// We should be able to go to files directly from `propath` entry in abl.toml
{include.i}

// We should be able to jump to this definition
FUNCTION local_mul RETURNS INTEGER (INPUT p_a AS INTEGER, INPUT p_b AS INTEGER):
  RETURN p_a * p_b.
END FUNCTION.

// Schema identifier should be a semantic token
FIND z9zw_mstr.

// We should not be able to get completion before the definitions
// lv_

// After these we should be able to autcomplete them
DEFINE VARIABLE lv_before AS CHARACTER NO-UNDO.
DEFINE VARIABLE lv_counter AS INTEGER NO-UNDO.
DEFINE VARIABLE lv_name AS CHAR NO-UNDO.

UPPER('x').

// Unknown symbol diagnostics should be case-insensitive.
lv_counter = missing_var.
lv_counter = Missing_Func(lv_counter).

// Type mismatch, should error
Lv_before = 1.

/* Hover + goto-definition/references on local symbols. */
lv_counter = 1.
lv_counter = local_mul(lv_counter, 2).


// Go to definition on z9zw_mstr should go to definition in .df file
FIND FIRST z9zw_mstr NO-LOCK NO-ERROR.

// We should get completions on pressin dot after known table
lv_name = z9zw_mstr.z9zw_name.

// Buffers act like aliases, should be handled the same (go to definition, completion)
DEFINE BUFFER b_mstr FOR z9zw_mstr.
lv_before = b_mstr.z9zw_name.

// Local temp/work tables should also support dot field completion.
DEFINE TEMP-TABLE tt_local NO-UNDO
  FIELD id AS INTEGER
  FIELD name AS CHARACTER.

DEFINE BUFFER b_tt FOR tt_local.
lv_name = tt_local.name.
lv_name = b_tt.name.

// Hover should work
MESSAGE "index name" z9zw_mstr.z9zw_name VIEW-AS ALERT-BOX INFO BUTTONS OK.

lv_counter = inc_plus(lv_before).
lv_counter = inc_minus(lv_counter).
lv_counter = local_mul(lv_counter, 2). // Signature help on '(' and ','

// Should error ecause both parameters are INTEGER
local_mul("5", 1).

// Wrong arity should produce an error diagnostic.
lv_counter = local_mul(lv_counter).
lv_counter = inc_plus(lv_counter, INTEGER(1)).
lv_counter = inc_minus(lv_counter, ?).
