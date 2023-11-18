//! # Dysql-tpl
//!
//! Fast [**Mustache**](https://mustache.github.io/) template engine implementation
//! in pure Rust.
//!
//! **Dysql-tpl** loads and processes templates **at runtime**. It comes with a derive macro
//! which allows for templates to be rendered from native Rust data structures without doing
//! temporary allocations, intermediate `HashMap`s or what have you.
//!
//! With a touch of magic 🎩, the power of friendship 🥂, and a sparkle of
//! [FNV hashing](https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function)
//! ✨, render times easily compete with static template engines like
//! [**Askama**](https://github.com/djc/askama).
//!
//! What else do you want, a sticker?
//!
//! ## Example
//!
//! ```ignore
//! use dysql_tpl::{Template, Content};
//!
//! #[derive(Content)]
//! struct Post<'a> {
//!     title: &'a str,
//!     teaser: &'a str,
//! }
//!
//! #[derive(Content)]
//! struct Blog<'a> {
//!     title: String,        // Strings are cool
//!     posts: Vec<Post<'a>>, // &'a [Post<'a>] would work too
//! }
//!
//! // Standard Mustache action here
//! let source = "<h1>{{title}}</h1>\
//!               {{#posts}}<article><h2>{{title}}</h2><p>{{teaser}}</p></article>{{/posts}}\
//!               {{^posts}}<p>No posts yet :(</p>{{/posts}}";
//!
//! let tpl = Template::new(source).unwrap();
//!
//! let rendered = tpl.render(&Blog {
//!     title: "My Awesome Blog!".to_string(),
//!     posts: vec![
//!         Post {
//!             title: "How I tried Ramhorns and found love 💖",
//!             teaser: "This can happen to you too",
//!         },
//!         Post {
//!             title: "Rust is kinda awesome",
//!             teaser: "Yes, even the borrow checker! 🦀",
//!         },
//!     ]
//! });
//!
//! assert_eq!(rendered, "<h1>My Awesome Blog!</h1>\
//!                       <article>\
//!                           <h2>How I tried Ramhorns and found love 💖</h2>\
//!                           <p>This can happen to you too</p>\
//!                       </article>\
//!                       <article>\
//!                           <h2>Rust is kinda awesome</h2>\
//!                           <p>Yes, even the borrow checker! 🦀</p>\
//!                       </article>");
//! ```

#![allow(missing_docs)]
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

use std::io::ErrorKind;

mod content;
mod error;
mod template;
mod simple;
pub mod traits;

pub mod encoding;

pub use content::Content;
pub use error::TemplateError;
use fnv::FnvHasher;
pub use template::{Section, Template};
pub use simple::{SimpleTemplate, SimpleSection, SimpleValue, SimpleError, SimpleInnerError};

#[cfg(feature = "export_derive")]
pub use dysql_tpl_derive::Content;
use traits::Combine;

/// Necessary so that the warning of very complex type created when compiling
/// with `cargo clippy` doesn't propagate to downstream crates
type Next<'section, C, X> = (<C as Combine>::I, <C as Combine>::J, <C as Combine>::K, X);

#[inline]
pub(crate) fn hash_name(name: &str) -> u64 {
    let mut hasher = FnvHasher::default();
    name.hash(&mut hasher);
    hasher.finish()
}

/// Aggregator for [`Template`s](./struct.Template.html), that allows them to
/// be loaded from the file system and use partials: `{{>partial}}`
///
/// For faster or DOS-resistant hashes, it is recommended to use
/// [aHash](https://docs.rs/ahash/latest/ahash/) `RandomState` as hasher.
pub struct Ramhorns<H = fnv::FnvBuildHasher> {
    partials: HashMap<String, Template, H>,
    dir: PathBuf,
}

impl<H: BuildHasher + Default> Ramhorns<H> {
    /// Loads all the `.html` files as templates from the given folder, making them
    /// accessible via their path, joining partials as required. If a custom
    /// extension is wanted, see [from_folder_with_extension]
    /// ```no_run
    /// # use dysql_tpl::Ramhorns;
    /// let tpls: Ramhorns = Ramhorns::from_folder("./templates").unwrap();
    /// let content = "I am the content";
    /// let rendered = tpls.get("hello.html").unwrap().render(&content);
    /// ```
    pub fn from_folder<P: AsRef<Path>>(dir: P) -> Result<Self, TemplateError> {
        Self::from_folder_with_extension(dir, "html")
    }

