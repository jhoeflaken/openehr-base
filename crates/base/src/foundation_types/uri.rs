mod authority;
mod characters;
mod context;
mod codec;
mod error;
mod decoder;
mod port;
mod ipv6_address;
mod ipv4_address;
mod absolute_uri;

use authority::Authority;
use characters::{ALPHA, PCHAR_NOT_PCT_ENCODED, QUERY_NOT_PCT_ENCODED_WITHOUT_PLUS,
                  QUERY_OR_FRAGMENT_NOT_PCT_ENCODED, SCHEME_NOT_FIRST};
use codec::{decode_element, encode_element};
use context::Context;
use error::Error;
use absolute_uri::AbsoluteUri;
use std::{collections::HashSet, convert::TryFrom, ops::Deref};
use std::{fmt::Write, string::FromUtf8Error};
use std::cmp::Ordering;
use crate::foundation_types::{Any, Ordered};

/// This type is used to parse and generate URI strings to and from their
/// various components.  Components are percent-encoded as necessary during
/// generation, and percent encodings are decoded during parsing.
///
/// Since most URI components, once decoded, may include non-UTF8 byte
/// sequences (which are always percent-encoded), getter methods such as
/// [`path`] and [`query`] return byte array [slice] references (`&[u8]`)
/// rather than string or string slice references.  Fallible convenience
/// methods ending in `_to_string`, such as [`path_to_string`] and
/// [`query_to_string`], are provided to convert these to strings.
///
/// The "Authority" part of the Uri is represented by the [`Authority` type].
/// Although the `Uri` type provides [`user_info`], [`host`], and [`port`]
/// methods for convenience, `Uri` holds these components through the
/// [`Authority` type], which can be accessed via [`authority`] and
/// [`set_authority`].  To set or change the `user_info`, host, or port of a
/// `Uri`, construct a new `Authority` value and set it in the `Uri` with
/// [`set_authority`].
///
/// # Examples
///
/// ## Parsing a URI into its components
///
/// ```rust
/// use uniresid::Uri;
///
/// # fn main() {
/// let uri = Uri::parse("http://www.example.com/foo?bar#baz").unwrap();
/// let authority = uri.authority().unwrap();
/// assert_eq!("www.example.com".as_bytes(), authority.host());
/// assert_eq!(Some("www.example.com"), uri.host_to_string().unwrap().as_deref());
/// assert_eq!("/foo", uri.path_to_string().unwrap());
/// assert_eq!(Some("bar"), uri.query_to_string().unwrap().as_deref());
/// assert_eq!(Some("baz"), uri.fragment_to_string().unwrap().as_deref());
/// # }
/// ```
///
/// Implementations are provided for the [`TryFrom`] trait, so that
/// [`TryFrom::try_from`] or [`TryInto::try_into`] may be used as alternatives
/// to [`parse`].
///
/// ## Generating a URI from its components
///
/// ```rust
/// use uniresid::{ Authority, Uri };
///
/// let mut uri = Uri::default();
/// assert!(uri.set_scheme(String::from("http")).is_ok());
/// let mut authority = Authority::default();
/// authority.set_host("www.example.com");
/// uri.set_authority(Some(authority));
/// uri.set_path_from_str("/foo");
/// uri.set_query(Some("bar".into()));
/// uri.set_fragment(Some("baz".into()));
/// assert_eq!("http://www.example.com/foo?bar#baz", uri.to_string());
/// ```
///
/// [`authority`]: #method.authority
/// [`Authority` type]: struct.Authority.html
/// [`host`]: #method.host
/// [`parse`]: #method.parse
/// [`path`]: #method.path
/// [`path_to_string`]: #method.path_to_string
/// [`port`]: #method.port
/// [`query`]: #method.query
/// [`query_to_string`]: #method.query_to_string
/// [`set_authority`]: #method.set_authority
/// [`user_info`]: #method.user_info
/// [slice]: https://doc.rust-lang.org/std/primitive.slice.html
/// [`TryFrom::try_from`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html#tymethod.try_from
/// [`TryInto::try_into`]: https://doc.rust-lang.org/std/convert/trait.TryInto.html#tymethod.try_into
#[derive(Clone, Default, Hash, PartialEq, Eq)]
pub struct Uri {
    scheme: Option<String>,
    authority: Option<Authority>,
    path: Vec<Vec<u8>>,
    query: Option<Vec<u8>>,
    fragment: Option<Vec<u8>>,
    raw: String,
}

