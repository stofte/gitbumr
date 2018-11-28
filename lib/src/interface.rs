/* generated by rust_qt_binding_generator */
use libc::{c_char, c_ushort, c_int};
use std::slice;
use std::char::decode_utf16;

use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr::null;

use implementation::*;


#[repr(C)]
pub struct COption<T> {
    data: T,
    some: bool,
}

impl<T> COption<T> {
    #![allow(dead_code)]
    fn into(self) -> Option<T> {
        if self.some {
            Some(self.data)
        } else {
            None
        }
    }
}

impl<T> From<Option<T>> for COption<T>
where
    T: Default,
{
    fn from(t: Option<T>) -> COption<T> {
        if let Some(v) = t {
            COption {
                data: v,
                some: true,
            }
        } else {
            COption {
                data: T::default(),
                some: false,
            }
        }
    }
}


pub enum QString {}

fn set_string_from_utf16(s: &mut String, str: *const c_ushort, len: c_int) {
    let utf16 = unsafe { slice::from_raw_parts(str, to_usize(len)) };
    let characters = decode_utf16(utf16.iter().cloned())
        .map(|r| r.unwrap());
    s.clear();
    s.extend(characters);
}



#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub enum SortOrder {
    Ascending = 0,
    Descending = 1,
}

#[repr(C)]
pub struct QModelIndex {
    row: c_int,
    internal_id: usize,
}


fn to_usize(n: c_int) -> usize {
    if n < 0 {
        panic!("Cannot cast {} to usize", n);
    }
    n as usize
}


fn to_c_int(n: usize) -> c_int {
    if n > c_int::max_value() as usize {
        panic!("Cannot cast {} to c_int", n);
    }
    n as c_int
}


pub struct BranchesQObject {}

pub struct BranchesEmitter {
    qobject: Arc<AtomicPtr<BranchesQObject>>,
    new_data_ready: fn(*mut BranchesQObject),
}

unsafe impl Send for BranchesEmitter {}

impl BranchesEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> BranchesEmitter {
        BranchesEmitter {
            qobject: self.qobject.clone(),
            new_data_ready: self.new_data_ready,
        }
    }
    fn clear(&self) {
        let n: *const BranchesQObject = null();
        self.qobject.store(n as *mut BranchesQObject, Ordering::SeqCst);
    }
    pub fn new_data_ready(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.new_data_ready)(ptr);
        }
    }
}

#[derive(Clone)]
pub struct BranchesList {
    qobject: *mut BranchesQObject,
    layout_about_to_be_changed: fn(*mut BranchesQObject),
    layout_changed: fn(*mut BranchesQObject),
    data_changed: fn(*mut BranchesQObject, usize, usize),
    begin_reset_model: fn(*mut BranchesQObject),
    end_reset_model: fn(*mut BranchesQObject),
    begin_insert_rows: fn(*mut BranchesQObject, usize, usize),
    end_insert_rows: fn(*mut BranchesQObject),
    begin_move_rows: fn(*mut BranchesQObject, usize, usize, usize),
    end_move_rows: fn(*mut BranchesQObject),
    begin_remove_rows: fn(*mut BranchesQObject, usize, usize),
    end_remove_rows: fn(*mut BranchesQObject),
}

