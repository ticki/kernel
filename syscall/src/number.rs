pub const SYS_CLASS: usize =    0xF000_0000;
pub const SYS_CLASS_PATH: usize=0x1000_0000;
pub const SYS_CLASS_FILE: usize=0x2000_0000;

pub const SYS_ARG: usize =      0x0F00_0000;
pub const SYS_ARG_SLICE: usize =0x0100_0000;
pub const SYS_ARG_MSLICE: usize=0x0200_0000;
pub const SYS_ARG_PATH: usize = 0x0300_0000;

pub const SYS_RET: usize =      0x00F0_0000;
pub const SYS_RET_FILE: usize = 0x0010_0000;

pub const SYS_LINK: usize =     SYS_CLASS_PATH | SYS_ARG_PATH | 9;
pub const SYS_OPEN: usize =     SYS_CLASS_PATH | SYS_RET_FILE | 5;
pub const SYS_MKDIR: usize =    SYS_CLASS_PATH | 39;
pub const SYS_RMDIR: usize =    SYS_CLASS_PATH | 84;
pub const SYS_UNLINK: usize =   SYS_CLASS_PATH | 10;

pub const SYS_CLOSE: usize =    SYS_CLASS_FILE | 6;
pub const SYS_DUP: usize =      SYS_CLASS_FILE | SYS_RET_FILE | 41;
pub const SYS_READ: usize =     SYS_CLASS_FILE | SYS_ARG_MSLICE | 3;
pub const SYS_WRITE: usize =    SYS_CLASS_FILE | SYS_ARG_SLICE | 4;
pub const SYS_FEVENT: usize =   SYS_CLASS_FILE | 927;
pub const SYS_LSEEK: usize =    SYS_CLASS_FILE | 19;
pub const SYS_FPATH: usize =    SYS_CLASS_FILE | SYS_ARG_MSLICE | 928;
pub const SYS_FSTAT: usize =    SYS_CLASS_FILE | SYS_ARG_MSLICE | 28;
pub const SYS_FSYNC: usize =    SYS_CLASS_FILE | 118;
pub const SYS_FTRUNCATE: usize =SYS_CLASS_FILE | 93;

pub const SYS_BRK: usize =      45;
pub const SYS_CHDIR: usize =    12;
pub const SYS_CLOCK_GETTIME: usize = 265;
pub const SYS_CLONE: usize =    120;
pub const SYS_EXECVE: usize =   11;
pub const SYS_EXIT: usize =     1;
pub const SYS_FUTEX: usize =    240;
pub const SYS_GETCWD: usize =   183;
pub const SYS_GETEGID: usize =  202;
pub const SYS_GETEUID: usize =  201;
pub const SYS_GETGID: usize =   200;
pub const SYS_GETPID: usize =   20;
pub const SYS_GETUID: usize =   199;
pub const SYS_IOPL: usize =     110;
pub const SYS_NANOSLEEP: usize =162;
pub const SYS_PHYSALLOC: usize =945;
pub const SYS_PHYSFREE: usize = 946;
pub const SYS_PHYSMAP: usize =  947;
pub const SYS_PHYSUNMAP: usize =948;
pub const SYS_VIRTTOPHYS: usize=949;
pub const SYS_PIPE2: usize =    331;
pub const SYS_SETGID: usize =   214;
pub const SYS_SETUID: usize =   213;
pub const SYS_WAITPID: usize =  7;
pub const SYS_YIELD: usize =    158;
