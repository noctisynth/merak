use utoipa::openapi::{KnownFormat, ObjectBuilder, RefOr, Schema, SchemaFormat, schema::Type};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)]
#[serde(transparent)]
pub struct BusinessCode(pub i32);

pub const CODE_OK: BusinessCode = BusinessCode(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum Category {
    Success = 0,
    BusinessError = 1,
    Processing = 2,
    PartialSuccess = 3,
    UnknownError = 9,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum Module {
    Auth = 1,
    User = 2,
    Org = 3,
    Project = 4,
    Space = 5,
    Workflow = 6,
    Node = 7,
    Subtask = 8,
    Link = 9,
    Doc = 10,
    Comment = 11,
    Notification = 12,
    Common = 99,
}

/// Build a business code using the CMMRR scheme.
/// C: category, MM: module, RR: reason.
pub const fn make_code(category: Category, module: Module, reason: i32) -> BusinessCode {
    BusinessCode((category as i32 * 10000) + (module as i32 * 100) + reason)
}

macro_rules! define_codes {
    ($enum_name:ident, $cat:expr, $mod_:expr, {
        $($(#[doc = $desc:literal])* $variant:ident = $reason:expr),* $(,)?
    }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(i32)]
        pub enum $enum_name {
            $($variant = $crate::common::code::make_code($cat, $mod_, $reason).0),*
        }

        impl From<$enum_name> for $crate::common::code::BusinessCode {
            fn from(v: $enum_name) -> Self {
                $crate::common::code::BusinessCode(v as i32)
            }
        }

        impl $enum_name {
            pub fn schema_items() -> Vec<utoipa::openapi::RefOr<utoipa::openapi::Schema>> {
                vec![$(
                    utoipa::openapi::ObjectBuilder::new()
                        .title(Some(concat!($($desc),*).trim()))
                        .schema_type(utoipa::openapi::schema::Type::Integer)
                        .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                            utoipa::openapi::KnownFormat::Int32,
                        )))
                        .enum_values(Some([$crate::common::code::make_code($cat, $mod_, $reason).0]))
                        .description(Some(concat!($($desc),*).trim()))
                        .into(),
                )*]
            }

            pub fn code_entries() -> Vec<(i32, &'static str)> {
                vec![$(
                    ($crate::common::code::make_code($cat, $mod_, $reason).0, concat!($($desc),*).trim()),
                )*]
            }
        }

        impl utoipa::PartialSchema for $enum_name {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
                let values = vec![$($crate::common::code::make_code($cat, $mod_, $reason).0),*];
                let descs: Vec<&str> = vec![$(concat!($($desc),*).trim()),*];
                let description = values.iter().zip(descs.iter())
                    .map(|(v, d)| format!("- `{}` — {}", v, d))
                    .collect::<Vec<_>>()
                    .join("\n");

                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int32,
                    )))
                    .enum_values(Some(values))
                    .description(Some(description))
                    .into()
            }
        }

        impl utoipa::ToSchema for $enum_name {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(stringify!($enum_name))
            }
        }

        inventory::submit!($crate::common::code::CodeSchemaEntry {
            name: <$enum_name as utoipa::ToSchema>::name,
            schema: <$enum_name as utoipa::PartialSchema>::schema,
        });
    };
}

define_codes!(CommonCode, Category::BusinessError, Module::Common, {
    /// Resource not found
    NotFound = 1,
});

pub struct SuccessCode;

impl utoipa::PartialSchema for SuccessCode {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .schema_type(Type::Integer)
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int32)))
            .enum_values(Some([0]))
            .description(Some("Success"))
            .into()
    }
}

impl utoipa::ToSchema for SuccessCode {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("SuccessCode")
    }
}

macro_rules! combine_codes {
    ($name:ident, [$($code_type:ty),+ $(,)?]) => {
        pub struct $name;

        impl utoipa::PartialSchema for $name {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::Schema> {
                let mut entries = Vec::new();
                $(entries.extend(<$code_type>::code_entries());)+

                let values: Vec<i32> = entries.iter().map(|(v, _)| *v).collect();
                let description = entries.iter()
                    .map(|(v, d)| format!("- `{}` — {}", v, d))
                    .collect::<Vec<_>>()
                    .join("\n");

                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int32,
                    )))
                    .enum_values(Some(values))
                    .description(Some(description))
                    .into()
            }
        }

        impl utoipa::ToSchema for $name {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(stringify!($name))
            }
        }

        inventory::submit!($crate::common::code::CodeSchemaEntry {
            name: <$name as utoipa::ToSchema>::name,
            schema: <$name as utoipa::PartialSchema>::schema,
        });
    };
}

pub struct CodeSchemaEntry {
    pub name: fn() -> std::borrow::Cow<'static, str>,
    pub schema: fn() -> RefOr<Schema>,
}

inventory::collect!(CodeSchemaEntry);

pub fn register_all_codes(api: &mut utoipa::openapi::OpenApi) {
    let components = api.components.get_or_insert_with(Default::default);
    for entry in inventory::iter::<CodeSchemaEntry> {
        components
            .schemas
            .insert((entry.name)().to_string(), (entry.schema)());
    }
}

pub(crate) use combine_codes;
pub(crate) use define_codes;
