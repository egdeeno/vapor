use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn vapor_blf_enc(_: *mut blf_ctx, _: *mut u_int32_t, _: u_int16_t);
    fn Vapor_Blowfish_stream2word(
        _: *const u_int8_t,
        _: u_int16_t,
        _: *mut u_int16_t,
    ) -> u_int32_t;
    fn Vapor_Blowfish_expand0state(_: *mut blf_ctx, _: *const u_int8_t, _: u_int16_t);
    fn Vapor_Blowfish_expandstate(
        _: *mut blf_ctx,
        _: *const u_int8_t,
        _: u_int16_t,
        _: *const u_int8_t,
        _: u_int16_t,
    );
    fn Vapor_Blowfish_initstate(_: *mut blf_ctx);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
}

pub type size_t = libc::c_ulong;
pub type u_int8_t = libc::c_uchar;
pub type u_int16_t = libc::c_ushort;
pub type u_int32_t = libc::c_uint;
pub type blf_ctx = BlowfishContext;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlowfishContext {
    pub S: [[u_int32_t; 256]; 4],
    pub P: [u_int32_t; 18],
}
pub const _ISdigit: C2RustUnnamed = 2048;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;

#[no_mangle]
pub unsafe extern "C" fn vapor_bcrypt_hashpass(
    mut key: *const libc::c_char,
    mut salt: *const libc::c_char,
    mut encrypted: *mut libc::c_char,
    mut encryptedlen: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut state: blf_ctx = blf_ctx {
        S: [[0; 256]; 4],
        P: [0; 18],
    };
    let mut rounds: u_int32_t = 0;
    let mut i: u_int32_t = 0;
    let mut k: u_int32_t = 0;
    let mut j: u_int16_t = 0;
    let mut key_len: size_t = 0;
    let mut salt_len: u_int8_t = 0;
    let mut logr: u_int8_t = 0;
    let mut minor: u_int8_t = 0;
    let mut ciphertext: [u_int8_t; 24] = *::core::mem::transmute::<
        &[u8; 24],
        &mut [u_int8_t; 24],
    >(b"OrpheanBeholderScryDoubt");
    let mut csalt: [u_int8_t; 16] = [0; 16];
    let mut cdata: [u_int32_t; 6] = [0; 6];
    if !(encryptedlen < 61) {
        if !(*salt.offset(0) as libc::c_int != '$' as i32) {
            salt = salt.offset(1);
            if !(*salt.offset(0) as libc::c_int != '2' as i32) {
                minor = *salt.offset(1) as u_int8_t;
                match minor as libc::c_int {
                    97 => {
                        current_block = 15122898966093540124;
                        match current_block {
                            15122898966093540124 => {
                                key_len = (strlen(key))
                                    .wrapping_add(1) as u_int8_t
                                    as size_t;
                            }
                            _ => {
                                key_len = strlen(key);
                                if key_len > 72 as libc::c_int as libc::c_ulong {
                                    key_len = 72 as libc::c_int as size_t;
                                }
                                key_len = key_len.wrapping_add(1);
                            }
                        }
                        if !(*salt.offset(2 as libc::c_int as isize) as libc::c_int
                            != '$' as i32)
                        {
                            salt = salt.offset(3);
                            if !(*(*__ctype_b_loc())
                                .offset(
                                    *salt.offset(0) as libc::c_uchar
                                        as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                == 0
                                || *(*__ctype_b_loc())
                                    .offset(
                                        *salt.offset(1) as libc::c_uchar
                                            as libc::c_int as isize,
                                    ) as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0
                                || *salt.offset(2) as libc::c_int
                                    != '$' as i32)
                            {
                                logr = (*salt.offset(1)
                                    as libc::c_int - '0' as i32
                                    + (*salt.offset(0) as libc::c_int
                                        - '0' as i32) * 10 as libc::c_int) as u_int8_t;
                                if !((logr as libc::c_int) < 4 as libc::c_int
                                    || logr as libc::c_int > 31 as libc::c_int)
                                {
                                    rounds = (1 as libc::c_uint) << logr as libc::c_int;
                                    salt = salt.offset(3);
                                    if !((strlen(salt))
                                        .wrapping_mul(3)
                                        .wrapping_div(4)
                                        < 16)
                                    {
                                        if !(decode_base64(
                                            csalt.as_mut_ptr(),
                                            16,
                                            salt,
                                        ) != 0)
                                        {
                                            salt_len = 16;
                                            Vapor_Blowfish_initstate(&mut state);
                                            Vapor_Blowfish_expandstate(
                                                &mut state,
                                                csalt.as_mut_ptr(),
                                                salt_len as u_int16_t,
                                                key as *mut u_int8_t,
                                                key_len as u_int16_t,
                                            );
                                            k = 0;
                                            while k < rounds {
                                                Vapor_Blowfish_expand0state(
                                                    &mut state,
                                                    key as *mut u_int8_t,
                                                    key_len as u_int16_t,
                                                );
                                                Vapor_Blowfish_expand0state(
                                                    &mut state,
                                                    csalt.as_mut_ptr(),
                                                    salt_len as u_int16_t,
                                                );
                                                k = k.wrapping_add(1);
                                            }
                                            j = 0;
                                            i = 0;
                                            while i < 6 {
                                                cdata[i
                                                    as usize] = Vapor_Blowfish_stream2word(
                                                    ciphertext.as_mut_ptr(),
                                                    (4 as libc::c_int * 6 as libc::c_int) as u_int16_t,
                                                    &mut j,
                                                );
                                                i = i.wrapping_add(1);
                                            }
                                            k = 0;
                                            while k < 64 {
                                                vapor_blf_enc(
                                                    &mut state,
                                                    cdata.as_mut_ptr(),
                                                    (6/2) as u_int16_t,
                                                );
                                                k = k.wrapping_add(1);
                                            }
                                            i = 0;
                                            while i < 6 as libc::c_int as libc::c_uint {
                                                ciphertext[(4 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(i)
                                                    .wrapping_add(3)
                                                    as usize] = (cdata[i as usize]
                                                    & 0xff as libc::c_int as libc::c_uint) as u_int8_t;
                                                cdata[i as usize] = cdata[i as usize] >> 8 as libc::c_int;
                                                ciphertext[(4 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(i)
                                                    .wrapping_add(2 as libc::c_int as libc::c_uint)
                                                    as usize] = (cdata[i as usize]
                                                    & 0xff as libc::c_int as libc::c_uint) as u_int8_t;
                                                cdata[i as usize] = cdata[i as usize] >> 8 as libc::c_int;
                                                ciphertext[(4 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(i)
                                                    .wrapping_add(1)
                                                    as usize] = (cdata[i as usize]
                                                    & 0xff as libc::c_int as libc::c_uint) as u_int8_t;
                                                cdata[i as usize] = cdata[i as usize] >> 8 as libc::c_int;
                                                ciphertext[(4 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(i)
                                                    .wrapping_add(0)
                                                    as usize] = (cdata[i as usize]
                                                    & 0xff as libc::c_int as libc::c_uint) as u_int8_t;
                                                i = i.wrapping_add(1);
                                            }
                                            snprintf(
                                                encrypted,
                                                8,
                                                b"$2%c$%2.2u$\0" as *const u8 as *const libc::c_char,
                                                minor as libc::c_int,
                                                logr as libc::c_int,
                                            );
                                            vapor_encode_base64(
                                                encrypted.offset(7),
                                                csalt.as_mut_ptr(),
                                                16,
                                            );
                                            vapor_encode_base64(
                                                encrypted
                                                    .offset(7)
                                                    .offset(22),
                                                ciphertext.as_mut_ptr(),
                                                4 * 6 - 1,
                                            );
                                            memset(
                                                &mut state as *mut blf_ctx as *mut libc::c_void,
                                                0,
                                                ::core::mem::size_of::<blf_ctx>() as libc::c_ulong,
                                            );
                                            memset(
                                                ciphertext.as_mut_ptr() as *mut libc::c_void,
                                                0,
                                                ::core::mem::size_of::<[u_int8_t; 24]>() as libc::c_ulong,
                                            );
                                            memset(
                                                csalt.as_mut_ptr() as *mut libc::c_void,
                                                0,
                                                ::core::mem::size_of::<[u_int8_t; 16]>() as libc::c_ulong,
                                            );
                                            memset(
                                                cdata.as_mut_ptr() as *mut libc::c_void,
                                                0,
                                                ::core::mem::size_of::<[u_int32_t; 6]>() as libc::c_ulong,
                                            );
                                            return 0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    98 => {
                        current_block = 10500237484492514752;
                        match current_block {
                            15122898966093540124 => {
                                key_len = (strlen(key))
                                    .wrapping_add(1) as u_int8_t
                                    as size_t;
                            }
                            _ => {
                                key_len = strlen(key);
                                if key_len > 72 {
                                    key_len = 72;
                                }
                                key_len = key_len.wrapping_add(1);
                            }
                        }
                        if !(*salt.offset(2) as libc::c_int
                            != '$' as i32)
                        {
                            salt = salt.offset(3);
                            if !(*(*__ctype_b_loc())
                                .offset(
                                    *salt.offset(0) as libc::c_uchar as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                == 0
                                || *(*__ctype_b_loc())
                                    .offset(
                                        *salt.offset(1) as libc::c_uchar as libc::c_int as isize,
                                    ) as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0
                                || *salt.offset(2) as libc::c_int
                                    != '$' as i32)
                            {
                                logr = (*salt.offset(1)
                                    as libc::c_int - '0' as i32
                                    + (*salt.offset(0) as libc::c_int
                                        - '0' as i32) * 10 as libc::c_int) as u_int8_t;
                                if !((logr as libc::c_int) < 4 as libc::c_int
                                    || logr as libc::c_int > 31 as libc::c_int)
                                {
                                    rounds = (1) << logr as libc::c_int;
                                    salt = salt.offset(3);
                                    if !((strlen(salt))
                                        .wrapping_mul(3)
                                        .wrapping_div(4)
                                        < 16)
                                    {
                                        if !(decode_base64(
                                            csalt.as_mut_ptr(),
                                            16,
                                            salt,
                                        ) != 0)
                                        {
                                            salt_len = 16;
                                            Vapor_Blowfish_initstate(&mut state);
                                            Vapor_Blowfish_expandstate(
                                                &mut state,
                                                csalt.as_mut_ptr(),
                                                salt_len as u_int16_t,
                                                key as *mut u_int8_t,
                                                key_len as u_int16_t,
                                            );
                                            k = 0;
                                            while k < rounds {
                                                Vapor_Blowfish_expand0state(
                                                    &mut state,
                                                    key as *mut u_int8_t,
                                                    key_len as u_int16_t,
                                                );
                                                Vapor_Blowfish_expand0state(
                                                    &mut state,
                                                    csalt.as_mut_ptr(),
                                                    salt_len as u_int16_t,
                                                );
                                                k = k.wrapping_add(1);
                                            }
                                            j = 0;
                                            i = 0;
                                            while i < 6 {
                                                cdata[i
                                                    as usize] = Vapor_Blowfish_stream2word(
                                                    ciphertext.as_mut_ptr(),
                                                    (4 * 6) as u_int16_t,
                                                    &mut j,
                                                );
                                                i = i.wrapping_add(1);
                                            }
                                            k = 0;
                                            while k < 64 {
                                                vapor_blf_enc(
                                                    &mut state,
                                                    cdata.as_mut_ptr(),
                                                    (6 as libc::c_int / 2 as libc::c_int) as u_int16_t,
                                                );
                                                k = k.wrapping_add(1);
                                            }
                                            i = 0;
                                            while i < 6 {
                                                ciphertext[(4 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(i)
                                                    .wrapping_add(3)
                                                    as usize] = (cdata[i as usize]
                                                    & 0xff as libc::c_int as libc::c_uint) as u_int8_t;
                                                cdata[i as usize] = cdata[i as usize] >> 8 as libc::c_int;
                                                ciphertext[(4 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(i)
                                                    .wrapping_add(2)
                                                    as usize] = (cdata[i as usize]
                                                    & 0xff as libc::c_int as libc::c_uint) as u_int8_t;
                                                cdata[i as usize] = cdata[i as usize] >> 8 as libc::c_int;
                                                ciphertext[(4 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(i)
                                                    .wrapping_add(1)
                                                    as usize] = (cdata[i as usize]
                                                    & 0xff as libc::c_int as libc::c_uint) as u_int8_t;
                                                cdata[i as usize] = cdata[i as usize] >> 8 as libc::c_int;
                                                ciphertext[(4 as libc::c_int as libc::c_uint)
                                                    .wrapping_mul(i)
                                                    .wrapping_add(0)
                                                    as usize] = (cdata[i as usize]
                                                    & 0xff as libc::c_int as libc::c_uint) as u_int8_t;
                                                i = i.wrapping_add(1);
                                            }
                                            snprintf(
                                                encrypted,
                                                8 as libc::c_int as libc::c_ulong,
                                                b"$2%c$%2.2u$\0" as *const u8 as *const libc::c_char,
                                                minor as libc::c_int,
                                                logr as libc::c_int,
                                            );
                                            vapor_encode_base64(
                                                encrypted.offset(7 as libc::c_int as isize),
                                                csalt.as_mut_ptr(),
                                                16,
                                            );
                                            vapor_encode_base64(
                                                encrypted
                                                    .offset(7 as libc::c_int as isize)
                                                    .offset(22 as libc::c_int as isize),
                                                ciphertext.as_mut_ptr(),
                                                (4 as libc::c_int * 6 as libc::c_int - 1 as libc::c_int)
                                                    as size_t,
                                            );
                                            memset(
                                                &mut state as *mut blf_ctx as *mut libc::c_void,
                                                0 as libc::c_int,
                                                ::core::mem::size_of::<blf_ctx>() as libc::c_ulong,
                                            );
                                            memset(
                                                ciphertext.as_mut_ptr() as *mut libc::c_void,
                                                0 as libc::c_int,
                                                ::core::mem::size_of::<[u_int8_t; 24]>() as libc::c_ulong,
                                            );
                                            memset(
                                                csalt.as_mut_ptr() as *mut libc::c_void,
                                                0 as libc::c_int,
                                                ::core::mem::size_of::<[u_int8_t; 16]>() as libc::c_ulong,
                                            );
                                            memset(
                                                cdata.as_mut_ptr() as *mut libc::c_void,
                                                0 as libc::c_int,
                                                ::core::mem::size_of::<[u_int32_t; 6]>() as libc::c_ulong,
                                            );
                                            return 0 as libc::c_int;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    *__errno_location() = 22 as libc::c_int;
    return -1;
}

static mut Base64Code: [u_int8_t; 65] = unsafe {
    *::core::mem::transmute::<
        &[u8; 65],
        &[u_int8_t; 65],
    >(b"./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\0")
};
static mut index_64: [u_int8_t; 128] = [
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    0,
    1,
    54,
    55,
    56,
    57,
    58,
    59,
    60,
    61,
    62,
    63,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    255,
    255,
    255,
    255,
    255,
    255,
    28,
    29,
    30,
    31,
    32,
    33,
    34,
    35,
    36,
    37,
    38,
    39,
    40,
    41,
    42,
    43,
    44,
    45,
    46,
    47,
    48,
    49,
    50,
    51,
    52,
    53,
    255,
    255,
    255,
    255,
    255,
];

unsafe extern "C" fn decode_base64(
    mut buffer: *mut u_int8_t,
    mut len: size_t,
    mut b64data: *const libc::c_char,
) -> libc::c_int {
    let mut bp: *mut u_int8_t = buffer;
    let mut p: *const u_int8_t = b64data as *mut u_int8_t;
    let mut c1: u_int8_t = 0;
    let mut c2: u_int8_t = 0;
    let mut c3: u_int8_t = 0;
    let mut c4: u_int8_t = 0;
    while bp < buffer.offset(len as isize) {
        c1 = (if *p as libc::c_int > 127 as libc::c_int {
            255 as libc::c_int
        } else {
            index_64[*p as usize] as libc::c_int
        }) as u_int8_t;
        if c1 as libc::c_int == 255 as libc::c_int {
            return -(1 as libc::c_int);
        }
        c2 = (if *p.offset(1) as libc::c_int > 127 as libc::c_int
        {
            255 as libc::c_int
        } else {
            index_64[*p.offset(1) as usize] as libc::c_int
        }) as u_int8_t;
        if c2 as libc::c_int == 255 as libc::c_int {
            return -(1 as libc::c_int);
        }
        let fresh0 = bp;
        bp = bp.offset(1);
        *fresh0 = ((c1 as libc::c_int) << 2 as libc::c_int
            | (c2 as libc::c_int & 0x30 as libc::c_int) >> 4 as libc::c_int) as u_int8_t;
        if bp >= buffer.offset(len as isize) {
            break;
        }
        c3 = (if *p.offset(2) as libc::c_int > 127 as libc::c_int
        {
            255 as libc::c_int
        } else {
            index_64[*p.offset(2) as usize] as libc::c_int
        }) as u_int8_t;
        if c3 as libc::c_int == 255 as libc::c_int {
            return -(1 as libc::c_int);
        }
        let fresh1 = bp;
        bp = bp.offset(1);
        *fresh1 = ((c2 as libc::c_int & 0xf as libc::c_int) << 4 as libc::c_int
            | (c3 as libc::c_int & 0x3c as libc::c_int) >> 2 as libc::c_int) as u_int8_t;
        if bp >= buffer.offset(len as isize) {
            break;
        }
        c4 = (if *p.offset(3) as libc::c_int > 127 as libc::c_int
        {
            255 as libc::c_int
        } else {
            index_64[*p.offset(3) as usize] as libc::c_int
        }) as u_int8_t;
        if c4 as libc::c_int == 255 as libc::c_int {
            return -(1 as libc::c_int);
        }
        let fresh2 = bp;
        bp = bp.offset(1);
        *fresh2 = ((c3 as libc::c_int & 0x3 as libc::c_int) << 6 as libc::c_int
            | c4 as libc::c_int) as u_int8_t;
        p = p.offset(4);
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn vapor_encode_base64(
    mut b64buffer: *mut libc::c_char,
    mut data: *const u_int8_t,
    mut len: size_t,
) -> libc::c_int {
    let mut bp: *mut u_int8_t = b64buffer as *mut u_int8_t;
    let mut p: *const u_int8_t = data;
    let mut c1: u_int8_t = 0;
    let mut c2: u_int8_t = 0;
    while p < data.offset(len as isize) {
        let fresh3 = p;
        p = p.offset(1);
        c1 = *fresh3;
        let fresh4 = bp;
        bp = bp.offset(1);
        *fresh4 = Base64Code[(c1 as libc::c_int >> 2 as libc::c_int) as usize];
        c1 = ((c1 as libc::c_int & 0x3 as libc::c_int) << 4 as libc::c_int) as u_int8_t;
        if p >= data.offset(len as isize) {
            let fresh5 = bp;
            bp = bp.offset(1);
            *fresh5 = Base64Code[c1 as usize];
            break;
        } else {
            let fresh6 = p;
            p = p.offset(1);
            c2 = *fresh6;
            c1 = (c1 as libc::c_int
                | c2 as libc::c_int >> 4 as libc::c_int & 0xf as libc::c_int)
                as u_int8_t;
            let fresh7 = bp;
            bp = bp.offset(1);
            *fresh7 = Base64Code[c1 as usize];
            c1 = ((c2 as libc::c_int & 0xf as libc::c_int) << 2 as libc::c_int)
                as u_int8_t;
            if p >= data.offset(len as isize) {
                let fresh8 = bp;
                bp = bp.offset(1);
                *fresh8 = Base64Code[c1 as usize];
                break;
            } else {
                let fresh9 = p;
                p = p.offset(1);
                c2 = *fresh9;
                c1 = (c1 as libc::c_int
                    | c2 as libc::c_int >> 6 as libc::c_int & 0x3 as libc::c_int)
                    as u_int8_t;
                let fresh10 = bp;
                bp = bp.offset(1);
                *fresh10 = Base64Code[c1 as usize];
                let fresh11 = bp;
                bp = bp.offset(1);
                *fresh11 = Base64Code[(c2 as libc::c_int & 0x3f as libc::c_int)
                    as usize];
            }
        }
    }
    *bp = '\0' as i32 as u_int8_t;
    return 0 as libc::c_int;
}
