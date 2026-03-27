use crate::core::{Config, RenderContext};

/// Represents any Lua primitive or composite value in a type-safe way.
/// This is the serialization backbone used by all Config nodes.
#[derive(Debug, Clone)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Str(String),
    Raw(String), // Arbitrary Lua expression (function, reference, etc.)
    List(Vec<LuaValue>),
    Table(Vec<(LuaKey, LuaValue)>),
}

/// Keys in a Lua table can be strings (quoted) or identifiers (bare).
#[derive(Debug, Clone)]
pub enum LuaKey {
    Ident(String),  // rendered as: key = value
    Quoted(String), // rendered as: ['key'] = value
}

impl LuaValue {
    /// Serialize the value to a Lua string, with proper indentation.
    pub fn to_lua(&self, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        let inner_pad = "  ".repeat(indent + 1);
        match self {
            LuaValue::Nil => "nil".to_string(),
            LuaValue::Boolean(b) => b.to_string(),
            LuaValue::Integer(n) => n.to_string(),
            LuaValue::Float(f) => format!("{}", f),
            LuaValue::Str(s) => format!("'{}'", s.replace('\'', "\\'")),
            LuaValue::Raw(s) => s.clone(),
            LuaValue::List(items) => {
                if items.is_empty() {
                    return "{}".to_string();
                }
                let elems: Vec<String> = items
                    .iter()
                    .map(|v| format!("{}{}", inner_pad, v.to_lua(indent + 1)))
                    .collect();
                format!("{{\n{}\n{}}}", elems.join(",\n"), pad)
            }
            LuaValue::Table(pairs) => {
                if pairs.is_empty() {
                    return "{}".to_string();
                }
                let elems: Vec<String> = pairs
                    .iter()
                    .map(|(k, v)| {
                        let key_str = match k {
                            LuaKey::Ident(s) => s.clone(),
                            LuaKey::Quoted(s) => format!("['{}']", s),
                        };
                        format!("{}{} = {}", inner_pad, key_str, v.to_lua(indent + 1))
                    })
                    .collect();
                format!("{{\n{}\n{}}}", elems.join(",\n"), pad)
            }
        }
    }
}

/// Constructor helpers
impl LuaValue {
    pub fn str(s: &str) -> Self {
        LuaValue::Str(s.to_string())
    }
    pub fn int(n: i64) -> Self {
        LuaValue::Integer(n)
    }
    pub fn float(f: f64) -> Self {
        LuaValue::Float(f)
    }
    pub fn bool(b: bool) -> Self {
        LuaValue::Boolean(b)
    }
    pub fn raw(s: &str) -> Self {
        LuaValue::Raw(s.to_string())
    }
    pub fn list(items: Vec<LuaValue>) -> Self {
        LuaValue::List(items)
    }
    pub fn table(pairs: Vec<(&str, LuaValue)>) -> Self {
        LuaValue::Table(
            pairs
                .into_iter()
                .map(|(k, v)| (LuaKey::Ident(k.to_string()), v))
                .collect(),
        )
    }
}

/// An escape-hatch struct wrapping arbitrary Lua code as a Config node.
/// Use this for any Neovim feature not yet modeled by the library.
pub struct RawLua {
    pub code: String,
    pub doc: Option<String>,
}

impl RawLua {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
            doc: None,
        }
    }

    pub fn with_doc(mut self, doc: &str) -> Self {
        self.doc = Some(doc.to_string());
        self
    }
}

impl Config for RawLua {
    fn render(&self, ctx: &RenderContext) -> String {
        let indent = ctx.indent();
        self.code
            .lines()
            .map(|l| format!("{}{}", indent, l))
            .collect::<Vec<_>>()
            .join("\n")
    }
    fn doc_comment(&self) -> Option<&str> {
        self.doc.as_deref()
    }
}