impl Uri {
    /// Borrow the authority (if any) of the URI.
    #[must_use = "authority not used"]
    pub fn authority(&self) -> Option<&Authority> {
        self.authority.as_ref()
    }

    /// Determines if the URI contains a relative path rather than an absolute
    /// path.
    #[must_use]
    pub fn contains_relative_path(&self) -> bool {
        !Self::is_path_absolute(&self.path)
    }

    /// Borrow the fragment (if any) of the URI.
    #[must_use]
    pub fn fragment(&self) -> Option<&[u8]> {
        self.fragment.as_deref()
    }

    /// Convert the fragment (if any) into a string.
    ///
    /// # Errors
    ///
    /// Since fragments may contain non-UTF8 byte sequences, this function may
    /// return [`Error::CannotExpressAsUtf8`][CannotExpressAsUtf8].
    ///
    /// [CannotExpressAsUtf8]: enum.Error.html#variant.CannotExpressAsUtf8
    pub fn fragment_to_string(&self) -> Result<Option<String>, FromUtf8Error> {
        self.fragment()
            .map(|fragment| String::from_utf8(fragment.to_vec()).map_err(Into::into))
            .transpose()
    }

    /// Borrow the host portion of the Authority (if any) of the URI.
    #[must_use]
    pub fn host(&self) -> Option<&[u8]> {
        self.authority.as_ref().map(Authority::host)
    }

    /// Convert the host portion of the Authority (if any) into a string.
    ///
    /// # Errors
    ///
    /// Since host names may contain non-UTF8 byte sequences, this function may
    /// return [`Error::CannotExpressAsUtf8`][CannotExpressAsUtf8].
    ///
    /// [CannotExpressAsUtf8]: enum.Error.html#variant.CannotExpressAsUtf8
    pub fn host_to_string(&self) -> Result<Option<String>, FromUtf8Error> {
        self.host()
            .map(|host| String::from_utf8(host.to_vec()))
            .transpose()
    }

    /// Determines if the URI is a `relative-ref` (relative reference), as
    /// defined in [RFC 3986 section
    /// 4.2](https://tools.ietf.org/html/rfc3986#section-4.2).  A relative
    /// reference has no scheme, but may still have an authority.
    #[must_use]
    pub fn is_relative_reference(&self) -> bool {
        self.scheme.is_none()
    }

    /// Apply the `remove_dot_segments` routine talked about
    /// in [RFC 3986 section
    /// 5.2](https://tools.ietf.org/html/rfc3986#section-5.2) to the path
    /// segments of the URI, in order to normalize the path (apply and remove
    /// "." and ".." segments).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uniresid::Uri;
    ///
    /// # fn main() {
    /// let mut uri = Uri::parse("/a/b/c/./../../g").unwrap();
    /// uri.normalize();
    /// assert_eq!("/a/g", uri.path_to_string().unwrap());
    /// # }
    /// ```
    pub fn normalize(&mut self) {
        self.path = Self::normalize_path(&self.path);
        self.update_raw();
    }

    /// Interpret the given string as a URI, separating its various components,
    /// returning a `Uri` value containing them.
    ///
    /// # Errors
    ///
    /// There are many ways to screw up a URI string, and this function will
    /// let you know what's up by returning a variant of the
    /// [`Error`](enum.Error.html) type.
    pub fn parse<T>(uri_string: T) -> Result<Self, Error>
    where
        T: AsRef<str>,
    {
        let s = uri_string.as_ref();
        let (scheme, rest) = Self::parse_scheme(s)?;
        let path_end = rest.find(&['?', '#'][..]).unwrap_or(rest.len());
        let authority_and_path_string = &rest[0..path_end];
        let query_and_or_fragment = &rest[path_end..];
        let (authority, path) =
            Self::split_authority_from_path_and_parse_them(authority_and_path_string)?;
        let (fragment, possible_query) = Self::parse_fragment(query_and_or_fragment)?;
        let query = Self::parse_query(possible_query)?;
        let mut this = Self {
            scheme,
            authority,
            path,
            query,
            fragment,
            raw: String::default(),
        };
        this.update_raw();
        Ok(this)
    }

