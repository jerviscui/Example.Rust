use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;

pub fn seek_test(path: PathBuf) {
    let file: File = OpenOptions::new().open(path).unwrap();

    // File -> error[E0596]: cannot borrow `file` as mutable, as it is not declared as mutable
    // file.seek(SeekFrom::Start(0)).unwrap(); // impl Seek for File

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

pub fn borrow_test(path: PathBuf) {
    let file: File = OpenOptions::new().open(path).unwrap();

    // &File
    let borrow_imt: &File = &file;

    // mut &File
    let mut borrow_imt: &File = &file;
    borrow_imt.seek(SeekFrom::Start(0)).unwrap(); // autorefd method, mut :&File -> &mut &File

    // &mut &File
    let mutref_borrow_imt: &mut &File = &mut borrow_imt;
    mutref_borrow_imt.seek(SeekFrom::Start(0)).unwrap(); // by value method

    // & &File
    let ref_borrow_imt: &&File = &borrow_imt;

    // &mut File
    let mut move_mut: File = file;
    let borrow_mut: &mut File = &mut move_mut;
    borrow_mut.seek(SeekFrom::Start(0)).unwrap(); // by value method

    // mut &mut File
    let mut borrow_mut: &mut File = &mut move_mut;

    // &mut &mut File
    let mutref_borrow_mut: &mut &mut File = &mut borrow_mut;

    // & &mut File
    let ref_borrow_mut: &&mut File = &borrow_mut;

    // &mut File -> &File
    let borrow_imt: &File = &*borrow_mut; // 方法调用时可以自动转为 &mut &File
    let mutref_borrow_imt: &mut &File = &mut &*borrow_mut;
}
