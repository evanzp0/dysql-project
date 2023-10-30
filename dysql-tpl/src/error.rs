// Ramhorns  Copyright (C) 2019  Maciej Hirsz
//
// This file is part of Ramhorns. This program comes with ABSOLUTELY NO WARRANTY;
// This is free software, and you are welcome to redistribute it under the
// conditions of the GNU General Public License version 3.0.
//
// You should have received a copy of the GNU General Public License
// along with Ramhorns.  If not, see <http://www.gnu.org/licenses/>

use std::{error, fmt, io};

/// Error type used that can be emitted during template parsing.
#[derive(Debug)]
pub enum TemplateError {
    /// There was an error with the IO (only happens when parsing a file)
    Io(io::Error),

    /// Stack overflow when parsing nested sections
    StackOverflow,

    /// Parser was expecting a tag closing a section `{{/foo}}`,
    /// but never found it or found a different one.
    UnclosedSection(Box<str>),

    /// Similar to above, but happens if `{{/foo}}` happens while
    /// no section was open
    UnopenedSection(Box<str>),

    /// Parser was expecting to find the closing braces of a tag `}}`, but never found it.
    UnclosedTag,

    /// Partials are not allowed in the given context (e.g. parsing a template from string)
    PartialsDisabled,

    /// Attempted to load a partial outside of the templates folder
    IllegalPartial(Box<str>),

    /// The template file with the given name was not found
    NotFound(Box<str>),
}

impl error::Error for TemplateError {}

impl From<io::Error> for TemplateError {
    fn from(err: io::Error) -> Self {
        TemplateError::Io(err)
    }
}

impl<T> From<arrayvec::CapacityError<T>> for TemplateError {
    fn from(_: arrayvec::CapacityError<T>) -> Self {
        TemplateError::StackOverflow
    }
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemplateError::Io(err) => err.fmt(f),
            TemplateError::StackOverflow => write!(
                f,
                "Ramhorns has overflown its stack when parsing nested sections",
            ),
            TemplateError::UnclosedSection(name) => write!(
                f,
                "Section not closed properly, was expecting {{{{/{}}}}}",
                name
            ),
            TemplateError::UnopenedSection(name) => {
                write!(f, "Unexpected closing section {{{{/{}}}}}", name)
            }
            TemplateError::UnclosedTag => write!(f, "Couldn't find closing braces matching opening braces"),
            TemplateError::PartialsDisabled => write!(f, "Partials are not allowed in the current context"),
            TemplateError::IllegalPartial(name) => write!(
                f,
                "Attempted to load {}; partials can only be loaded from the template directory",
                name
            ),
            TemplateError::NotFound(name) => write!(f, "Template file {} not found", name),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn displays_properly() {
        assert_eq!(
            TemplateError::UnclosedSection("foo".into()).to_string(),
            "Section not closed properly, was expecting {{/foo}}"
        );
        assert_eq!(
            TemplateError::UnclosedTag.to_string(),
            "Couldn't find closing braces matching opening braces"
        );
    }
}