    /// Borrow the path component of the URI.
    ///
    /// The path is represented as a two-dimensional vector:
    /// * the "segments" or pieces of the path between the slashes
    /// * the bytes that make up each segment
    ///
    /// Byte vectors are used instead of strings because segments may contain
    /// non-UTF8 sequences.
    ///
    /// Leading and trailing slashes in the path are special cases represented
    /// by extra empty segments at the beginning and/or end of the path.
    ///
    /// # Examples
    ///
    /// (Note: the examples below show strings, not byte vectors, simply to be
    /// more readable.)
    ///
    /// ```text
    /// "foo/bar"   -> ["foo", "bar"]
    /// "/foo/bar"  -> ["", "foo", "bar"]
    /// "foo/bar/"  -> ["foo", "bar", ""]
    /// "/foo/bar/" -> ["", "foo", "bar", ""]
    /// "/"         -> [""]
    /// ""          -> []
    /// ```
    #[must_use]
    pub fn path(&self) -> &Vec<Vec<u8>> {
        &self.path
    }

    /// Convert the path portion of the URI into a string.
    ///
    /// # Errors
    ///
    /// Since path segments may contain non-UTF8 byte sequences, this function
    /// may return
    /// [`Error::CannotExpressAsUtf8`][CannotExpressAsUtf8].
    ///
    /// [CannotExpressAsUtf8]: enum.Error.html#variant.CannotExpressAsUtf8
    pub fn path_to_string(&self) -> Result<String, FromUtf8Error> {
        match &*self.path {
            [segment] if segment.is_empty() => Ok("/".to_string()),
            path => Ok(String::from_utf8(path.join(&b"/"[..]))?),
        }
    }

    /// Return a copy of the port (if any) contained in the URI.
    pub fn port(&self) -> Option<u16> {
        self.authority.as_ref().and_then(Authority::port)
    }

    /// Borrow the query (if any) of the URI.
    #[must_use]
    pub fn query(&self) -> Option<&[u8]> {
        self.query.as_deref()
    }

    /// Convert the query (if any) into a string.
    ///
    /// # Errors
    ///
    /// Since queries may contain non-UTF8 byte sequences, this function may
    /// return [`Error::CannotExpressAsUtf8`][CannotExpressAsUtf8].
    ///
    /// [CannotExpressAsUtf8]: enum.Error.html#variant.CannotExpressAsUtf8
    pub fn query_to_string(&self) -> Result<Option<String>, FromUtf8Error> {
        self.query()
            .map(|query| String::from_utf8(query.to_vec()))
            .transpose()
    }

