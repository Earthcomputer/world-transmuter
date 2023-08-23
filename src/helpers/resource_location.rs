use nom::Slice;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ResourceLocation {
    pub(crate) namespace: String,
    pub(crate) path: String,
}

impl ResourceLocation {
    pub(crate) fn new(namespace: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            path: path.into(),
        }
    }

    pub(crate) fn parse_with_separator(s: &str, sep: char) -> Result<Self, String> {
        if let Some(index) = s.find(sep) {
            let (namespace, path) = s.split_at(index);
            let path = path.slice(1..);
            Self::validate_namespace(namespace)?;
            Self::validate_path(path)?;
            Ok(Self::new(namespace, path))
        } else {
            Self::validate_path(s)?;
            Ok(Self::new("minecraft", s))
        }
    }

    fn validate_namespace(namespace: &str) -> Result<(), String> {
        if namespace
            .chars()
            .all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-'))
        {
            Ok(())
        } else {
            Err(format!(
                "Non [a-z0-9_.-] character in resource location namespace {}",
                namespace
            ))
        }
    }

    fn validate_path(path: &str) -> Result<(), String> {
        if path
            .chars()
            .all(|c| matches!(c, 'a'..='z' | '0'..='9' | '/' | '_' | '.' | '-'))
        {
            Ok(())
        } else {
            Err(format!(
                "Non [a-z0-9/_.-] character in resource location path {}",
                path
            ))
        }
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl FromStr for ResourceLocation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_with_separator(s, ':')
    }
}
