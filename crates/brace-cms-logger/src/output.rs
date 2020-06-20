use std::fmt::{self, Display};
use std::path::PathBuf;

use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

#[derive(Clone, Debug, PartialEq)]
pub enum Output {
    Stdout,
    Stderr,
    File(PathBuf),
}

impl Default for Output {
    fn default() -> Self {
        Self::Stdout
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Stdout => write!(f, "stdout"),
            Self::Stderr => write!(f, "stderr"),
            Self::File(file) => write!(f, "{:?}", file),
        }
    }
}

impl Serialize for Output {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Stdout => serializer.serialize_unit_variant("Output", 0, "stdout"),
            Self::Stderr => serializer.serialize_unit_variant("Output", 1, "stderr"),
            Self::File(file) => serializer.serialize_str(&format!("{}", file.display())),
        }
    }
}

impl<'de> Deserialize<'de> for Output {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct OutputVisitor;

        impl<'de> Visitor<'de> for OutputVisitor {
            type Value = Output;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("stdout, stderr or file path")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
                match value {
                    "stdout" => Ok(Output::Stdout),
                    "stderr" => Ok(Output::Stderr),
                    file => Ok(Output::File(file.into())),
                }
            }
        }

        deserializer.deserialize_str(OutputVisitor)
    }
}
