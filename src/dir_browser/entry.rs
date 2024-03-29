use std::{
	fs, io,
	path::{Path, PathBuf},
};

use crate::traits::try_from::MyTryFrom;

#[derive(Debug, Clone)]
pub enum Entry {
	File(String),
	Directory(String),
	Symlink(Symlink),
}

impl Entry {
    pub fn symlink(filename: String, link: PathBuf) -> Self {
        Entry::Symlink(Symlink { name: filename, link })
    }
}

#[derive(Debug, Clone)]
pub struct Symlink {
	pub name: String,
	pub link: PathBuf,
}

impl<P: AsRef<Path>> MyTryFrom<P> for Entry {
	type Error = io::Error;

	fn my_try_from(value: P) -> Result<Self, Self::Error> {
		let value = value.as_ref();
		let metadata = value.metadata()?;
		let name = value
			.file_name()
			.ok_or_else(|| io::ErrorKind::InvalidData)?
			.to_str()
			.ok_or_else(|| io::ErrorKind::InvalidData)?
			.to_owned();

		if metadata.is_file() {
			Ok(Entry::File(name))
		} else if metadata.is_dir() {
			Ok(Entry::Directory(name))
		} else {
			let link = fs::read_link(value)?;
			Ok(Entry::Symlink(Symlink { name, link }))
		}
	}
}

impl TryFrom<PathBuf> for Entry {
	type Error = io::Error;

	fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
		Entry::my_try_from(value)
	}
}
