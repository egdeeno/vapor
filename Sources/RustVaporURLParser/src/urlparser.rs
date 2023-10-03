use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type size_t = libc::c_ulong;
pub type vapor_urlparser_fields = libc::c_uint;
pub const UF_MAX: vapor_urlparser_fields = 7;
pub const UF_USERINFO: vapor_urlparser_fields = 6;
pub const UF_FRAGMENT: vapor_urlparser_fields = 5;
pub const UF_QUERY: vapor_urlparser_fields = 4;
pub const UF_PATH: vapor_urlparser_fields = 3;
pub const UF_PORT: vapor_urlparser_fields = 2;
pub const UF_HOST: vapor_urlparser_fields = 1;
pub const UF_SCHEMA: vapor_urlparser_fields = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vapor_urlparser_field_data {
    pub off: uint16_t,
    pub len: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vapor_urlparser_url {
    pub field_set: uint16_t,
    pub port: uint16_t,
    pub field_data: [vapor_urlparser_field_data; 7],
}
pub const s_http_userinfo_start: http_host_state = 2;
pub const s_http_userinfo: http_host_state = 3;
pub const s_http_host_port_start: http_host_state = 11;
pub const s_http_host_v6_zone: http_host_state = 10;
pub const s_http_host_v6_zone_start: http_host_state = 9;
pub const s_http_host_v6: http_host_state = 7;
pub const s_http_host_v6_start: http_host_state = 5;
pub const s_http_host_start: http_host_state = 4;
pub type http_host_state = libc::c_uint;
pub const s_http_host_port: http_host_state = 12;
pub const s_http_host_v6_end: http_host_state = 8;
pub const s_http_host: http_host_state = 6;
pub const s_http_host_dead: http_host_state = 1;
pub const s_req_fragment: state = 31;
pub const s_req_query_string: state = 29;
pub const s_req_path: state = 27;
pub const s_req_server: state = 25;
pub const s_req_server_with_at: state = 26;
pub const s_req_schema: state = 21;
pub const s_req_fragment_start: state = 30;
pub const s_req_query_string_start: state = 28;
pub const s_req_server_start: state = 24;
pub const s_req_schema_slash_slash: state = 23;
pub const s_req_schema_slash: state = 22;
pub const s_dead: state = 1;
pub type state = libc::c_uint;
pub const s_message_done: state = 64;
pub const s_body_identity_eof: state = 63;
pub const s_body_identity: state = 62;
pub const s_chunk_data_done: state = 61;
pub const s_chunk_data_almost_done: state = 60;
pub const s_chunk_data: state = 59;
pub const s_headers_done: state = 58;
pub const s_headers_almost_done: state = 57;
pub const s_chunk_size_almost_done: state = 56;
pub const s_chunk_parameters: state = 55;
pub const s_chunk_size: state = 54;
pub const s_chunk_size_start: state = 53;
pub const s_header_almost_done: state = 52;
pub const s_header_value_lws: state = 51;
pub const s_header_value: state = 50;
pub const s_header_value_start: state = 49;
pub const s_header_value_discard_lws: state = 48;
pub const s_header_value_discard_ws_almost_done: state = 47;
pub const s_header_value_discard_ws: state = 46;
pub const s_header_field: state = 45;
pub const s_header_field_start: state = 44;
pub const s_req_line_almost_done: state = 43;
pub const s_req_http_end: state = 42;
pub const s_req_http_minor: state = 41;
pub const s_req_http_dot: state = 40;
pub const s_req_http_major: state = 39;
pub const s_req_http_IC: state = 38;
pub const s_req_http_I: state = 37;
pub const s_req_http_HTTP: state = 36;
pub const s_req_http_HTT: state = 35;
pub const s_req_http_HT: state = 34;
pub const s_req_http_H: state = 33;
pub const s_req_http_start: state = 32;
pub const s_req_spaces_before_url: state = 20;
pub const s_req_method: state = 19;
pub const s_start_req: state = 18;
pub const s_res_line_almost_done: state = 17;
pub const s_res_status: state = 16;
pub const s_res_status_start: state = 15;
pub const s_res_status_code: state = 14;
pub const s_res_first_status_code: state = 13;
pub const s_res_http_end: state = 12;
pub const s_res_http_minor: state = 11;
pub const s_res_http_dot: state = 10;
pub const s_res_http_major: state = 9;
pub const s_res_HTTP: state = 8;
pub const s_res_HTT: state = 7;
pub const s_res_HT: state = 6;
pub const s_res_H: state = 5;
pub const s_start_res: state = 4;
pub const s_res_or_resp_H: state = 3;
pub const s_start_req_or_res: state = 2;
static mut normal_url_char: [uint8_t; 32] = [
    (0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int
        | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int)
        as uint8_t,
    (0 as libc::c_int | 2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int
        | 16 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int)
        as uint8_t,
    (0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int
        | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int)
        as uint8_t,
    (0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int
        | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int)
        as uint8_t,
    (0 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 0 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 0 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 0 as libc::c_int)
        as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn parse_url_char(mut s: state, ch: libc::c_char) -> state {
    if ch as libc::c_int == ' ' as i32 || ch as libc::c_int == '\r' as i32
        || ch as libc::c_int == '\n' as i32
    {
        return s_dead;
    }
    let mut current_block_58: u64;
    match s as libc::c_uint {
        20 => {
            if ch as libc::c_int == '/' as i32 || ch as libc::c_int == '*' as i32 {
                return s_req_path;
            }
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
            {
                return s_req_schema;
            }
            current_block_58 = 7420279277351916581;
        }
        21 => {
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
            {
                return s;
            }
            if ch as libc::c_int == ':' as i32 {
                return s_req_schema_slash;
            }
            current_block_58 = 7420279277351916581;
        }
        22 => {
            if ch as libc::c_int == '/' as i32 {
                return s_req_schema_slash_slash;
            }
            current_block_58 = 7420279277351916581;
        }
        23 => {
            if ch as libc::c_int == '/' as i32 {
                return s_req_server_start;
            }
            current_block_58 = 7420279277351916581;
        }
        26 => {
            if ch as libc::c_int == '@' as i32 {
                return s_dead;
            }
            current_block_58 = 7200080685983564597;
        }
        24 | 25 => {
            current_block_58 = 7200080685983564597;
        }
        27 => {
            if normal_url_char[(ch as libc::c_uchar as libc::c_uint >> 3 as libc::c_int)
                as usize] as libc::c_uint
                & ((1 as libc::c_int)
                    << (ch as libc::c_uchar as libc::c_uint
                        & 7 as libc::c_int as libc::c_uint)) as libc::c_uint != 0
                || ch as libc::c_int & 0x80 as libc::c_int != 0
            {
                return s;
            }
            match ch as libc::c_int {
                63 => return s_req_query_string_start,
                35 => return s_req_fragment_start,
                _ => {}
            }
            current_block_58 = 7420279277351916581;
        }
        28 | 29 => {
            if normal_url_char[(ch as libc::c_uchar as libc::c_uint >> 3 as libc::c_int)
                as usize] as libc::c_uint
                & ((1 as libc::c_int)
                    << (ch as libc::c_uchar as libc::c_uint
                        & 7 as libc::c_int as libc::c_uint)) as libc::c_uint != 0
                || ch as libc::c_int & 0x80 as libc::c_int != 0
            {
                return s_req_query_string;
            }
            match ch as libc::c_int {
                63 => return s_req_query_string,
                35 => return s_req_fragment_start,
                _ => {}
            }
            current_block_58 = 7420279277351916581;
        }
        30 => {
            if normal_url_char[(ch as libc::c_uchar as libc::c_uint >> 3 as libc::c_int)
                as usize] as libc::c_uint
                & ((1 as libc::c_int)
                    << (ch as libc::c_uchar as libc::c_uint
                        & 7 as libc::c_int as libc::c_uint)) as libc::c_uint != 0
                || ch as libc::c_int & 0x80 as libc::c_int != 0
            {
                return s_req_fragment;
            }
            match ch as libc::c_int {
                63 => return s_req_fragment,
                35 => return s,
                _ => {}
            }
            current_block_58 = 7420279277351916581;
        }
        31 => {
            if normal_url_char[(ch as libc::c_uchar as libc::c_uint >> 3 as libc::c_int)
                as usize] as libc::c_uint
                & ((1 as libc::c_int)
                    << (ch as libc::c_uchar as libc::c_uint
                        & 7 as libc::c_int as libc::c_uint)) as libc::c_uint != 0
                || ch as libc::c_int & 0x80 as libc::c_int != 0
            {
                return s;
            }
            match ch as libc::c_int {
                63 | 35 => return s,
                _ => {}
            }
            current_block_58 = 7420279277351916581;
        }
        _ => {
            current_block_58 = 7420279277351916581;
        }
    }
    match current_block_58 {
        7200080685983564597 => {
            if ch as libc::c_int == '/' as i32 {
                return s_req_path;
            }
            if ch as libc::c_int == '?' as i32 {
                return s_req_query_string_start;
            }
            if ch as libc::c_int == '@' as i32 {
                return s_req_server_with_at;
            }
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
                || ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32
                || (ch as libc::c_int == '-' as i32 || ch as libc::c_int == '_' as i32
                    || ch as libc::c_int == '.' as i32 || ch as libc::c_int == '!' as i32
                    || ch as libc::c_int == '~' as i32 || ch as libc::c_int == '*' as i32
                    || ch as libc::c_int == '\'' as i32
                    || ch as libc::c_int == '(' as i32
                    || ch as libc::c_int == ')' as i32)
                || ch as libc::c_int == '%' as i32 || ch as libc::c_int == ';' as i32
                || ch as libc::c_int == ':' as i32 || ch as libc::c_int == '&' as i32
                || ch as libc::c_int == '=' as i32 || ch as libc::c_int == '+' as i32
                || ch as libc::c_int == '$' as i32 || ch as libc::c_int == ',' as i32
                || ch as libc::c_int == '[' as i32 || ch as libc::c_int == ']' as i32
            {
                return s_req_server;
            }
        }
        _ => {}
    }
    return s_dead;
}
unsafe extern "C" fn http_parse_host_char(
    mut s: http_host_state,
    ch: libc::c_char,
) -> http_host_state {
    let mut current_block_35: u64;
    match s as libc::c_uint {
        3 | 2 => {
            if ch as libc::c_int == '@' as i32 {
                return s_http_host_start;
            }
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
                || ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32
                || (ch as libc::c_int == '-' as i32 || ch as libc::c_int == '_' as i32
                    || ch as libc::c_int == '.' as i32 || ch as libc::c_int == '!' as i32
                    || ch as libc::c_int == '~' as i32 || ch as libc::c_int == '*' as i32
                    || ch as libc::c_int == '\'' as i32
                    || ch as libc::c_int == '(' as i32
                    || ch as libc::c_int == ')' as i32)
                || ch as libc::c_int == '%' as i32 || ch as libc::c_int == ';' as i32
                || ch as libc::c_int == ':' as i32 || ch as libc::c_int == '&' as i32
                || ch as libc::c_int == '=' as i32 || ch as libc::c_int == '+' as i32
                || ch as libc::c_int == '$' as i32 || ch as libc::c_int == ',' as i32
            {
                return s_http_userinfo;
            }
            current_block_35 = 2891135413264362348;
        }
        4 => {
            if ch as libc::c_int == '[' as i32 {
                return s_http_host_v6_start;
            }
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
                || ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32
                || ch as libc::c_int == '.' as i32 || ch as libc::c_int == '-' as i32
                || ch as libc::c_int == '_' as i32
            {
                return s_http_host;
            }
            current_block_35 = 2891135413264362348;
        }
        6 => {
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
                || ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32
                || ch as libc::c_int == '.' as i32 || ch as libc::c_int == '-' as i32
                || ch as libc::c_int == '_' as i32
            {
                return s_http_host;
            }
            current_block_35 = 5723828990952924696;
        }
        8 => {
            current_block_35 = 5723828990952924696;
        }
        7 => {
            if ch as libc::c_int == ']' as i32 {
                return s_http_host_v6_end;
            }
            current_block_35 = 3815536834111895603;
        }
        5 => {
            current_block_35 = 3815536834111895603;
        }
        10 => {
            if ch as libc::c_int == ']' as i32 {
                return s_http_host_v6_end;
            }
            current_block_35 = 9926130506383208315;
        }
        9 => {
            current_block_35 = 9926130506383208315;
        }
        12 | 11 => {
            if ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32 {
                return s_http_host_port;
            }
            current_block_35 = 2891135413264362348;
        }
        _ => {
            current_block_35 = 2891135413264362348;
        }
    }
    match current_block_35 {
        3815536834111895603 => {
            if ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32
                || (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int >= 'a' as i32
                    && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                        as libc::c_int <= 'f' as i32 || ch as libc::c_int == ':' as i32
                || ch as libc::c_int == '.' as i32
            {
                return s_http_host_v6;
            }
            if s as libc::c_uint == s_http_host_v6 as libc::c_int as libc::c_uint
                && ch as libc::c_int == '%' as i32
            {
                return s_http_host_v6_zone_start;
            }
        }
        5723828990952924696 => {
            if ch as libc::c_int == ':' as i32 {
                return s_http_host_port_start;
            }
        }
        9926130506383208315 => {
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
                || ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32
                || ch as libc::c_int == '%' as i32 || ch as libc::c_int == '.' as i32
                || ch as libc::c_int == '-' as i32 || ch as libc::c_int == '_' as i32
                || ch as libc::c_int == '~' as i32
            {
                return s_http_host_v6_zone;
            }
        }
        _ => {}
    }
    return s_http_host_dead;
}
unsafe extern "C" fn http_parse_host(
    mut buf: *const libc::c_char,
    mut u: *mut vapor_urlparser_url,
    mut found_at: libc::c_int,
) -> libc::c_int {
    let mut s: http_host_state = 0 as http_host_state;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut buflen: size_t = ((*u).field_data[UF_HOST as libc::c_int as usize].off
        as libc::c_int
        + (*u).field_data[UF_HOST as libc::c_int as usize].len as libc::c_int) as size_t;
    if (*u).field_set as libc::c_int & (1 as libc::c_int) << UF_HOST as libc::c_int != 0
    {} else {
        __assert_fail(
            b"u->field_set & (1 << UF_HOST)\0" as *const u8 as *const libc::c_char,
            b"/mnt/ext/etc/projects/swift_projects/vapor/Sources/CVaporURLParser/urlparser.c\0"
                as *const u8 as *const libc::c_char,
            506 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"int http_parse_host(const char *, struct vapor_urlparser_url *, int)\0"))
                .as_ptr(),
        );
    }
    'c_1399: {
        if (*u).field_set as libc::c_int & (1 as libc::c_int) << UF_HOST as libc::c_int
            != 0
        {} else {
            __assert_fail(
                b"u->field_set & (1 << UF_HOST)\0" as *const u8 as *const libc::c_char,
                b"/mnt/ext/etc/projects/swift_projects/vapor/Sources/CVaporURLParser/urlparser.c\0"
                    as *const u8 as *const libc::c_char,
                506 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"int http_parse_host(const char *, struct vapor_urlparser_url *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*u).field_data[UF_HOST as libc::c_int as usize].len = 0 as libc::c_int as uint16_t;
    s = (if found_at != 0 {
        s_http_userinfo_start as libc::c_int
    } else {
        s_http_host_start as libc::c_int
    }) as http_host_state;
    p = buf
        .offset(
            (*u).field_data[UF_HOST as libc::c_int as usize].off as libc::c_int as isize,
        );
    while p < buf.offset(buflen as isize) {
        let mut new_s: http_host_state = http_parse_host_char(s, *p);
        if new_s as libc::c_uint == s_http_host_dead as libc::c_int as libc::c_uint {
            return 1 as libc::c_int;
        }
        match new_s as libc::c_uint {
            6 => {
                if s as libc::c_uint != s_http_host as libc::c_int as libc::c_uint {
                    (*u)
                        .field_data[UF_HOST as libc::c_int as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                }
                (*u)
                    .field_data[UF_HOST as libc::c_int as usize]
                    .len = ((*u).field_data[UF_HOST as libc::c_int as usize].len)
                    .wrapping_add(1);
                (*u).field_data[UF_HOST as libc::c_int as usize].len;
            }
            7 => {
                if s as libc::c_uint != s_http_host_v6 as libc::c_int as libc::c_uint {
                    (*u)
                        .field_data[UF_HOST as libc::c_int as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                }
                (*u)
                    .field_data[UF_HOST as libc::c_int as usize]
                    .len = ((*u).field_data[UF_HOST as libc::c_int as usize].len)
                    .wrapping_add(1);
                (*u).field_data[UF_HOST as libc::c_int as usize].len;
            }
            9 | 10 => {
                (*u)
                    .field_data[UF_HOST as libc::c_int as usize]
                    .len = ((*u).field_data[UF_HOST as libc::c_int as usize].len)
                    .wrapping_add(1);
                (*u).field_data[UF_HOST as libc::c_int as usize].len;
            }
            12 => {
                if s as libc::c_uint != s_http_host_port as libc::c_int as libc::c_uint {
                    (*u)
                        .field_data[UF_PORT as libc::c_int as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                    (*u)
                        .field_data[UF_PORT as libc::c_int as usize]
                        .len = 0 as libc::c_int as uint16_t;
                    (*u)
                        .field_set = ((*u).field_set as libc::c_int
                        | (1 as libc::c_int) << UF_PORT as libc::c_int) as uint16_t;
                }
                (*u)
                    .field_data[UF_PORT as libc::c_int as usize]
                    .len = ((*u).field_data[UF_PORT as libc::c_int as usize].len)
                    .wrapping_add(1);
                (*u).field_data[UF_PORT as libc::c_int as usize].len;
            }
            3 => {
                if s as libc::c_uint != s_http_userinfo as libc::c_int as libc::c_uint {
                    (*u)
                        .field_data[UF_USERINFO as libc::c_int as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                    (*u)
                        .field_data[UF_USERINFO as libc::c_int as usize]
                        .len = 0 as libc::c_int as uint16_t;
                    (*u)
                        .field_set = ((*u).field_set as libc::c_int
                        | (1 as libc::c_int) << UF_USERINFO as libc::c_int) as uint16_t;
                }
                (*u)
                    .field_data[UF_USERINFO as libc::c_int as usize]
                    .len = ((*u).field_data[UF_USERINFO as libc::c_int as usize].len)
                    .wrapping_add(1);
                (*u).field_data[UF_USERINFO as libc::c_int as usize].len;
            }
            _ => {}
        }
        s = new_s;
        p = p.offset(1);
        p;
    }
    match s as libc::c_uint {
        4 | 5 | 7 | 9 | 10 | 11 | 3 | 2 => return 1 as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn urlparser_url_init(mut u: *mut vapor_urlparser_url) {
    memset(
        u as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<vapor_urlparser_url>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn vapor_urlparser_parse(
    mut buf: *const libc::c_char,
    mut buflen: size_t,
    mut is_connect: libc::c_int,
    mut u: *mut vapor_urlparser_url,
) -> libc::c_int {
    let mut s: state = 0 as state;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut uf: vapor_urlparser_fields = UF_SCHEMA;
    let mut old_uf: vapor_urlparser_fields = UF_SCHEMA;
    let mut found_at: libc::c_int = 0 as libc::c_int;
    if buflen == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    (*u).field_set = 0 as libc::c_int as uint16_t;
    (*u).port = (*u).field_set;
    s = (if is_connect != 0 {
        s_req_server_start as libc::c_int
    } else {
        s_req_spaces_before_url as libc::c_int
    }) as state;
    old_uf = UF_MAX;
    let mut current_block_21: u64;
    p = buf;
    while p < buf.offset(buflen as isize) {
        s = parse_url_char(s, *p);
        match s as libc::c_uint {
            1 => return 1 as libc::c_int,
            22 | 23 | 24 | 28 | 30 => {
                current_block_21 = 11875828834189669668;
            }
            21 => {
                uf = UF_SCHEMA;
                current_block_21 = 26972500619410423;
            }
            26 => {
                found_at = 1 as libc::c_int;
                current_block_21 = 2398777646703561453;
            }
            25 => {
                current_block_21 = 2398777646703561453;
            }
            27 => {
                uf = UF_PATH;
                current_block_21 = 26972500619410423;
            }
            29 => {
                uf = UF_QUERY;
                current_block_21 = 26972500619410423;
            }
            31 => {
                uf = UF_FRAGMENT;
                current_block_21 = 26972500619410423;
            }
            _ => {
                if (b"Unexpected state\0" as *const u8 as *const libc::c_char).is_null()
                {} else {
                    __assert_fail(
                        b"!\"Unexpected state\"\0" as *const u8 as *const libc::c_char,
                        b"/mnt/ext/etc/projects/swift_projects/vapor/Sources/CVaporURLParser/urlparser.c\0"
                            as *const u8 as *const libc::c_char,
                        644 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 83],
                            &[libc::c_char; 83],
                        >(
                            b"int vapor_urlparser_parse(const char *, size_t, int, struct vapor_urlparser_url *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_1581: {
                    if (b"Unexpected state\0" as *const u8 as *const libc::c_char)
                        .is_null()
                    {} else {
                        __assert_fail(
                            b"!\"Unexpected state\"\0" as *const u8
                                as *const libc::c_char,
                            b"/mnt/ext/etc/projects/swift_projects/vapor/Sources/CVaporURLParser/urlparser.c\0"
                                as *const u8 as *const libc::c_char,
                            644 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 83],
                                &[libc::c_char; 83],
                            >(
                                b"int vapor_urlparser_parse(const char *, size_t, int, struct vapor_urlparser_url *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                return 1 as libc::c_int;
            }
        }
        match current_block_21 {
            2398777646703561453 => {
                uf = UF_HOST;
                current_block_21 = 26972500619410423;
            }
            _ => {}
        }
        match current_block_21 {
            26972500619410423 => {
                if uf as libc::c_uint == old_uf as libc::c_uint {
                    (*u)
                        .field_data[uf as usize]
                        .len = ((*u).field_data[uf as usize].len).wrapping_add(1);
                    (*u).field_data[uf as usize].len;
                } else {
                    (*u)
                        .field_data[uf as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                    (*u).field_data[uf as usize].len = 1 as libc::c_int as uint16_t;
                    (*u)
                        .field_set = ((*u).field_set as libc::c_int
                        | (1 as libc::c_int) << uf as libc::c_uint) as uint16_t;
                    old_uf = uf;
                }
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    if (*u).field_set as libc::c_int & (1 as libc::c_int) << UF_SCHEMA as libc::c_int
        != 0
        && (*u).field_set as libc::c_int & (1 as libc::c_int) << UF_HOST as libc::c_int
            == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if (*u).field_set as libc::c_int & (1 as libc::c_int) << UF_HOST as libc::c_int != 0
    {
        if http_parse_host(buf, u, found_at) != 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    if is_connect != 0
        && (*u).field_set as libc::c_int
            != (1 as libc::c_int) << UF_HOST as libc::c_int
                | (1 as libc::c_int) << UF_PORT as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if (*u).field_set as libc::c_int & (1 as libc::c_int) << UF_PORT as libc::c_int != 0
    {
        let mut off: uint16_t = 0;
        let mut len: uint16_t = 0;
        let mut p_0: *const libc::c_char = 0 as *const libc::c_char;
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        let mut v: libc::c_ulong = 0;
        off = (*u).field_data[UF_PORT as libc::c_int as usize].off;
        len = (*u).field_data[UF_PORT as libc::c_int as usize].len;
        end = buf
            .offset(off as libc::c_int as isize)
            .offset(len as libc::c_int as isize);
        if (off as libc::c_int + len as libc::c_int) as libc::c_ulong <= buflen
            && !(b"Port number overflow\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"off + len <= buflen && \"Port number overflow\"\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/ext/etc/projects/swift_projects/vapor/Sources/CVaporURLParser/urlparser.c\0"
                    as *const u8 as *const libc::c_char,
                691 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"int vapor_urlparser_parse(const char *, size_t, int, struct vapor_urlparser_url *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_283: {
            if (off as libc::c_int + len as libc::c_int) as libc::c_ulong <= buflen
                && !(b"Port number overflow\0" as *const u8 as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"off + len <= buflen && \"Port number overflow\"\0" as *const u8
                        as *const libc::c_char,
                    b"/mnt/ext/etc/projects/swift_projects/vapor/Sources/CVaporURLParser/urlparser.c\0"
                        as *const u8 as *const libc::c_char,
                    691 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 83],
                        &[libc::c_char; 83],
                    >(
                        b"int vapor_urlparser_parse(const char *, size_t, int, struct vapor_urlparser_url *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        v = 0 as libc::c_int as libc::c_ulong;
        p_0 = buf.offset(off as libc::c_int as isize);
        while p_0 < end {
            v = v.wrapping_mul(10 as libc::c_int as libc::c_ulong);
            v = v.wrapping_add((*p_0 as libc::c_int - '0' as i32) as libc::c_ulong);
            if v > 0xffff as libc::c_int as libc::c_ulong {
                return 1 as libc::c_int;
            }
            p_0 = p_0.offset(1);
            p_0;
        }
        (*u).port = v as uint16_t;
    }
    return 0 as libc::c_int;
}