    /// Loads all files with the extension given in the `extension` parameter as templates
    /// from the given folder, making them accessible via their path, joining partials as
    /// required.
    /// ```no_run
    /// # use dysql_tpl::Ramhorns;
    /// let tpls: Ramhorns = Ramhorns::from_folder_with_extension("./templates", "mustache").unwrap();
    /// let content = "I am the content";
    /// let rendered = tpls.get("hello.mustache").unwrap().render(&content);
    /// ```
    #[inline]
    pub fn from_folder_with_extension<P: AsRef<Path>>(
        dir: P,
        extension: &str,
    ) -> Result<Self, TemplateError> {
        let mut templates = Ramhorns::lazy(dir)?;
        templates.load_folder(&templates.dir.clone(), extension)?;

        Ok(templates)
    }

    /// Extends the template collection with files with `.html` extension
    /// from the given folder, making them accessible via their path, joining partials as
    /// required.
    /// If there is a file with the same name as a  previously loaded template or partial,
    /// it will not be loaded.
    pub fn extend_from_folder<P: AsRef<Path>>(&mut self, dir: P) -> Result<(), TemplateError> {
        self.extend_from_folder_with_extension(dir, "html")
    }

    /// Extends the template collection with files with `extension`
    /// from the given folder, making them accessible via their path, joining partials as
    /// required.
    /// If there is a file with the same name as a  previously loaded template or partial,
    /// it will not be loaded.
    #[inline]
    pub fn extend_from_folder_with_extension<P: AsRef<Path>>(
        &mut self,
        dir: P,
        extension: &str,
    ) -> Result<(), TemplateError> {
        let dir = std::mem::replace(&mut self.dir, dir.as_ref().canonicalize()?);
        self.load_folder(&self.dir.clone(), extension)?;
        self.dir = dir;

        Ok(())
    }

    fn load_folder(&mut self, dir: &Path, extension: &str) -> Result<(), TemplateError> {
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                self.load_folder(&path, extension)?;
            } else if path.extension().map(|e| e == extension).unwrap_or(false) {
                let name = path
                    .strip_prefix(&self.dir)
                    .unwrap_or(&path)
                    .to_string_lossy();
                if !self.partials.contains_key(name.as_ref()) {
                    self.load_internal(&path, &name)?;
                }
            }
        }
        Ok(())
    }

    /// Create a new empty aggregator for a given folder. This won't do anything until
    /// a template has been added using [`from_file`](#method.from_file).
    /// ```no_run
    /// # use dysql_tpl::Ramhorns;
    /// let mut tpls: Ramhorns = Ramhorns::lazy("./templates").unwrap();
    /// let content = "I am the content";
    /// let rendered = tpls.from_file("hello.html").unwrap().render(&content);
    /// ```
    pub fn lazy<P: AsRef<Path>>(dir: P) -> Result<Self, TemplateError> {
        Ok(Ramhorns {
            partials: HashMap::default(),
            dir: dir.as_ref().canonicalize()?,
        })
    }

    /// Get the template with the given name, if it exists.
    pub fn get(&self, name: &str) -> Option<&Template>
    {
        self.partials.get(name)
    }

    /// Get the template with the given name. If the template doesn't exist,
    /// it will be loaded from file and parsed first.
    ///
    /// Use this method in tandem with [`lazy`](#method.lazy).
    pub fn from_file(&mut self, name: &str) -> Result<&Template, TemplateError> {
        let path = self.dir.join(name);
        if !self.partials.contains_key(name) {
            self.load_internal(&path, name)?;
        }
        Ok(&self.partials[name])
    }

    // Unsafe to expose as it loads the template from arbitrary path.
    #[inline]
    fn load_internal(&mut self, path: &Path, name: &str) -> Result<(), TemplateError> {
        let file = match std::fs::read_to_string(&path) {
            Ok(file) => Ok(file),
            Err(e) if e.kind() == ErrorKind::NotFound => {
                Err(TemplateError::NotFound(name.to_string().into()))
            }
            Err(e) => Err(TemplateError::Io(e)),
        }?;
        let template = Template::load(&file, self)?;
        self.partials.insert(name.to_owned(), template);
        Ok(())
    }
}

pub(crate) trait Partials {
    fn get_partial(&mut self, name: &str) -> Result<&Template, TemplateError>;
}

impl<H: BuildHasher + Default> Partials for Ramhorns<H> {
    fn get_partial(&mut self, name: &str) -> Result<&Template, TemplateError> {
        if !self.partials.contains_key(name) {
            let path = self.dir.join(name).canonicalize()?;
            if !path.starts_with(&self.dir) {
                return Err(TemplateError::IllegalPartial(name.into()));
            }
            self.load_internal(&path, name)?;
        }
        Ok(&self.partials[name])
    }
}
