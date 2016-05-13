use std::os::raw::{c_int, c_uint};

// Standard return values from Symisc public interfaces
const SXRET_OK: c_int = 0;      /* Not an error */
const SXERR_MEM: c_int = -1; /* Out of memory */
const SXERR_IO: c_int = -2; /* IO error */
const SXERR_EMPTY: c_int = -3; /* Empty field */
const SXERR_LOCKED: c_int = -4; /* Locked operation */
const SXERR_ORANGE: c_int = -5; /* Out of range value */
const SXERR_NOTFOUND: c_int = -6; /* Item not found */
const SXERR_LIMIT: c_int = -7; /* Limit reached */
const SXERR_MORE: c_int = -8; /* Need more input */
const SXERR_INVALID: c_int = -9; /* Invalid parameter */
const SXERR_ABORT: c_int = -10; /* User callback request an operation abort */
const SXERR_EXISTS: c_int = -11; /* Item exists */
const SXERR_SYNTAX: c_int = -12; /* Syntax error */
const SXERR_UNKNOWN: c_int = -13; /* Unknown error */
const SXERR_BUSY: c_int = -14; /* Busy operation */
const SXERR_OVERFLOW: c_int = -15; /* Stack or buffer overflow */
const SXERR_WILLBLOCK: c_int = -16; /* Operation will block */
const SXERR_NOTIMPLEMENTED: c_int = -17; /* Operation not implemented */
const SXERR_EOF: c_int = -18; /* End of input */
const SXERR_PERM: c_int = -19; /* Permission error */
const SXERR_NOOP: c_int = -20; /* No-op */
const SXERR_FORMAT: c_int = -21; /* Invalid format */
const SXERR_NEXT: c_int = -22; /* Not an error */
const SXERR_OS: c_int = -23; /* System call return an error */
const SXERR_CORRUPT: c_int = -24; /* Corrupted pointer */
const SXERR_CONTINUE: c_int = -25; /* Not an error: Operation in progress */
const SXERR_NOMATCH: c_int = -26; /* No match */
const SXERR_RESET: c_int = -27; /* Operation reset */
const SXERR_DONE: c_int = -28; /* Not an error */
const SXERR_SHORT: c_int = -29; /* Buffer too short */
const SXERR_PATH: c_int = -30; /* Path error */
const SXERR_TIMEOUT: c_int = -31; /* Timeout */
const SXERR_BIG: c_int = -32; /* Too big for processing */
const SXERR_RETRY: c_int = -33; /* Retry your call */
const SXERR_IGNORE: c_int = -63; /* Ignore */

// Standard UnQLite return values
/// Successful result
pub const UNQLITE_OK: c_int = SXRET_OK;
// Beginning of error codes
/// Out of memory
pub const UNQLITE_NOMEM: c_int = SXERR_MEM;
/// Another thread have released this instance
pub const UNQLITE_ABORT: c_int = SXERR_ABORT;
/// IO error
pub const UNQLITE_IOERR: c_int = SXERR_IO;
/// Corrupt pointer
pub const UNQLITE_CORRUPT: c_int = SXERR_CORRUPT;
/// Forbidden Operation
pub const UNQLITE_LOCKED: c_int = SXERR_LOCKED;
/// The database file is locked
pub const UNQLITE_BUSY: c_int = SXERR_BUSY;
/// Operation done
pub const UNQLITE_DONE: c_int = SXERR_DONE;
/// Permission error
pub const UNQLITE_PERM: c_int = SXERR_PERM;
/// Method not implemented by the underlying Key/Value storage engine
pub const UNQLITE_NOTIMPLEMENTED: c_int = SXERR_NOTIMPLEMENTED;
/// No such record
pub const UNQLITE_NOTFOUND: c_int = SXERR_NOTFOUND;
/// No such method
pub const UNQLITE_NOOP: c_int = SXERR_NOOP;
/// Invalid parameter
pub const UNQLITE_INVALID: c_int = SXERR_INVALID;
/// End Of Input
pub const UNQLITE_EOF: c_int = SXERR_EOF;
/// Unknown configuration option
pub const UNQLITE_UNKNOWN: c_int = SXERR_UNKNOWN;
/// Database limit reached
pub const UNQLITE_LIMIT: c_int = SXERR_LIMIT;
/// Record exists
pub const UNQLITE_EXISTS: c_int = SXERR_EXISTS;
/// Empty record
pub const UNQLITE_EMPTY: c_int = SXERR_EMPTY;
/// Compilation error
pub const UNQLITE_COMPILE_ERR: c_int = -70;
/// Virtual machine error
pub const UNQLITE_VM_ERR: c_int = -71;
/// Full database unlikely
pub const UNQLITE_FULL: c_int = -73;
/// Unable to open the database file
pub const UNQLITE_CANTOPEN: c_int = -74;
/// Read only Key/Value storage engine
pub const UNQLITE_READ_ONLY: c_int = -75;
/// Locking protocol error
pub const UNQLITE_LOCKERR: c_int = -76;
// end-of-error-codes

