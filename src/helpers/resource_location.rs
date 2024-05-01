use java_string::{JavaStr, JavaString};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ResourceLocation {
    pub(crate) namespace: JavaString,
    pub(crate) path: JavaString,
}

impl ResourceLocation {
    pub(crate) fn new(namespace: impl Into<JavaString>, path: impl Into<JavaString>) -> Self {
        Self {
            namespace: namespace.into(),
            path: path.into(),
        }
    }

    pub(crate) fn minecraft(path: impl Into<JavaString>) -> Self {
        Self::new("minecraft", path)
    }

    pub(crate) fn parse(s: &JavaStr) -> Result<Self, ResourceLocationError> {
        Self::parse_with_separator(s, ':')
    }

    pub(crate) fn parse_with_separator(
        s: &JavaStr,
        sep: char,
    ) -> Result<Self, ResourceLocationError> {
        if let Some(index) = s.find(sep) {
            let (namespace, path) = s.split_at(index);
            let path = &path[1..];
            Self::validate_namespace(namespace)?;
            Self::validate_path(path)?;
            Ok(Self::new(namespace, path))
        } else {
            Self::validate_path(s)?;
            Ok(Self::new("minecraft", s))
        }
    }

    fn validate_namespace(namespace: &JavaStr) -> Result<(), ResourceLocationError> {
        if namespace.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.' || c == '-'
        }) {
            Ok(())
        } else {
            Err(ResourceLocationError::Namespace {
                namespace: namespace.to_owned(),
            })
        }
    }

    fn validate_path(path: &JavaStr) -> Result<(), ResourceLocationError> {
        if path.chars().all(|c| {
            c.is_ascii_lowercase()
                || c.is_ascii_digit()
                || c == '/'
                || c == '_'
                || c == '.'
                || c == '-'
        }) {
            Ok(())
        } else {
            Err(ResourceLocationError::Path {
                path: path.to_owned(),
            })
        }
    }

    pub fn to_java_string(&self) -> JavaString {
        let mut result = self.namespace.clone();
        result.push(':');
        result.push_java_str(&self.path);
        result
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ResourceLocationError {
    Namespace { namespace: JavaString },
    Path { path: JavaString },
}

impl Display for ResourceLocationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceLocationError::Namespace { namespace } => write!(
                f,
                "Non [a-z0-9_.-] character in resource location namespace {namespace}"
            ),
            ResourceLocationError::Path { path } => write!(
                f,
                "Non [a-z0-9/_.-] character in resource location path {path}"
            ),
        }
    }
}

impl std::error::Error for ResourceLocationError {}
