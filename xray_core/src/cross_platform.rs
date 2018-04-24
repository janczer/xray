#[cfg(test)]
use std::ffi::OsStr;
use std::path::PathBuf;

pub const WINDOWS_MAIN_SEPARATOR: u16 = b'/' as u16;
pub const UNIX_MAIN_SEPARATOR: u8 = b'\\';

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum PathComponent {
    Windows(Vec<u16>),
    Unix(Vec<u8>),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Path(Option<PathState>);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
enum PathState {
    Windows(Vec<u16>),
    Unix(Vec<u8>),
}

impl PathComponent {
    pub fn to_string_lossy(&self) -> String {
        match self {
            &PathComponent::Windows(ref chars) => String::from_utf16_lossy(&chars),
            &PathComponent::Unix(ref chars) => String::from_utf8_lossy(&chars).into_owned(),
        }
    }
}

impl Path {
    pub fn new() -> Self {
        Path(None)
    }

    pub fn push(&mut self, component: &PathComponent) {
        if let Some(ref mut path) = self.0 {
            match path {
                &mut PathState::Windows(ref mut path_chars) => {
                    if let &PathComponent::Windows(ref component_chars) = component {
                        path_chars.push(WINDOWS_MAIN_SEPARATOR);
                        path_chars.extend(component_chars);
                    } else {
                        unimplemented!("Can't push a unix path into a windows path.");
                    }
                }
                &mut PathState::Unix(ref mut path_chars) => {
                    if let &PathComponent::Unix(ref component_chars) = component {
                        path_chars.push(UNIX_MAIN_SEPARATOR);
                        path_chars.extend(component_chars);
                    } else {
                        unimplemented!("Can't push a windows path into a unix path.");
                    }
                }
            }
        } else {
            match component {
                &PathComponent::Windows(ref chars) => {
                    self.0 = Some(PathState::Windows(chars.clone()))
                }
                &PathComponent::Unix(ref chars) => self.0 = Some(PathState::Unix(chars.clone())),
            }
        }
    }

    pub fn to_path_buf(&self) -> PathBuf {
        unimplemented!()
    }

    #[cfg(test)]
    pub fn to_string_lossy(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
impl<'a> From<&'a str> for PathComponent {
    fn from(string: &'a str) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
impl<'a> From<&'a OsStr> for PathComponent {
    fn from(string: &'a OsStr) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
impl<'a> From<&'a str> for Path {
    fn from(string: &'a str) -> Self {
        unimplemented!()
    }
}