pub const UNQLITE_CONFIG_JX9_ERR_LOG: c_int = 1;
pub const UNQLITE_CONFIG_MAX_PAGE_CACHE: c_int = 2;
pub const UNQLITE_CONFIG_ERR_LOG: c_int = 3;
pub const UNQLITE_CONFIG_KV_ENGINE: c_int = 4;
pub const UNQLITE_CONFIG_DISABLE_AUTO_COMMIT: c_int = 5;
pub const UNQLITE_CONFIG_GET_KV_NAME: c_int = 6;

// UnQLite/Jx9 Virtual Machine Configuration Commands.
//
// The following set of constants are the available configuration verbs that can
// be used by the host-application to configure the Jx9 (Via UnQLite) Virtual machine.
// These constants must be passed as the second argument to the [unqlite_vm_config()]
// interface.
// Each options require a variable number of arguments.
// The [unqlite_vm_config()] interface will return UNQLITE_OK on success, any other return
// value indicates failure.
// There are many options but the most importants are: UNQLITE_VM_CONFIG_OUTPUT which install
// a VM output consumer callback, UNQLITE_VM_CONFIG_HTTP_REQUEST which parse and register
// a HTTP request and UNQLITE_VM_CONFIG_ARGV_ENTRY which populate the $argv array.
// For a full discussion on the configuration verbs and their expected parameters, please
// refer to this page:
//      http://unqlite.org/c_api/unqlite_vm_config.html
//
///  TWO ARGUMENTS: int (*xConsumer)(const void *, unsigned int, void *), void *
pub const UNQLITE_VM_CONFIG_OUTPUT: c_int = 1;
///  ONE ARGUMENT: const char *zIncludePath
pub const UNQLITE_VM_CONFIG_IMPORT_PATH: c_int = 2;
///  NO ARGUMENTS: Report all run-time errors in the VM output
pub const UNQLITE_VM_CONFIG_ERR_REPORT: c_int = 3;
///  ONE ARGUMENT: int nMaxDepth
pub const UNQLITE_VM_CONFIG_RECURSION_DEPTH: c_int = 4;
///  ONE ARGUMENT: unsigned int *pLength
pub const UNQLITE_VM_OUTPUT_LENGTH: c_int = 5;
///  TWO ARGUMENTS: const char *zName, unqlite_value *pValue
pub const UNQLITE_VM_CONFIG_CREATE_VAR: c_int = 6;
///  TWO ARGUMENTS: const char *zRawRequest, int nRequestLength
pub const UNQLITE_VM_CONFIG_HTTP_REQUEST: c_int = 7;
///  THREE ARGUMENTS: const char *zKey, const char *zValue, int nLen
pub const UNQLITE_VM_CONFIG_SERVER_ATTR: c_int = 8;
///  THREE ARGUMENTS: const char *zKey, const char *zValue, int nLen
pub const UNQLITE_VM_CONFIG_ENV_ATTR: c_int = 9;
///  ONE ARGUMENT: unqlite_value **ppValue
pub const UNQLITE_VM_CONFIG_EXEC_VALUE: c_int = 10;
///  ONE ARGUMENT: const unqlite_io_stream *pStream
pub const UNQLITE_VM_CONFIG_IO_STREAM: c_int = 11;
///  ONE ARGUMENT: const char *zValue
pub const UNQLITE_VM_CONFIG_ARGV_ENTRY: c_int = 12;
///  TWO ARGUMENTS: const void **ppOut, unsigned int *pOutputLen
pub const UNQLITE_VM_CONFIG_EXTRACT_OUTPUT: c_int = 13;

