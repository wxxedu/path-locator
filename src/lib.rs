use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathError {
    NotFound,
}

unsafe impl Send for PathError {}
unsafe impl Sync for PathError {}

pub fn find(
    path: impl AsRef<Path>,
    name: impl AsRef<Path>,
) -> Result<PathBuf, PathError> {
    let mut path = path.as_ref();
    let name = name.as_ref();

    loop {
        let candidate = path.join(&name);
        if candidate.exists() {
            return Ok(candidate);
        }

        path = match path.parent() {
            Some(parent) => parent,
            None => return Err(PathError::NotFound),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let path = Path::new("src");
        let name = Path::new(".gitignore");
        let res = find(path, name);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Path::new(".gitignore"));
    }
}