    /// Return a new URI which is the result of applying the given relative
    /// reference to the URI, following the algorithm from [RFC 3986 section
    /// 5.2.2](https://tools.ietf.org/html/rfc3986#section-5.2.2).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uniresid::Uri;
    ///
    /// # fn main() -> Result<(), uniresid::Error> {
    /// let base = Uri::parse("http://a/b/c/d;p?q")?;
    /// let relative_reference = Uri::parse("g;x?y#s")?;
    /// let resolved = base.resolve(&relative_reference);
    /// assert_eq!("http://a/b/c/g;x?y#s", resolved.to_string());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn resolve(&self, relative_reference: &Self) -> Self {
        let (scheme, authority, path, query) = if relative_reference.scheme.is_some() {
            (
                relative_reference.scheme.clone(),
                relative_reference.authority.clone(),
                Self::normalize_path(&relative_reference.path),
                relative_reference.query.clone(),
            )
        } else {
            relative_reference.authority.as_ref().map_or_else(
                || {
                    let scheme = self.scheme.clone();
                    let authority = self.authority.clone();
                    if relative_reference.path.is_empty() {
                        let path = self.path.clone();
                        let query = if relative_reference.query.is_none() {
                            self.query.clone()
                        } else {
                            relative_reference.query.clone()
                        };
                        (scheme, authority, path, query)
                    } else {
                        let query = relative_reference.query.clone();

                        // RFC describes this as:
                        // "if (R.path starts-with "/") then"
                        if Self::is_path_absolute(&relative_reference.path) {
                            (scheme, authority, relative_reference.path.clone(), query)
                        } else {
                            // RFC describes this as:
                            // "T.path = merge(Base.path, R.path);"
                            let mut path = self.path.clone();
                            if self.authority.is_none() || path.len() > 1 {
                                path.pop();
                            }
                            path.extend(relative_reference.path.iter().cloned());
                            (scheme, authority, Self::normalize_path(&path), query)
                        }
                    }
                },
                |authority| {
                    (
                        self.scheme.clone(),
                        Some(authority.clone()),
                        Self::normalize_path(&relative_reference.path),
                        relative_reference.query.clone(),
                    )
                },
            )
        };
        let mut temp = Self {
            scheme,
            authority,
            path,
            query,
            fragment: relative_reference.fragment.clone(),
            raw: String::default(),
        };
        temp.update_raw();
        temp
    }

    /// Borrow the scheme (if any) component of the URI.
    #[must_use]
    pub fn scheme(&self) -> Option<&str> {
        // NOTE: This seemingly magic `as_deref` works because of two
        // things that are going on here:
        // 1) String implements DeRef with `str` as the associated type
        //    `Target`, meaning you can use a String in a context requiring
        //    &str, and String does the conversion work.
        // 2) as_deref works by turning `Option<T>` into `Option<&T::Target>`,
        //    requiring T to implement Deref.  In this case T is String.
        self.scheme.as_deref()
    }

    /// Change the authority of the URI.
    pub fn set_authority<T>(&mut self, authority: T)
    where
        T: Into<Option<Authority>>,
    {
        self.authority = authority.into();
        self.update_raw();
    }

    /// Change the fragment of the URI.
    pub fn set_fragment<T>(&mut self, fragment: T)
    where
        T: Into<Option<Vec<u8>>>,
    {
        self.fragment = fragment.into();
        self.update_raw();
    }

    /// Change the path of the URI.
    ///
    /// Note: See [`path`](#method.path) for special notes about what the
    /// segments of the path mean.
    pub fn set_path<T>(&mut self, path: T)
    where
        T: Into<Vec<Vec<u8>>>,
    {
        self.path = path.into();
        self.update_raw();
    }

    /// Change the path of the URI using a string which is split by its slash
    /// (`/`) characters to determine the path segments.
    ///
    /// Note: See [`path`](#method.path) for special notes about what the
    /// segments of the path mean.
    pub fn set_path_from_str<T>(&mut self, path: T)
    where
        T: AsRef<str>,
    {
        match path.as_ref() {
            "" => self.set_path(vec![]),
            path => self.set_path(
                path.split('/')
                    .map(|segment| segment.as_bytes().to_vec())
                    .collect::<Vec<Vec<u8>>>(),
            ),
        }
        self.update_raw();
    }

    /// Change the query of the URI.
    pub fn set_query<T>(&mut self, query: T)
    where
        T: Into<Option<Vec<u8>>>,
    {
        self.query = query.into();
        self.update_raw();
    }

    /// Change the scheme of the URI.
    ///
    /// # Errors
    ///
    /// The set of characters allowed in the scheme of a URI is limited.
    /// [`Error::IllegalCharacter`](enum.Error.html#variant.IllegalCharacter)
    /// is returned if you try to use a character that isn't allowed.
    pub fn set_scheme<T>(&mut self, scheme: T) -> Result<(), Error>
    where
        T: Into<Option<String>>,
    {
        self.scheme = match scheme.into() {
            Some(scheme) => {
                Self::check_scheme(&scheme)?;
                Some(scheme)
            }
            None => None,
        };
        self.update_raw();
        Ok(())
    }

    // /// Remove and return the authority portion (if any) of the URI.
    // #[must_use]
    // pub fn take_authority(&mut self) -> Option<Authority> {
    //     let authority = self.authority.take();
    //     self.update_raw();
    //     authority
    // }

    // /// Remove and return the fragment portion (if any) of the URI.
    // #[must_use]
    // pub fn take_fragment(&mut self) -> Option<Vec<u8>> {
    //     let fragment = self.fragment.take();
    //     self.update_raw();
    //     fragment
    // }

    // /// Remove and return the query portion (if any) of the URI.
    // #[must_use]
    // pub fn take_query(&mut self) -> Option<Vec<u8>> {
    //     let query = self.query.take();
    //     self.update_raw();
    //     query
    // }

    // /// Remove and return the scheme portion (if any) of the URI.
    // #[must_use]
    // pub fn take_scheme(&mut self) -> Option<String> {
    //     let scheme = self.scheme.take();
    //     self.update_raw();
    //     scheme
    // }

    /// Borrow the `user_info` portion (if any) of the Authority (if any) of the
    /// URI.
    ///
    /// Note that you can get `None` if there is either no Authority in the URI
    /// or there is an Authority in the URI but it has no `user_info` in it.
    #[must_use]
    pub fn user_info(&self) -> Option<&[u8]> {
        self.authority.as_ref().and_then(Authority::user_info)
    }

    /// Convert the fragment (if any) into a string.
    ///
    /// # Errors
    ///
    /// Since fragments may contain non-UTF8 byte sequences, this function may
    /// return [`Error::CannotExpressAsUtf8`][CannotExpressAsUtf8].
    ///
    /// [CannotExpressAsUtf8]: enum.Error.html#variant.CannotExpressAsUtf8
    pub fn user_info_to_string(&self) -> Result<Option<String>, FromUtf8Error> {
        self.user_info()
            .map(|user_info| String::from_utf8(user_info.to_vec()))
            .transpose()
    }

    // ----------------------------------------------------------------------------------------------
    //                                         private methods
    // ----------------------------------------------------------------------------------------------
    fn is_path_absolute<T>(path: T) -> bool
    where
        T: AsRef<[Vec<u8>]>,
    {
        matches!(path.as_ref(), [segment, ..] if segment.is_empty())
    }

    fn decode_query_or_fragment<T>(query_or_fragment: T, context: Context) -> Result<Vec<u8>, Error>
    where
        T: AsRef<str>,
    {
        decode_element(
            query_or_fragment,
            &QUERY_OR_FRAGMENT_NOT_PCT_ENCODED,
            context,
        )
    }

    fn update_raw(&mut self) {
        let mut raw = String::new();

        if let Some(scheme) = &self.scheme {
            write!(&mut raw, "{}:", scheme).unwrap();
        }
        if let Some(authority) = &self.authority {
            write!(&mut raw, "//{}", authority).unwrap();
        }
        // Special case: absolute but otherwise empty path.
        if Self::is_path_absolute(&self.path) && self.path.len() == 1 {
            write!(&mut raw, "/").unwrap();
        }
        for (i, segment) in self.path.iter().enumerate() {
            write!(
                &mut raw,
                "{}",
                encode_element(segment, &PCHAR_NOT_PCT_ENCODED)
            )
                .unwrap();
            if i + 1 < self.path.len() {
                write!(&mut raw, "/").unwrap();
            }
        }
        if let Some(query) = &self.query {
            write!(
                &mut raw,
                "?{}",
                encode_element(query, &QUERY_NOT_PCT_ENCODED_WITHOUT_PLUS)
            )
                .unwrap();
        }
        if let Some(fragment) = &self.fragment {
            write!(
                raw,
                "#{}",
                encode_element(fragment, &QUERY_OR_FRAGMENT_NOT_PCT_ENCODED)
            )
                .unwrap();
        }

        self.raw = raw;
    }

    fn parse_fragment(query_and_or_fragment: &str) -> Result<(Option<Vec<u8>>, &str), Error> {
        if let Some(fragment_delimiter) = query_and_or_fragment.find('#') {
            let fragment = Self::decode_query_or_fragment(
                &query_and_or_fragment[fragment_delimiter + 1..],
                Context::Fragment,
            )?;
            Ok((
                Some(fragment),
                &query_and_or_fragment[0..fragment_delimiter],
            ))
        } else {
            Ok((None, query_and_or_fragment))
        }
    }

    fn parse_path<T>(path_string: T) -> Result<Vec<Vec<u8>>, Error>
    where
        T: AsRef<str>,
    {
        match path_string.as_ref() {
            "/" => {
                // Special case of an empty absolute path, which we want to
                // represent as single empty-string element to indicate that it
                // is absolute.
                Ok(vec![vec![]])
            }

            "" => {
                // Special case of an empty relative path, which we want to
                // represent as an empty vector.
                Ok(vec![])
            }

            path_string => path_string
                .split('/')
                .map(|segment| decode_element(&segment, &PCHAR_NOT_PCT_ENCODED, Context::Path))
                .collect(),
        }
    }

    fn parse_query<T>(query_and_or_fragment: T) -> Result<Option<Vec<u8>>, Error>
    where
        T: AsRef<str>,
    {
        let query_and_or_fragment = query_and_or_fragment.as_ref();
        if query_and_or_fragment.is_empty() {
            Ok(None)
        } else {
            let query =
                Self::decode_query_or_fragment(&query_and_or_fragment[1..], Context::Query)?;
            Ok(Some(query))
        }
    }

    fn parse_scheme(uri_string: &str) -> Result<(Option<String>, &str), Error> {
        // Limit our search so we don't scan into the authority
        // or path elements, because these may have the colon
        // character as well, which we might misinterpret
        // as the scheme delimiter.
        let authority_or_path_delimiter_start = uri_string.find('/').unwrap_or(uri_string.len());
        if let Some(scheme_end) = &uri_string[0..authority_or_path_delimiter_start].find(':') {
            let scheme = Self::check_scheme(&uri_string[0..*scheme_end])?.to_lowercase();
            Ok((Some(scheme), &uri_string[*scheme_end + 1..]))
        } else {
            Ok((None, uri_string))
        }
    }

    fn normalize_path<T>(original_path: T) -> Vec<Vec<u8>>
    where
        T: AsRef<[Vec<u8>]>,
    {
        // Rebuild the path one segment
        // at a time, removing and applying special
        // navigation segments ("." and "..") as we go.
        //
        // The `at_directory_level` variable tracks whether or not
        // the `normalized_path` refers to a directory.
        let mut at_directory_level = false;
        let mut normalized_path = Vec::new();
        for segment in original_path.as_ref() {
            if segment == b"." {
                at_directory_level = true;
            } else if segment == b".." {
                // Remove last path element
                // if we can navigate up a level.
                if !normalized_path.is_empty()
                    && Self::can_navigate_path_up_one_level(&normalized_path)
                {
                    normalized_path.pop();
                }
                at_directory_level = true;
            } else {
                // Non-relative elements can just
                // transfer over fine.  An empty
                // segment marks a transition to
                // a directory level context.  If we're
                // already in that context, we
                // want to ignore the transition.
                let new_at_directory_level = segment.is_empty();
                if !at_directory_level || !segment.is_empty() {
                    normalized_path.push(segment.clone());
                }
                at_directory_level = new_at_directory_level;
            }
        }

        // If at the end of rebuilding the path,
        // we're in a directory level context,
        // add an empty segment to mark the fact.
        match (at_directory_level, normalized_path.last()) {
            (true, Some(segment)) if !segment.is_empty() => {
                normalized_path.push(vec![]);
            }
            _ => (),
        }
        normalized_path
    }

    fn split_authority_from_path_and_parse_them<T>(
        authority_and_path_string: T,
    ) -> Result<(Option<Authority>, Vec<Vec<u8>>), Error>
    where
        T: AsRef<str>,
    {
        // Split authority from path.  If there is an authority, parse it.
        let authority_and_path_string = authority_and_path_string.as_ref();
        if let Some(authority_and_path_string) = authority_and_path_string.strip_prefix("//") {
            // First separate the authority from the path.
            let authority_end = authority_and_path_string
                .find('/')
                .unwrap_or(authority_and_path_string.len());
            let authority_string = &authority_and_path_string[0..authority_end];
            let path_string = &authority_and_path_string[authority_end..];

            // Parse the elements inside the authority string.
            let authority = Authority::parse(authority_string)?;
            let path = if path_string.is_empty() {
                vec![vec![]]
            } else {
                Self::parse_path(path_string)?
            };
            Ok((Some(authority), path))
        } else {
            let path = Self::parse_path(authority_and_path_string)?;
            Ok((None, path))
        }
    }

    fn can_navigate_path_up_one_level<T>(path: T) -> bool
    where
        T: AsRef<[Vec<u8>]>,
    {
        let path = path.as_ref();
        match path.first() {
            // First segment empty means path has leading slash,
            // so we can only navigate up if there are two or more segments.
            Some(segment) if segment.is_empty() => path.len() > 1,

            // Otherwise, we can navigate up as long as there is at least one
            // segment.
            Some(_) => true,
            None => false,
        }
    }

    fn check_scheme<T>(scheme: T) -> Result<T, Error>
    where
        T: AsRef<str>,
    {
        match scheme.as_ref() {
            "" => return Err(Error::EmptyScheme),
            scheme => scheme.chars().enumerate().try_fold((), |_, (i, c)| {
                let valid_characters: &HashSet<char> =
                    if i == 0 { &ALPHA } else { &SCHEME_NOT_FIRST };
                if valid_characters.contains(&c) {
                    Ok(())
                } else {
                    Err(Error::IllegalCharacter(Context::Scheme))
                }
            })?,
        };
        Ok(scheme)
    }
}

