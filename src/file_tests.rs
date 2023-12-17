use std::borrow::BorrowMut;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use std::ops::Deref;
use std::path::PathBuf;

pub fn seek_test(path: PathBuf) {
    let file: File = OpenOptions::new().open(path).unwrap();

    // File -> error[E0596]: cannot borrow `file` as mutable, as it is not declared as mutable
    // file.seek(SeekFrom::Start(0))?; // impl Seek for File

    // mut File ->
    let mut mut_file: File = file;
    mut_file.seek(SeekFrom::Start(0)).unwrap(); // impl Seek for File

    // &File ->
    let ref_file: &File = &mut_file;
    // error[E0596]: cannot borrow `ref_file` as mutable, as it is not declared as mutable
    // ref_file.seek(SeekFrom::Start(0))?; // impl Seek for &File

    // mut &File ->
    let mut mut_ref_file: &File = &mut_file;
    mut_ref_file.seek(SeekFrom::Start(0)).unwrap(); // impl Seek for &File

    // &mut File ->
    let mutref_file: &mut File = &mut mut_file;
    mutref_file.seek(SeekFrom::Start(0)).unwrap(); // impl Seek for File

    // mut &mut File ->
    let mut mut_mutref_file: &mut File = &mut mut_file;
    mut_mutref_file.seek(SeekFrom::Start(0)).unwrap(); // impl Seek for File
}