// Storage engine configuration commands.
//
// The following set of constants are the available configuration verbs that can
// be used by the host-application to configure the underlying storage engine
// (i.e Hash, B+tree, R+tree).
//
// These constants must be passed as the first argument to [unqlite_kv_config()].
// Each options require a variable number of arguments.
// The [unqlite_kv_config()] interface will return UNQLITE_OK on success, any other return
// value indicates failure.
// For a full discussion on the configuration verbs and their expected parameters, please
// refer to this page:
//      http://unqlite.org/c_api/unqlite_kv_config.html
//
///  ONE ARGUMENT: unsigned int (*xHash)(const void *,unsigned int)
pub const UNQLITE_KV_CONFIG_HASH_FUNC: c_int = 1;
///  ONE ARGUMENT: int (*xCmp)(const void *,const void *,unsigned int)
pub const UNQLITE_KV_CONFIG_CMP_FUNC: c_int = 2;

// Global Library Configuration Commands.
//
// The following set of constants are the available configuration verbs that can
// be used by the host-application to configure the whole library.
// These constants must be passed as the first argument to [unqlite_lib_config()].
//
// Each options require a variable number of arguments.
// The [unqlite_lib_config()] interface will return UNQLITE_OK on success, any other return
// value indicates failure.
// Notes:
// The default configuration is recommended for most applications and so the call to
// [unqlite_lib_config()] is usually not necessary. It is provided to support rare
// applications with unusual needs.
// The [unqlite_lib_config()] interface is not threadsafe. The application must insure that
// no other [unqlite_*()] interfaces are invoked by other threads while [unqlite_lib_config()]
// is running. Furthermore, [unqlite_lib_config()] may only be invoked prior to library
// initialization using [unqlite_lib_init()] or [unqlite_init()] or after shutdown
// by [unqlite_lib_shutdown()]. If [unqlite_lib_config()] is called after [unqlite_lib_init()]
// or [unqlite_init()] and before [unqlite_lib_shutdown()] then it will return UNQLITE_LOCKED.
// For a full discussion on the configuration verbs and their expected parameters, please
// refer to this page:
//      http://unqlite.org/c_api/unqlite_lib.html
//
///  ONE ARGUMENT: const SyMemMethods *pMemMethods
pub const UNQLITE_LIB_CONFIG_USER_MALLOC: c_int = 1;
///  TWO ARGUMENTS: int (*xMemError)(void *), void *pUserData
pub const UNQLITE_LIB_CONFIG_MEM_ERR_CALLBACK: c_int = 2;
///  ONE ARGUMENT: const SyMutexMethods *pMutexMethods
pub const UNQLITE_LIB_CONFIG_USER_MUTEX: c_int = 3;
///  NO ARGUMENTS
pub const UNQLITE_LIB_CONFIG_THREAD_LEVEL_SINGLE: c_int = 4;
///  NO ARGUMENTS
pub const UNQLITE_LIB_CONFIG_THREAD_LEVEL_MULTI: c_int = 5;
///  ONE ARGUMENT: const unqlite_vfs *pVfs
pub const UNQLITE_LIB_CONFIG_VFS: c_int = 6;
///  ONE ARGUMENT: unqlite_kv_methods *pStorage
pub const UNQLITE_LIB_CONFIG_STORAGE_ENGINE: c_int = 7;
///  ONE ARGUMENT: int iPageSize
pub const UNQLITE_LIB_CONFIG_PAGE_SIZE: c_int = 8;

