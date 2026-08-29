/// Complex enums exposed as one flat pyclass per variant, driven by one table per family.
///
/// flat_variants!(PyFamily {
///     Variant => PyFamilyVariant as "FamilyVariant" { field: Type },  // fielded class
///     Variant => PyFamilyVariant as "FamilyVariant",                  // unit class
/// });
///
/// Each row emits a frozen value-semantic `#[pyclass]` struct (eq + hash; `get_all` iff
/// fielded). The table then emits the Rust enum — one tuple variant per row, table order =
/// declaration order — and its IntoPyObject
/// dispatch, whose union OUTPUT_TYPE makes generated stubs type fields as
/// `VariantA | VariantB | ...`. The table IS the declaration: grep a class name to find its row.
///
/// Call sites must have `use pyo3::prelude::*` in scope (the expansion resolves pyclass,
/// PyTypeInfo, Bound and type_hint_union! at the invocation site). rustfmt does not format
/// invocation bodies — keep one row per variant. Ground truth: `cargo expand ffi::amount`.
macro_rules! flat_variants {
    // @cls PyAmountAbsolute, "AmountAbsolute" { amount: u16 }
    (@cls $cls:ident, $name:literal { $($f:ident : $t:ty),+ $(,)? }) => {
        #[pyclass(
            skip_from_py_object,
            eq,
            hash,
            frozen,
            get_all,
            name = $name,
            module = "slai.slai"
        )]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $cls { $(pub $f: $t,)+ }
    };
    // @cls PyAmountEventGoldAsk, "AmountEventGoldAsk"
    (@cls $cls:ident, $name:literal) => {
        #[pyclass(skip_from_py_object, eq, hash, frozen, name = $name, module = "slai.slai")]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $cls;
    };
    // @enum PyAmount { Absolute => PyAmountAbsolute, Range => PyAmountRange }
    (@enum $enum:ident { $($variant:ident => $cls:ident),+ }) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $enum { $($variant($cls),)+ }
        flat_variants!(@into $enum { $($variant => $cls),+ });
    };

    // @into PyAmount { Absolute => PyAmountAbsolute, Range => PyAmountRange }
    (@into $enum:ident { $($variant:ident => $cls:ident),+ }) => {
        impl<'py> IntoPyObject<'py> for $enum {
            type Target = PyAny;
            type Output = Bound<'py, PyAny>;
            type Error = PyErr;
            const OUTPUT_TYPE: PyStaticExpr =
                type_hint_union!($(<$cls as PyTypeInfo>::TYPE_HINT),+);
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                Ok(match self {
                    $( Self::$variant(v) => Bound::new(py, v)?.into_any(), )+
                })
            }
        }
    };

    ($enum:ident {
        $($variant:ident => $cls:ident as $name:literal $({ $($f:ident : $t:ty),+ $(,)? })?),+ $(,)?
    }) => {
        $( flat_variants!(@cls $cls, $name $({ $($f: $t),+ })?); )+
        flat_variants!(@enum $enum { $($variant => $cls),+ });
    };
}

/// A unit pyclass enum mirroring an internal enum 1:1, plus its `From` impl, from one table.
///
/// ```ignore
/// mirror_enum!(PyCardName from CardName, "CardName", {
///     AThousandCuts, Accuracy, /* one ident per variant, internal declaration order */
/// });
/// ```
///
/// Table order = declaration order = int discriminant
/// stays a compile error. Mirrored enums are snapshot-only, so the pyclass is
/// always `skip_from_py_object`. Call sites need `use pyo3::prelude::*` in scope.
macro_rules! mirror_enum {
    ($py:ident from $internal:ident, $name:literal, { $($v:ident),+ $(,)? }) => {
        mirror_enum!($py from $internal, $name, { $($v),+ }, {});
    };
    ($py:ident from $internal:ident, $name:literal, { $($v:ident),+ $(,)? }, { $($extra:item)* }) => {
        #[pyclass(skip_from_py_object, eq, eq_int, frozen, name = $name, module = "slai.slai")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $py { $($v,)+ }

        impl From<$internal> for $py {
            fn from(v: $internal) -> Self {
                match v {
                    $($internal::$v => Self::$v,)+
                }
            }
        }

        #[pymethods]
        impl $py {
            // Declaration order, which is also int() order
            #[staticmethod]
            fn members() -> Vec<$py> {
                vec![$($py::$v),+]
            }

            // Hash by raw discriminant so eq_int and hash agree; the derived hash
            // breaks Python's eq/hash contract for int-keyed dicts
            fn __hash__(&self) -> isize {
                *self as isize
            }

            $($extra)*
        }
    };
}

pub(crate) use flat_variants;
pub(crate) use mirror_enum;
