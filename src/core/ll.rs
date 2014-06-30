#![allow(dead_code)]

/* automatically generated by rust-bindgen */

pub type int8_t = ::libc::c_char;
pub type int16_t = ::libc::c_short;
pub type int32_t = ::libc::c_int;
pub type int64_t = ::libc::c_long;
pub type uint8_t = ::libc::c_uchar;
pub type uint16_t = ::libc::c_ushort;
pub type uint32_t = ::libc::c_uint;
pub type uint64_t = ::libc::c_ulong;
pub type int_least8_t = ::libc::c_char;
pub type int_least16_t = ::libc::c_short;
pub type int_least32_t = ::libc::c_int;
pub type int_least64_t = ::libc::c_long;
pub type uint_least8_t = ::libc::c_uchar;
pub type uint_least16_t = ::libc::c_ushort;
pub type uint_least32_t = ::libc::c_uint;
pub type uint_least64_t = ::libc::c_ulong;
pub type int_fast8_t = ::libc::c_char;
pub type int_fast16_t = ::libc::c_long;
pub type int_fast32_t = ::libc::c_long;
pub type int_fast64_t = ::libc::c_long;
pub type uint_fast8_t = ::libc::c_uchar;
pub type uint_fast16_t = ::libc::c_ulong;
pub type uint_fast32_t = ::libc::c_ulong;
pub type uint_fast64_t = ::libc::c_ulong;
pub type intptr_t = ::libc::c_long;
pub type uintptr_t = ::libc::c_ulong;
pub type intmax_t = ::libc::c_long;
pub type uintmax_t = ::libc::c_ulong;
pub type Enum_Unnamed1 = ::libc::c_int;
pub static TOX_FAERR_TOOLONG: ::libc::c_int = -1;
pub static TOX_FAERR_NOMESSAGE: ::libc::c_int = -2;
pub static TOX_FAERR_OWNKEY: ::libc::c_int = -3;
pub static TOX_FAERR_ALREADYSENT: ::libc::c_int = -4;
pub static TOX_FAERR_UNKNOWN: ::libc::c_int = -5;
pub static TOX_FAERR_BADCHECKSUM: ::libc::c_int = -6;
pub static TOX_FAERR_SETNEWNOSPAM: ::libc::c_int = -7;
pub static TOX_FAERR_NOMEM: ::libc::c_int = -8;
pub type TOX_USERSTATUS = ::libc::c_uint;
pub static TOX_USERSTATUS_NONE: ::libc::c_uint = 0;
pub static TOX_USERSTATUS_AWAY: ::libc::c_uint = 1;
pub static TOX_USERSTATUS_BUSY: ::libc::c_uint = 2;
pub static TOX_USERSTATUS_INVALID: ::libc::c_uint = 3;
pub enum Struct_Tox { }
pub type Tox = Struct_Tox;
pub type TOX_CHAT_CHANGE = ::libc::c_uint;
pub static TOX_CHAT_CHANGE_PEER_ADD: ::libc::c_uint = 0;
pub static TOX_CHAT_CHANGE_PEER_DEL: ::libc::c_uint = 1;
pub static TOX_CHAT_CHANGE_PEER_NAME: ::libc::c_uint = 2;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub static TOX_FILECONTROL_ACCEPT: ::libc::c_uint = 0;
pub static TOX_FILECONTROL_PAUSE: ::libc::c_uint = 1;
pub static TOX_FILECONTROL_KILL: ::libc::c_uint = 2;
pub static TOX_FILECONTROL_FINISHED: ::libc::c_uint = 3;
pub static TOX_FILECONTROL_RESUME_BROKEN: ::libc::c_uint = 4;
#[link(name = "toxcore")]
extern "C" {
    pub fn tox_get_address(tox: *mut Tox, address: *mut uint8_t);
    pub fn tox_add_friend(tox: *mut Tox, address: *mut uint8_t,
                          data: *mut uint8_t, length: uint16_t) -> int32_t;
    pub fn tox_add_friend_norequest(tox: *mut Tox, client_id: *const uint8_t) ->
     int32_t;
    pub fn tox_get_friend_number(tox: *mut Tox, client_id: *mut uint8_t) ->
     int32_t;
    pub fn tox_get_client_id(tox: *mut Tox, friendnumber: int32_t,
                             client_id: *mut uint8_t) -> ::libc::c_int;
    pub fn tox_del_friend(tox: *mut Tox, friendnumber: int32_t) ->
     ::libc::c_int;
    pub fn tox_get_friend_connection_status(tox: *mut Tox,
                                            friendnumber: int32_t) ->
     ::libc::c_int;
    pub fn tox_friend_exists(tox: *mut Tox, friendnumber: int32_t) ->
     ::libc::c_int;
    pub fn tox_send_message(tox: *mut Tox, friendnumber: int32_t,
                            message: *const uint8_t, length: uint32_t) -> uint32_t;
    pub fn tox_send_message_withid(tox: *mut Tox, friendnumber: int32_t,
                                   theid: uint32_t, message: *mut uint8_t,
                                   length: uint32_t) -> uint32_t;
    pub fn tox_send_action(tox: *mut Tox, friendnumber: int32_t,
                           action: *mut uint8_t, length: uint32_t) ->
     uint32_t;
    pub fn tox_send_action_withid(tox: *mut Tox, friendnumber: int32_t,
                                  theid: uint32_t, action: *mut uint8_t,
                                  length: uint32_t) -> uint32_t;
    pub fn tox_set_name(tox: *mut Tox, name: *mut uint8_t, length: uint16_t)
     -> ::libc::c_int;
    pub fn tox_get_self_name(tox: *mut Tox, name: *mut uint8_t) -> uint16_t;
    pub fn tox_get_name(tox: *mut Tox, friendnumber: int32_t,
                        name: *mut uint8_t) -> ::libc::c_int;
    pub fn tox_get_name_size(tox: *mut Tox, friendnumber: int32_t) ->
     ::libc::c_int;
    pub fn tox_get_self_name_size(tox: *mut Tox) -> ::libc::c_int;
    pub fn tox_set_status_message(tox: *mut Tox, status: *mut uint8_t,
                                  length: uint16_t) -> ::libc::c_int;
    pub fn tox_set_user_status(tox: *mut Tox, userstatus: uint8_t) ->
     ::libc::c_int;
    pub fn tox_get_status_message_size(tox: *mut Tox, friendnumber: int32_t)
     -> ::libc::c_int;
    pub fn tox_get_self_status_message_size(tox: *mut Tox) -> ::libc::c_int;
    pub fn tox_get_status_message(tox: *mut Tox, friendnumber: int32_t,
                                  buf: *mut uint8_t, maxlen: uint32_t) ->
     ::libc::c_int;
    pub fn tox_get_self_status_message(tox: *mut Tox, buf: *mut uint8_t,
                                       maxlen: uint32_t) -> ::libc::c_int;
    pub fn tox_get_user_status(tox: *mut Tox, friendnumber: int32_t) ->
     uint8_t;
    pub fn tox_get_self_user_status(tox: *mut Tox) -> uint8_t;
    pub fn tox_get_last_online(tox: *mut Tox, friendnumber: int32_t) ->
     uint64_t;
    pub fn tox_set_user_is_typing(tox: *mut Tox, friendnumber: int32_t,
                                  is_typing: uint8_t) -> ::libc::c_int;
    pub fn tox_get_is_typing(tox: *mut Tox, friendnumber: int32_t) -> uint8_t;
    pub fn tox_set_sends_receipts(tox: *mut Tox, friendnumber: int32_t,
                                  yesno: ::libc::c_int);
    pub fn tox_count_friendlist(tox: *mut Tox) -> uint32_t;
    pub fn tox_get_num_online_friends(tox: *mut Tox) -> uint32_t;
    pub fn tox_get_friendlist(tox: *mut Tox, out_list: *mut int32_t,
                              list_size: uint32_t) -> uint32_t;
    pub fn tox_callback_friend_request(tox: *mut Tox,
                                       function:
                                           ::std::option::Option<extern "C" fn
                                                                     (arg1:
                                                                          *mut Tox,
                                                                      arg2:
                                                                          *const uint8_t,
                                                                      arg3:
                                                                          *const uint8_t,
                                                                      arg4:
                                                                          uint16_t,
                                                                      arg5:
                                                                          *mut ::libc::c_void)>,
                                       userdata: *mut ::libc::c_void);
    pub fn tox_callback_friend_message(tox: *mut Tox,
                                       function:
                                           ::std::option::Option<extern "C" fn
                                                                     (arg1:
                                                                          *mut Tox,
                                                                      arg2:
                                                                          int32_t,
                                                                      arg3:
                                                                          *mut uint8_t,
                                                                      arg4:
                                                                          uint16_t,
                                                                      arg5:
                                                                          *mut ::libc::c_void)>,
                                       userdata: *mut ::libc::c_void);
    pub fn tox_callback_friend_action(tox: *mut Tox,
                                      function:
                                          ::std::option::Option<extern "C" fn
                                                                    (arg1:
                                                                         *mut Tox,
                                                                     arg2:
                                                                         int32_t,
                                                                     arg3:
                                                                         *mut uint8_t,
                                                                     arg4:
                                                                         uint16_t,
                                                                     arg5:
                                                                         *mut ::libc::c_void)>,
                                      userdata: *mut ::libc::c_void);
    pub fn tox_callback_name_change(tox: *mut Tox,
                                    function:
                                        ::std::option::Option<extern "C" fn
                                                                  (arg1:
                                                                       *mut Tox,
                                                                   arg2:
                                                                       int32_t,
                                                                   arg3:
                                                                       *mut uint8_t,
                                                                   arg4:
                                                                       uint16_t,
                                                                   arg5:
                                                                       *mut ::libc::c_void)>,
                                    userdata: *mut ::libc::c_void);
    pub fn tox_callback_status_message(tox: *mut Tox,
                                       function:
                                           ::std::option::Option<extern "C" fn
                                                                     (arg1:
                                                                          *mut Tox,
                                                                      arg2:
                                                                          int32_t,
                                                                      arg3:
                                                                          *mut uint8_t,
                                                                      arg4:
                                                                          uint16_t,
                                                                      arg5:
                                                                          *mut ::libc::c_void)>,
                                       userdata: *mut ::libc::c_void);
    pub fn tox_callback_user_status(tox: *mut Tox,
                                    function:
                                        ::std::option::Option<extern "C" fn
                                                                  (arg1:
                                                                       *mut Tox,
                                                                   arg2:
                                                                       int32_t,
                                                                   arg3:
                                                                       uint8_t,
                                                                   arg4:
                                                                       *mut ::libc::c_void)>,
                                    userdata: *mut ::libc::c_void);
    pub fn tox_callback_typing_change(tox: *mut Tox,
                                      function:
                                          ::std::option::Option<extern "C" fn
                                                                    (arg1:
                                                                         *mut Tox,
                                                                     arg2:
                                                                         int32_t,
                                                                     arg3:
                                                                         uint8_t,
                                                                     arg4:
                                                                         *mut ::libc::c_void)>,
                                      userdata: *mut ::libc::c_void);
    pub fn tox_callback_read_receipt(tox: *mut Tox,
                                     function:
                                         ::std::option::Option<extern "C" fn
                                                                   (arg1:
                                                                        *mut Tox,
                                                                    arg2:
                                                                        int32_t,
                                                                    arg3:
                                                                        uint32_t,
                                                                    arg4:
                                                                        *mut ::libc::c_void)>,
                                     userdata: *mut ::libc::c_void);
    pub fn tox_callback_connection_status(tox: *mut Tox,
                                          function:
                                              ::std::option::Option<extern "C" fn
                                                                        (arg1:
                                                                             *mut Tox,
                                                                         arg2:
                                                                             int32_t,
                                                                         arg3:
                                                                             uint8_t,
                                                                         arg4:
                                                                             *mut ::libc::c_void)>,
                                          userdata: *mut ::libc::c_void);
    pub fn tox_get_nospam(tox: *mut Tox) -> uint32_t;
    pub fn tox_set_nospam(tox: *mut Tox, nospam: uint32_t);
    pub fn tox_callback_group_invite(tox: *mut Tox,
                                     function:
                                         ::std::option::Option<extern "C" fn
                                                                   (arg1:
                                                                        *mut Tox,
                                                                    arg2:
                                                                        int32_t,
                                                                    arg3:
                                                                        *mut uint8_t,
                                                                    arg4:
                                                                        *mut ::libc::c_void)>,
                                     userdata: *mut ::libc::c_void);
    pub fn tox_callback_group_message(tox: *mut Tox,
                                      function:
                                          ::std::option::Option<extern "C" fn
                                                                    (arg1:
                                                                         *mut Tox,
                                                                     arg2:
                                                                         ::libc::c_int,
                                                                     arg3:
                                                                         ::libc::c_int,
                                                                     arg4:
                                                                         *mut uint8_t,
                                                                     arg5:
                                                                         uint16_t,
                                                                     arg6:
                                                                         *mut ::libc::c_void)>,
                                      userdata: *mut ::libc::c_void);
    pub fn tox_callback_group_action(tox: *mut Tox,
                                     function:
                                         ::std::option::Option<extern "C" fn
                                                                   (arg1:
                                                                        *mut Tox,
                                                                    arg2:
                                                                        ::libc::c_int,
                                                                    arg3:
                                                                        ::libc::c_int,
                                                                    arg4:
                                                                        *mut uint8_t,
                                                                    arg5:
                                                                        uint16_t,
                                                                    arg6:
                                                                        *mut ::libc::c_void)>,
                                     userdata: *mut ::libc::c_void);
    pub fn tox_callback_group_namelist_change(tox: *mut Tox,
                                              function:
                                                  ::std::option::Option<extern "C" fn
                                                                            (arg1:
                                                                                 *mut Tox,
                                                                             arg2:
                                                                                 ::libc::c_int,
                                                                             arg3:
                                                                                 ::libc::c_int,
                                                                             arg4:
                                                                                 uint8_t,
                                                                             arg5:
                                                                                 *mut ::libc::c_void)>,
                                              userdata: *mut ::libc::c_void);
    pub fn tox_add_groupchat(tox: *mut Tox) -> ::libc::c_int;
    pub fn tox_del_groupchat(tox: *mut Tox, groupnumber: ::libc::c_int) ->
     ::libc::c_int;
    pub fn tox_group_peername(tox: *mut Tox, groupnumber: ::libc::c_int,
                              peernumber: ::libc::c_int, name: *mut uint8_t)
     -> ::libc::c_int;
    pub fn tox_invite_friend(tox: *mut Tox, friendnumber: int32_t,
                             groupnumber: ::libc::c_int) -> ::libc::c_int;
    pub fn tox_join_groupchat(tox: *mut Tox, friendnumber: int32_t,
                              friend_group_public_key: *mut uint8_t) ->
     ::libc::c_int;
    pub fn tox_group_message_send(tox: *mut Tox, groupnumber: ::libc::c_int,
                                  message: *mut uint8_t, length: uint32_t) ->
     ::libc::c_int;
    pub fn tox_group_action_send(tox: *mut Tox, groupnumber: ::libc::c_int,
                                 action: *mut uint8_t, length: uint32_t) ->
     ::libc::c_int;
    pub fn tox_group_number_peers(tox: *mut Tox, groupnumber: ::libc::c_int)
     -> ::libc::c_int;
    pub fn tox_group_get_names(tox: *mut Tox, groupnumber: ::libc::c_int,
                               names: *mut [uint8_t, ..128u],
                               lengths: *mut uint16_t, length: uint16_t) ->
     ::libc::c_int;
    pub fn tox_count_chatlist(tox: *mut Tox) -> uint32_t;
    pub fn tox_get_chatlist(tox: *mut Tox, out_list: *mut ::libc::c_int,
                            list_size: uint32_t) -> uint32_t;
    pub fn tox_callback_file_send_request(tox: *mut Tox,
                                          function:
                                              ::std::option::Option<extern "C" fn
                                                                        (arg1:
                                                                             *mut Tox,
                                                                         arg2:
                                                                             int32_t,
                                                                         arg3:
                                                                             uint8_t,
                                                                         arg4:
                                                                             uint64_t,
                                                                         arg5:
                                                                             *mut uint8_t,
                                                                         arg6:
                                                                             uint16_t,
                                                                         arg7:
                                                                             *mut ::libc::c_void)>,
                                          userdata: *mut ::libc::c_void);
    pub fn tox_callback_file_control(tox: *mut Tox,
                                     function:
                                         ::std::option::Option<extern "C" fn
                                                                   (arg1:
                                                                        *mut Tox,
                                                                    arg2:
                                                                        int32_t,
                                                                    arg3:
                                                                        uint8_t,
                                                                    arg4:
                                                                        uint8_t,
                                                                    arg5:
                                                                        uint8_t,
                                                                    arg6:
                                                                        *mut uint8_t,
                                                                    arg7:
                                                                        uint16_t,
                                                                    arg8:
                                                                        *mut ::libc::c_void)>,
                                     userdata: *mut ::libc::c_void);
    pub fn tox_callback_file_data(tox: *mut Tox,
                                  function:
                                      ::std::option::Option<extern "C" fn
                                                                (arg1:
                                                                     *mut Tox,
                                                                 arg2:
                                                                     int32_t,
                                                                 arg3:
                                                                     uint8_t,
                                                                 arg4:
                                                                     *mut uint8_t,
                                                                 arg5:
                                                                     uint16_t,
                                                                 arg6:
                                                                     *mut ::libc::c_void)>,
                                  userdata: *mut ::libc::c_void);
    pub fn tox_new_file_sender(tox: *mut Tox, friendnumber: int32_t,
                               filesize: uint64_t, filename: *mut uint8_t,
                               filename_length: uint16_t) -> ::libc::c_int;
    pub fn tox_file_send_control(tox: *mut Tox, friendnumber: int32_t,
                                 send_receive: uint8_t, filenumber: uint8_t,
                                 message_id: uint8_t, data: *mut uint8_t,
                                 length: uint16_t) -> ::libc::c_int;
    pub fn tox_file_send_data(tox: *mut Tox, friendnumber: int32_t,
                              filenumber: uint8_t, data: *mut uint8_t,
                              length: uint16_t) -> ::libc::c_int;
    pub fn tox_file_data_size(tox: *mut Tox, friendnumber: int32_t) ->
     ::libc::c_int;
    pub fn tox_file_data_remaining(tox: *mut Tox, friendnumber: int32_t,
                                   filenumber: uint8_t, send_receive: uint8_t)
     -> uint64_t;
    pub fn tox_bootstrap_from_address(tox: *mut Tox, address: *const ::libc::c_char,
                                      ipv6enabled: uint8_t, port: uint16_t,
                                      public_key: *mut uint8_t) ->
     ::libc::c_int;
    pub fn tox_isconnected(tox: *mut Tox) -> ::libc::c_int;
    pub fn tox_new(ipv6enabled: uint8_t) -> *mut Tox;
    pub fn tox_kill(tox: *mut Tox);
    pub fn tox_do_interval(tox: *mut Tox) -> uint32_t;
    pub fn tox_do(tox: *mut Tox);
    pub fn tox_size(tox: *mut Tox) -> uint32_t;
    pub fn tox_save(tox: *mut Tox, data: *mut uint8_t);
    pub fn tox_load(tox: *mut Tox, data: *mut uint8_t, length: uint32_t) ->
     ::libc::c_int;
}