// These bit values are intended for use in the 3rd parameter to the [unqlite_open()] interface
// and in the 4th parameter to the xOpen method of the [unqlite_vfs] object.
//
///  Read only mode. Ok for [unqlite_open]
pub const UNQLITE_OPEN_READONLY: c_uint = 0x00000001;
///  Ok for [unqlite_open]
pub const UNQLITE_OPEN_READWRITE: c_uint = 0x00000002;
///  Ok for [unqlite_open]
pub const UNQLITE_OPEN_CREATE: c_uint = 0x00000004;
///  VFS only
pub const UNQLITE_OPEN_EXCLUSIVE: c_uint = 0x00000008;
///  VFS only
pub const UNQLITE_OPEN_TEMP_DB: c_uint = 0x00000010;
///  Ok for [unqlite_open]
pub const UNQLITE_OPEN_NOMUTEX: c_uint = 0x00000020;
///  Omit journaling for this database. Ok for [unqlite_open]
pub const UNQLITE_OPEN_OMIT_JOURNALING: c_uint = 0x00000040;
///  An in memory database. Ok for [unqlite_open]
pub const UNQLITE_OPEN_IN_MEMORY: c_uint = 0x00000080;
///  Obtain a memory view of the whole file. Ok for [unqlite_open]
pub const UNQLITE_OPEN_MMAP: c_uint = 0x00000100;
// Synchronization Type Flags
//
// When UnQLite invokes the xSync() method of an [unqlite_io_methods] object it uses
// a combination of these integer values as the second argument.
//
// When the UNQLITE_SYNC_DATAONLY flag is used, it means that the sync operation only
// needs to flush data to mass storage.: c_int = Inode information need not be flushed.
// If the lower four bits of the flag equal UNQLITE_SYNC_NORMAL, that means to use normal
// fsync() semantics. If the lower four bits equal UNQLITE_SYNC_FULL, that means to use
// Mac OS X style fullsync instead of fsync().
//
pub const UNQLITE_SYNC_NORMAL: c_int = 0x00002;
pub const UNQLITE_SYNC_FULL: c_int = 0x00003;
pub const UNQLITE_SYNC_DATAONLY: c_int = 0x00010;
// File Locking Levels
//
// UnQLite uses one of these integer values as the second
// argument to calls it makes to the xLock() and xUnlock() methods
// of an [unqlite_io_methods] object.
//
pub const UNQLITE_LOCK_NONE: c_int = 0;
pub const UNQLITE_LOCK_SHARED: c_int = 1;
pub const UNQLITE_LOCK_RESERVED: c_int = 2;
pub const UNQLITE_LOCK_PENDING: c_int = 3;
pub const UNQLITE_LOCK_EXCLUSIVE: c_int = 4;

// Flags for the xAccess VFS method
//
// These integer constants can be used as the third parameter to
// the xAccess method of an [unqlite_vfs] object.  They determine
// what kind of permissions the xAccess method is looking for.
// With UNQLITE_ACCESS_EXISTS, the xAccess method
// simply checks whether the file exists.
// With UNQLITE_ACCESS_READWRITE, the xAccess method
// checks whether the named directory is both readable and writable
// (in other words, if files can be added, removed, and renamed within
// the directory).
// The UNQLITE_ACCESS_READWRITE constant is currently used only by the
// [temp_store_directory pragma], though this could change in a future
// release of UnQLite.
// With UNQLITE_ACCESS_READ, the xAccess method
// checks whether the file is readable.  The UNQLITE_ACCESS_READ constant is
// currently unused, though it might be used in a future release of
// UnQLite.
//
pub const UNQLITE_ACCESS_EXISTS: c_int = 0;
pub const UNQLITE_ACCESS_READWRITE: c_int = 1;
pub const UNQLITE_ACCESS_READ: c_int = 2;
// Possible seek positions.
//
pub const UNQLITE_CURSOR_MATCH_EXACT: c_int = 1;
pub const UNQLITE_CURSOR_MATCH_LE: c_int = 2;
pub const UNQLITE_CURSOR_MATCH_GE: c_int = 3;
// UnQLite journal file suffix.
//
// #ifndef UNQLITE_JOURNAL_FILE_SUFFIX
pub const UNQLITE_JOURNAL_FILE_SUFFIX: &'static str = "_unqlite_journal";
// #endif
//
// Call Context - Error Message Serverity Level.
//
// The following constans are the allowed severity level that can
// passed as the second argument to the [unqlite_context_throw_error()] or
// [unqlite_context_throw_error_format()] interfaces.
// Refer to the official documentation for additional information.
//

///  Call context error such as unexpected number of arguments, invalid types and so on.
pub const UNQLITE_CTX_ERR: c_int = 1;
///  Call context Warning
pub const UNQLITE_CTX_WARNING: c_int = 2;
///  Call context Notice
pub const UNQLITE_CTX_NOTICE: c_int = 3;
