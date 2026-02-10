// We should be able to go to files for relative paths
{includes/include.i}

// We should be able to go to files directly from `propath` entry in abl.toml
{include.i}

// We should be able to jump to this definition
FUNCTION local_mul RETURNS INTEGER (INPUT p_a AS INTEGER, INPUT p_b AS INTEGER):
  RETURN p_a * p_b.
END FUNCTION.

// We should not be able to get completion before the definitions
// lv_

// After these we shuold be able to autcomplete them
DEFINE VARIABLE lv_before AS CHARACTER NO-UNDO.
DEFINE VARIABLE lv_counter AS INTEGER NO-UNDO.
DEFINE VARIABLE lv_name AS CHARACTER NO-UNDO.

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

// Hover should work
MESSAGE "index name" z9zw_mstr.z9zw_name VIEW-AS ALERT-BOX INFO BUTTONS OK.

lv_counter = inc_plus(lv_counter).
lv_counter = inc_minus(lv_counter).

// Wrong arity should produce an error diagnostic.
lv_counter = local_mul(lv_counter).
lv_counter = inc_plus(lv_counter, INTEGER(1)).
lv_counter = inc_minus(lv_counter, ?).