impl std::fmt::Debug for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Uri").field(&self.to_string()).finish()
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.raw)
    }
}

impl Deref for Uri {
    type Target = str;
    fn deref(&self) -> &str {
        &self.raw
    }
}

impl TryFrom<&'_ str> for Uri {
    type Error = Error;

    fn try_from(uri_string: &'_ str) -> Result<Self, Self::Error> {
        Uri::parse(uri_string)
    }
}

impl TryFrom<String> for Uri {
    type Error = Error;

    fn try_from(uri_string: String) -> Result<Self, Self::Error> {
        Uri::parse(uri_string)
    }
}

impl From<AbsoluteUri> for Uri {
    fn from(absolute_uri: AbsoluteUri) -> Self {
        absolute_uri.uri
    }
}

impl From<&AbsoluteUri> for Uri {
    fn from(absolute_uri: &AbsoluteUri) -> Self {
        absolute_uri.uri.clone()
    }
}

impl Any for Uri {

    fn is_equal(&self, other: &Self) -> bool {
        self.raw == other.raw
    }

    fn instance_of(&self, type_name: &str) -> bool {
        "Uri" == type_name
    }

    fn type_of(&self) -> String {
        "Uri".to_string()
    }
}

impl PartialOrd for Uri {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.partial_cmp(other)
    }
}

impl Ord for Uri {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Ordered for Uri {}

#[cfg(feature = "url")]
impl TryInto<Uri> for url_::Url {
    type Error = Error;
    fn try_into(self) -> Result<Uri, Self::Error> {
        Uri::parse(self)
    }
}

#[cfg(feature = "url")]
impl TryFrom<Uri> for url_::Url {
    type Error = url_::ParseError;

    fn try_from(value: Uri) -> Result<Self, Self::Error> {
        value.try_into()
    }
}

#[cfg(feature = "serde")]
impl serde_::Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_::Serializer,
    {
        serializer.serialize_str(&self.raw)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde_::Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Uri::parse(s).map_err(serde_::de::Error::custom)
    }
}




