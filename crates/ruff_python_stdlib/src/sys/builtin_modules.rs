//! This file is generated by `scripts/generate_builtin_modules.py`

/// Return `true` if `module` is a [builtin module] on the given
/// Python 3 version.
///
/// "Builtin modules" are modules that are compiled directly into the
/// Python interpreter. These can never be shadowed by first-party
/// modules; the normal rules of module resolution do not apply to these
/// modules.
///
/// [builtin module]: https://docs.python.org/3/library/sys.html#sys.builtin_module_names
#[allow(clippy::unnested_or_patterns)]
pub fn is_builtin_module(minor_version: u8, module: &str) -> bool {
    matches!(
        (minor_version, module),
        (
            _,
            "_abc"
                | "_ast"
                | "_codecs"
                | "_collections"
                | "_functools"
                | "_imp"
                | "_io"
                | "_locale"
                | "_operator"
                | "_signal"
                | "_sre"
                | "_stat"
                | "_string"
                | "_symtable"
                | "_thread"
                | "_tracemalloc"
                | "_warnings"
                | "_weakref"
                | "atexit"
                | "builtins"
                | "errno"
                | "faulthandler"
                | "gc"
                | "itertools"
                | "marshal"
                | "posix"
                | "pwd"
                | "sys"
                | "time"
        ) | (7, "xxsubtype" | "zipimport")
            | (8, "xxsubtype")
            | (9, "_peg_parser" | "xxsubtype")
            | (10, "xxsubtype")
            | (11, "_tokenize" | "xxsubtype")
            | (12, "_tokenize" | "_typing")
            | (13, "_suggestions" | "_sysconfig" | "_tokenize" | "_typing")
    )
}