impl BranchesList {
    pub fn layout_about_to_be_changed(&mut self) {
        (self.layout_about_to_be_changed)(self.qobject);
    }
    pub fn layout_changed(&mut self) {
        (self.layout_changed)(self.qobject);
    }
    pub fn data_changed(&mut self, first: usize, last: usize) {
        (self.data_changed)(self.qobject, first, last);
    }
    pub fn begin_reset_model(&mut self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&mut self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&mut self, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, first, last);
    }
    pub fn end_insert_rows(&mut self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_move_rows(&mut self, first: usize, last: usize, destination: usize) {
        (self.begin_move_rows)(self.qobject, first, last, destination);
    }
    pub fn end_move_rows(&mut self) {
        (self.end_move_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&mut self, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, first, last);
    }
    pub fn end_remove_rows(&mut self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait BranchesTrait {
    fn new(emit: BranchesEmitter, model: BranchesList) -> Self;
    fn emit(&mut self) -> &mut BranchesEmitter;
    fn row_count(&self) -> usize;
    fn insert_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn remove_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn can_fetch_more(&self) -> bool {
        false
    }
    fn fetch_more(&mut self) {}
    fn sort(&mut self, u8, SortOrder) {}
    fn checkedout(&self, index: usize) -> bool;
    fn name(&self, index: usize) -> &str;
    fn oid(&self, index: usize) -> &str;
}

#[no_mangle]
pub extern "C" fn branches_new(
    branches: *mut BranchesQObject,
    branches_new_data_ready: fn(*mut BranchesQObject),
    branches_layout_about_to_be_changed: fn(*mut BranchesQObject),
    branches_layout_changed: fn(*mut BranchesQObject),
    branches_data_changed: fn(*mut BranchesQObject, usize, usize),
    branches_begin_reset_model: fn(*mut BranchesQObject),
    branches_end_reset_model: fn(*mut BranchesQObject),
    branches_begin_insert_rows: fn(*mut BranchesQObject, usize, usize),
    branches_end_insert_rows: fn(*mut BranchesQObject),
    branches_begin_move_rows: fn(*mut BranchesQObject, usize, usize, usize),
    branches_end_move_rows: fn(*mut BranchesQObject),
    branches_begin_remove_rows: fn(*mut BranchesQObject, usize, usize),
    branches_end_remove_rows: fn(*mut BranchesQObject),
) -> *mut Branches {
    let branches_emit = BranchesEmitter {
        qobject: Arc::new(AtomicPtr::new(branches)),
        new_data_ready: branches_new_data_ready,
    };
    let model = BranchesList {
        qobject: branches,
        layout_about_to_be_changed: branches_layout_about_to_be_changed,
        layout_changed: branches_layout_changed,
        data_changed: branches_data_changed,
        begin_reset_model: branches_begin_reset_model,
        end_reset_model: branches_end_reset_model,
        begin_insert_rows: branches_begin_insert_rows,
        end_insert_rows: branches_end_insert_rows,
        begin_move_rows: branches_begin_move_rows,
        end_move_rows: branches_end_move_rows,
        begin_remove_rows: branches_begin_remove_rows,
        end_remove_rows: branches_end_remove_rows,
    };
    let d_branches = Branches::new(branches_emit, model);
    Box::into_raw(Box::new(d_branches))
}

#[no_mangle]
pub unsafe extern "C" fn branches_free(ptr: *mut Branches) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn branches_row_count(ptr: *const Branches) -> c_int {
    to_c_int((&*ptr).row_count())
}
#[no_mangle]
pub unsafe extern "C" fn branches_insert_rows(ptr: *mut Branches, row: c_int, count: c_int) -> bool {
    (&mut *ptr).insert_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn branches_remove_rows(ptr: *mut Branches, row: c_int, count: c_int) -> bool {
    (&mut *ptr).remove_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn branches_can_fetch_more(ptr: *const Branches) -> bool {
    (&*ptr).can_fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn branches_fetch_more(ptr: *mut Branches) {
    (&mut *ptr).fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn branches_sort(
    ptr: *mut Branches,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn branches_data_checkedout(ptr: *const Branches, row: c_int) -> bool {
    let o = &*ptr;
    o.checkedout(to_usize(row)).into()
}

#[no_mangle]
pub unsafe extern "C" fn branches_data_name(
    ptr: *const Branches, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.name(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn branches_data_oid(
    ptr: *const Branches, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.oid(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

pub struct GitQObject {}

pub struct GitEmitter {
    qobject: Arc<AtomicPtr<GitQObject>>,
    revwalk_filter_changed: fn(*mut GitQObject),
}

unsafe impl Send for GitEmitter {}

impl GitEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> GitEmitter {
        GitEmitter {
            qobject: self.qobject.clone(),
            revwalk_filter_changed: self.revwalk_filter_changed,
        }
    }
    fn clear(&self) {
        let n: *const GitQObject = null();
        self.qobject.store(n as *mut GitQObject, Ordering::SeqCst);
    }
    pub fn revwalk_filter_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.revwalk_filter_changed)(ptr);
        }
    }
}

pub trait GitTrait {
    fn new(emit: GitEmitter,
        branches: Branches) -> Self;
    fn emit(&mut self) -> &mut GitEmitter;
    fn branches(&self) -> &Branches;
    fn branches_mut(&mut self) -> &mut Branches;
    fn revwalk_filter(&self) -> &str;
    fn load(&mut self, path: String) -> ();
}

#[no_mangle]
pub extern "C" fn git_new(
    git: *mut GitQObject,
    branches: *mut BranchesQObject,
    branches_new_data_ready: fn(*mut BranchesQObject),
    branches_layout_about_to_be_changed: fn(*mut BranchesQObject),
    branches_layout_changed: fn(*mut BranchesQObject),
    branches_data_changed: fn(*mut BranchesQObject, usize, usize),
    branches_begin_reset_model: fn(*mut BranchesQObject),
    branches_end_reset_model: fn(*mut BranchesQObject),
    branches_begin_insert_rows: fn(*mut BranchesQObject, usize, usize),
    branches_end_insert_rows: fn(*mut BranchesQObject),
    branches_begin_move_rows: fn(*mut BranchesQObject, usize, usize, usize),
    branches_end_move_rows: fn(*mut BranchesQObject),
    branches_begin_remove_rows: fn(*mut BranchesQObject, usize, usize),
    branches_end_remove_rows: fn(*mut BranchesQObject),
    git_revwalk_filter_changed: fn(*mut GitQObject),
) -> *mut Git {
    let branches_emit = BranchesEmitter {
        qobject: Arc::new(AtomicPtr::new(branches)),
        new_data_ready: branches_new_data_ready,
    };
    let model = BranchesList {
        qobject: branches,
        layout_about_to_be_changed: branches_layout_about_to_be_changed,
        layout_changed: branches_layout_changed,
        data_changed: branches_data_changed,
        begin_reset_model: branches_begin_reset_model,
        end_reset_model: branches_end_reset_model,
        begin_insert_rows: branches_begin_insert_rows,
        end_insert_rows: branches_end_insert_rows,
        begin_move_rows: branches_begin_move_rows,
        end_move_rows: branches_end_move_rows,
        begin_remove_rows: branches_begin_remove_rows,
        end_remove_rows: branches_end_remove_rows,
    };
    let d_branches = Branches::new(branches_emit, model);
    let git_emit = GitEmitter {
        qobject: Arc::new(AtomicPtr::new(git)),
        revwalk_filter_changed: git_revwalk_filter_changed,
    };
    let d_git = Git::new(git_emit,
        d_branches);
    Box::into_raw(Box::new(d_git))
}

#[no_mangle]
pub unsafe extern "C" fn git_free(ptr: *mut Git) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn git_branches_get(ptr: *mut Git) -> *mut Branches {
    (&mut *ptr).branches_mut()
}

#[no_mangle]
pub unsafe extern "C" fn git_revwalk_filter_get(
    ptr: *const Git,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.revwalk_filter();
    let s: *const c_char = v.as_ptr() as (*const c_char);
    set(p, s, to_c_int(v.len()));
}

#[no_mangle]
pub unsafe extern "C" fn git_load(ptr: *mut Git, path_str: *const c_ushort, path_len: c_int) -> () {
    let mut path = String::new();
    set_string_from_utf16(&mut path, path_str, path_len);
    let o = &mut *ptr;
    let r = o.load(path);
    r
}

pub struct LogQObject {}

pub struct LogEmitter {
    qobject: Arc<AtomicPtr<LogQObject>>,
    new_data_ready: fn(*mut LogQObject),
}

unsafe impl Send for LogEmitter {}

impl LogEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> LogEmitter {
        LogEmitter {
            qobject: self.qobject.clone(),
            new_data_ready: self.new_data_ready,
        }
    }
    fn clear(&self) {
        let n: *const LogQObject = null();
        self.qobject.store(n as *mut LogQObject, Ordering::SeqCst);
    }
    pub fn new_data_ready(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.new_data_ready)(ptr);
        }
    }
}

#[derive(Clone)]
pub struct LogList {
    qobject: *mut LogQObject,
    layout_about_to_be_changed: fn(*mut LogQObject),
    layout_changed: fn(*mut LogQObject),
    data_changed: fn(*mut LogQObject, usize, usize),
    begin_reset_model: fn(*mut LogQObject),
    end_reset_model: fn(*mut LogQObject),
    begin_insert_rows: fn(*mut LogQObject, usize, usize),
    end_insert_rows: fn(*mut LogQObject),
    begin_move_rows: fn(*mut LogQObject, usize, usize, usize),
    end_move_rows: fn(*mut LogQObject),
    begin_remove_rows: fn(*mut LogQObject, usize, usize),
    end_remove_rows: fn(*mut LogQObject),
}

impl LogList {
    pub fn layout_about_to_be_changed(&mut self) {
        (self.layout_about_to_be_changed)(self.qobject);
    }
    pub fn layout_changed(&mut self) {
        (self.layout_changed)(self.qobject);
    }
    pub fn data_changed(&mut self, first: usize, last: usize) {
        (self.data_changed)(self.qobject, first, last);
    }
    pub fn begin_reset_model(&mut self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&mut self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&mut self, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, first, last);
    }
    pub fn end_insert_rows(&mut self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_move_rows(&mut self, first: usize, last: usize, destination: usize) {
        (self.begin_move_rows)(self.qobject, first, last, destination);
    }
    pub fn end_move_rows(&mut self) {
        (self.end_move_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&mut self, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, first, last);
    }
    pub fn end_remove_rows(&mut self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait LogTrait {
    fn new(emit: LogEmitter, model: LogList) -> Self;
    fn emit(&mut self) -> &mut LogEmitter;
    fn filter(&mut self, filter: String) -> ();
    fn load(&mut self, path: String) -> ();
    fn row_count(&self) -> usize;
    fn insert_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn remove_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn can_fetch_more(&self) -> bool {
        false
    }
    fn fetch_more(&mut self) {}
    fn sort(&mut self, u8, SortOrder) {}
    fn author(&self, index: usize) -> &str;
    fn message(&self, index: usize) -> &str;
    fn oid(&self, index: usize) -> &str;
    fn time(&self, index: usize) -> &str;
}

#[no_mangle]
pub extern "C" fn log_new(
    log: *mut LogQObject,
    log_new_data_ready: fn(*mut LogQObject),
    log_layout_about_to_be_changed: fn(*mut LogQObject),
    log_layout_changed: fn(*mut LogQObject),
    log_data_changed: fn(*mut LogQObject, usize, usize),
    log_begin_reset_model: fn(*mut LogQObject),
    log_end_reset_model: fn(*mut LogQObject),
    log_begin_insert_rows: fn(*mut LogQObject, usize, usize),
    log_end_insert_rows: fn(*mut LogQObject),
    log_begin_move_rows: fn(*mut LogQObject, usize, usize, usize),
    log_end_move_rows: fn(*mut LogQObject),
    log_begin_remove_rows: fn(*mut LogQObject, usize, usize),
    log_end_remove_rows: fn(*mut LogQObject),
) -> *mut Log {
    let log_emit = LogEmitter {
        qobject: Arc::new(AtomicPtr::new(log)),
        new_data_ready: log_new_data_ready,
    };
    let model = LogList {
        qobject: log,
        layout_about_to_be_changed: log_layout_about_to_be_changed,
        layout_changed: log_layout_changed,
        data_changed: log_data_changed,
        begin_reset_model: log_begin_reset_model,
        end_reset_model: log_end_reset_model,
        begin_insert_rows: log_begin_insert_rows,
        end_insert_rows: log_end_insert_rows,
        begin_move_rows: log_begin_move_rows,
        end_move_rows: log_end_move_rows,
        begin_remove_rows: log_begin_remove_rows,
        end_remove_rows: log_end_remove_rows,
    };
    let d_log = Log::new(log_emit, model);
    Box::into_raw(Box::new(d_log))
}

#[no_mangle]
pub unsafe extern "C" fn log_free(ptr: *mut Log) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn log_filter(ptr: *mut Log, filter_str: *const c_ushort, filter_len: c_int) -> () {
    let mut filter = String::new();
    set_string_from_utf16(&mut filter, filter_str, filter_len);
    let o = &mut *ptr;
    let r = o.filter(filter);
    r
}

#[no_mangle]
pub unsafe extern "C" fn log_load(ptr: *mut Log, path_str: *const c_ushort, path_len: c_int) -> () {
    let mut path = String::new();
    set_string_from_utf16(&mut path, path_str, path_len);
    let o = &mut *ptr;
    let r = o.load(path);
    r
}

#[no_mangle]
pub unsafe extern "C" fn log_row_count(ptr: *const Log) -> c_int {
    to_c_int((&*ptr).row_count())
}
#[no_mangle]
pub unsafe extern "C" fn log_insert_rows(ptr: *mut Log, row: c_int, count: c_int) -> bool {
    (&mut *ptr).insert_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn log_remove_rows(ptr: *mut Log, row: c_int, count: c_int) -> bool {
    (&mut *ptr).remove_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn log_can_fetch_more(ptr: *const Log) -> bool {
    (&*ptr).can_fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn log_fetch_more(ptr: *mut Log) {
    (&mut *ptr).fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn log_sort(
    ptr: *mut Log,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn log_data_author(
    ptr: *const Log, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.author(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn log_data_message(
    ptr: *const Log, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.message(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn log_data_oid(
    ptr: *const Log, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.oid(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn log_data_time(
    ptr: *const Log, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.time(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

pub struct RepositoriesQObject {}

pub struct RepositoriesEmitter {
    qobject: Arc<AtomicPtr<RepositoriesQObject>>,
    active_repository_changed: fn(*mut RepositoriesQObject),
    new_data_ready: fn(*mut RepositoriesQObject),
}

unsafe impl Send for RepositoriesEmitter {}

impl RepositoriesEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> RepositoriesEmitter {
        RepositoriesEmitter {
            qobject: self.qobject.clone(),
            active_repository_changed: self.active_repository_changed,
            new_data_ready: self.new_data_ready,
        }
    }
    fn clear(&self) {
        let n: *const RepositoriesQObject = null();
        self.qobject.store(n as *mut RepositoriesQObject, Ordering::SeqCst);
    }
    pub fn active_repository_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.active_repository_changed)(ptr);
        }
    }
    pub fn new_data_ready(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.new_data_ready)(ptr);
        }
    }
}

#[derive(Clone)]
pub struct RepositoriesList {
    qobject: *mut RepositoriesQObject,
    layout_about_to_be_changed: fn(*mut RepositoriesQObject),
    layout_changed: fn(*mut RepositoriesQObject),
    data_changed: fn(*mut RepositoriesQObject, usize, usize),
    begin_reset_model: fn(*mut RepositoriesQObject),
    end_reset_model: fn(*mut RepositoriesQObject),
    begin_insert_rows: fn(*mut RepositoriesQObject, usize, usize),
    end_insert_rows: fn(*mut RepositoriesQObject),
    begin_move_rows: fn(*mut RepositoriesQObject, usize, usize, usize),
    end_move_rows: fn(*mut RepositoriesQObject),
    begin_remove_rows: fn(*mut RepositoriesQObject, usize, usize),
    end_remove_rows: fn(*mut RepositoriesQObject),
}

impl RepositoriesList {
    pub fn layout_about_to_be_changed(&mut self) {
        (self.layout_about_to_be_changed)(self.qobject);
    }
    pub fn layout_changed(&mut self) {
        (self.layout_changed)(self.qobject);
    }
    pub fn data_changed(&mut self, first: usize, last: usize) {
        (self.data_changed)(self.qobject, first, last);
    }
    pub fn begin_reset_model(&mut self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&mut self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&mut self, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, first, last);
    }
    pub fn end_insert_rows(&mut self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_move_rows(&mut self, first: usize, last: usize, destination: usize) {
        (self.begin_move_rows)(self.qobject, first, last, destination);
    }
    pub fn end_move_rows(&mut self) {
        (self.end_move_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&mut self, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, first, last);
    }
    pub fn end_remove_rows(&mut self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait RepositoriesTrait {
    fn new(emit: RepositoriesEmitter, model: RepositoriesList) -> Self;
    fn emit(&mut self) -> &mut RepositoriesEmitter;
    fn active_repository(&self) -> &str;
    fn add(&mut self, path: String) -> bool;
    fn add_last_error(&self) -> String;
    fn init(&mut self, db_file_name: String) -> ();
    fn remove(&mut self, index: u64) -> bool;
    fn set_current(&mut self, id: i64) -> ();
    fn row_count(&self) -> usize;
    fn insert_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn remove_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn can_fetch_more(&self) -> bool {
        false
    }
    fn fetch_more(&mut self) {}
    fn sort(&mut self, u8, SortOrder) {}
    fn current(&self, index: usize) -> bool;
    fn display_name(&self, index: usize) -> &str;
    fn id(&self, index: usize) -> i64;
}

#[no_mangle]
pub extern "C" fn repositories_new(
    repositories: *mut RepositoriesQObject,
    repositories_active_repository_changed: fn(*mut RepositoriesQObject),
    repositories_new_data_ready: fn(*mut RepositoriesQObject),
    repositories_layout_about_to_be_changed: fn(*mut RepositoriesQObject),
    repositories_layout_changed: fn(*mut RepositoriesQObject),
    repositories_data_changed: fn(*mut RepositoriesQObject, usize, usize),
    repositories_begin_reset_model: fn(*mut RepositoriesQObject),
    repositories_end_reset_model: fn(*mut RepositoriesQObject),
    repositories_begin_insert_rows: fn(*mut RepositoriesQObject, usize, usize),
    repositories_end_insert_rows: fn(*mut RepositoriesQObject),
    repositories_begin_move_rows: fn(*mut RepositoriesQObject, usize, usize, usize),
    repositories_end_move_rows: fn(*mut RepositoriesQObject),
    repositories_begin_remove_rows: fn(*mut RepositoriesQObject, usize, usize),
    repositories_end_remove_rows: fn(*mut RepositoriesQObject),
) -> *mut Repositories {
    let repositories_emit = RepositoriesEmitter {
        qobject: Arc::new(AtomicPtr::new(repositories)),
        active_repository_changed: repositories_active_repository_changed,
        new_data_ready: repositories_new_data_ready,
    };
    let model = RepositoriesList {
        qobject: repositories,
        layout_about_to_be_changed: repositories_layout_about_to_be_changed,
        layout_changed: repositories_layout_changed,
        data_changed: repositories_data_changed,
        begin_reset_model: repositories_begin_reset_model,
        end_reset_model: repositories_end_reset_model,
        begin_insert_rows: repositories_begin_insert_rows,
        end_insert_rows: repositories_end_insert_rows,
        begin_move_rows: repositories_begin_move_rows,
        end_move_rows: repositories_end_move_rows,
        begin_remove_rows: repositories_begin_remove_rows,
        end_remove_rows: repositories_end_remove_rows,
    };
    let d_repositories = Repositories::new(repositories_emit, model);
    Box::into_raw(Box::new(d_repositories))
}

#[no_mangle]
pub unsafe extern "C" fn repositories_free(ptr: *mut Repositories) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn repositories_active_repository_get(
    ptr: *const Repositories,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.active_repository();
    let s: *const c_char = v.as_ptr() as (*const c_char);
    set(p, s, to_c_int(v.len()));
}

#[no_mangle]
pub unsafe extern "C" fn repositories_add(ptr: *mut Repositories, path_str: *const c_ushort, path_len: c_int) -> bool {
    let mut path = String::new();
    set_string_from_utf16(&mut path, path_str, path_len);
    let o = &mut *ptr;
    let r = o.add(path);
    r
}

#[no_mangle]
pub unsafe extern "C" fn repositories_add_last_error(ptr: *const Repositories, d: *mut QString, set: fn(*mut QString, str: *const c_char, len: c_int)) {
    let o = &*ptr;
    let r = o.add_last_error();
    let s: *const c_char = r.as_ptr() as (*const c_char);
    set(d, s, r.len() as i32);
}

#[no_mangle]
pub unsafe extern "C" fn repositories_init(ptr: *mut Repositories, db_file_name_str: *const c_ushort, db_file_name_len: c_int) -> () {
    let mut db_file_name = String::new();
    set_string_from_utf16(&mut db_file_name, db_file_name_str, db_file_name_len);
    let o = &mut *ptr;
    let r = o.init(db_file_name);
    r
}

#[no_mangle]
pub unsafe extern "C" fn repositories_remove(ptr: *mut Repositories, index: u64) -> bool {
    let o = &mut *ptr;
    let r = o.remove(index);
    r
}

#[no_mangle]
pub unsafe extern "C" fn repositories_set_current(ptr: *mut Repositories, id: i64) -> () {
    let o = &mut *ptr;
    let r = o.set_current(id);
    r
}

#[no_mangle]
pub unsafe extern "C" fn repositories_row_count(ptr: *const Repositories) -> c_int {
    to_c_int((&*ptr).row_count())
}
#[no_mangle]
pub unsafe extern "C" fn repositories_insert_rows(ptr: *mut Repositories, row: c_int, count: c_int) -> bool {
    (&mut *ptr).insert_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn repositories_remove_rows(ptr: *mut Repositories, row: c_int, count: c_int) -> bool {
    (&mut *ptr).remove_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn repositories_can_fetch_more(ptr: *const Repositories) -> bool {
    (&*ptr).can_fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn repositories_fetch_more(ptr: *mut Repositories) {
    (&mut *ptr).fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn repositories_sort(
    ptr: *mut Repositories,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn repositories_data_current(ptr: *const Repositories, row: c_int) -> bool {
    let o = &*ptr;
    o.current(to_usize(row)).into()
}

#[no_mangle]
pub unsafe extern "C" fn repositories_data_display_name(
    ptr: *const Repositories, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.display_name(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn repositories_data_id(ptr: *const Repositories, row: c_int) -> i64 {
    let o = &*ptr;
    o.id(to_usize(row)).into()
}