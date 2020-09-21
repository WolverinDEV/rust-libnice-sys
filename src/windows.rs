#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_in {
    __data: [u8; 4]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_in6 {
    __data: [u8; 16]
}

#[repr(C)]
pub union sockaddr_storage {
    v4: sockaddr_in6,
    v6: sockaddr_in6
}
pub type socklen_t = u32;
pub type SSIZE_T = i64;