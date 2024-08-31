pub type c_char = u8;
pub type c_long = i32;
pub type c_ulong = u32;
pub type wchar_t = i32;
pub type time_t = i64;
pub type suseconds_t = i32;
pub type register_t = i32;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_size: ::off_t,
        pub st_blocks: ::blkcnt_t,
        pub st_blksize: ::blksize_t,
        pub st_flags: ::fflags_t,
        pub st_gen: u32,
        pub st_lspare: i32,
        pub st_birthtime: ::time_t,
        pub st_birthtime_nsec: ::c_long,
    }
}

s_no_extra_traits! {
    #[repr(align(16))]
    pub struct mcontext_t {
        pub mc_vers: ::c_int,
        pub mc_flags: ::c_int,
        pub mc_onstack: ::c_int,
        pub mc_len: ::c_int,
        pub mc_avec: [u64; 64],
        pub mc_av: [u32; 2],
        pub mc_frame: [::register_t; 42],
        pub mc_fpreg: [u64; 33],
        pub mc_vsxfpreg: [u64; 32],
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for mcontext_t {
            fn eq(&self, other: &mcontext_t) -> bool {
                self.mc_vers == other.mc_vers &&
                self.mc_flags == other.mc_flags &&
                self.mc_onstack == other.mc_onstack &&
                self.mc_len == other.mc_len &&
                self.mc_avec == other.mc_avec &&
                self.mc_av == other.mc_av &&
                self.mc_frame == other.mc_frame &&
                self.mc_fpreg == other.mc_fpreg &&
                self.mc_vsxfpreg == other.mc_vsxfpreg
            }
        }
        impl Eq for mcontext_t {}
        impl ::fmt::Debug for mcontext_t {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("mcontext_t")
                    .field("mc_vers", &self.mc_vers)
                    .field("mc_flags", &self.mc_flags)
                    .field("mc_onstack", &self.mc_onstack)
                    .field("mc_len", &self.mc_len)
                    .field("mc_avec", &self.mc_avec)
                    .field("mc_av", &self.mc_av)
                    .field("mc_frame", &self.mc_frame)
                    .field("mc_fpreg", &self.mc_fpreg)
                    .field("mc_vsxfpreg", &self.mc_vsxfpreg)
                    .finish()
            }
        }
        impl ::hash::Hash for mcontext_t {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.mc_vers.hash(state);
                self.mc_flags.hash(state);
                self.mc_onstack.hash(state);
                self.mc_len.hash(state);
                self.mc_avec.hash(state);
                self.mc_av.hash(state);
                self.mc_frame.hash(state);
                self.mc_fpreg.hash(state);
                self.mc_vsxfpreg.hash(state);
            }
        }
    }
}

pub(crate) const _ALIGNBYTES: usize = ::mem::size_of::<::c_int>() - 1;
pub const MAP_32BIT: ::c_int = 0x00080000;
pub const MINSIGSTKSZ: ::size_t = 2048; // 512 * 4
