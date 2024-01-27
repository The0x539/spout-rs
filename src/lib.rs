use std::sync::OnceLock;

use gl::types::{GLenum, GLuint};
use std::ffi::{c_char, c_double, c_int, c_long, c_uchar, c_uint, c_void};
type DWORD = u32;
type HANDLE = *mut c_void;

use libloading::{Library, Symbol};

pub struct Spout {
    handle: *const SpoutLibrary,
}

#[repr(C)]
#[non_exhaustive] // I believe this struct has some other private data.
struct SpoutLibrary {
    vtable: *const SpoutVtable,
}

macro_rules! spout_library {
    (
        $wrapper:ident
        $library:ident
        $vtable:ident
        $(
            $vis:vis fn $func:ident (
                $($arg:ident : $aty:ty),*
                $(,)?
            ) $(-> $ret:ty)? ;
        )*
    ) => {
        #[repr(C)]
        struct $vtable {
            $(
                $func: unsafe extern "C" fn(
                    *const $library, $($aty),*
                ) $(-> $ret)? ,
            )*
        }

        #[allow(dead_code)]
        impl $wrapper {
            $(
                $vis unsafe fn $func(
                    &self,
                    $($arg : $aty),*
                ) $(-> $ret)? {
                    let handle: *const $library = self.handle;
                    let method = (*(*handle).vtable).$func;
                    method(handle, $($arg),*)
                }
            )*
        }
    };
}

spout_library! {
    Spout
    SpoutLibrary
    SpoutVtable

    pub fn set_sender_name(sendername: *const c_char);
    pub fn set_sender_format(format: DWORD);
    pub fn release_sender(msec: DWORD);
    pub fn send_fbo(fbo_id: GLuint, width: c_uint, height: c_uint, invert: bool) -> bool;
    pub fn send_texture(texture_id: GLuint, texture_target: GLuint, width: c_uint, height: c_uint, invert: bool, host_fbo: GLuint) -> bool;
    pub fn send_image(pixels: *const c_uchar, width: c_uint, height: c_uint, format: GLenum, invert: bool) -> bool;
    pub fn get_name() -> *const c_char;
    pub fn get_width() -> c_uint;
    pub fn get_height() -> c_uint;
    pub fn get_fps() -> c_double;
    pub fn get_frame() -> c_long;
    pub fn get_handle() -> HANDLE;
    pub fn get_cpu() -> bool;
    pub fn get_gldx() -> bool;

    pub fn set_receiver_name(sendername: *const c_char);
    pub fn release_receiver();

    pub fn receive_texture(texture_id: GLuint, texture_target: GLuint, invert: bool, host_fbo: GLuint) -> bool;
    pub fn receive_image(pixels: *mut c_uchar, format: GLenum, invert: bool, host_fbo: GLuint) -> bool;
    pub fn is_updated() -> bool;
    pub fn is_connected() -> bool;
    pub fn is_frame_new() -> bool;
    pub fn get_sender_name() -> *const c_char;
    pub fn get_sender_width() -> c_uint;
    pub fn get_sender_height() -> c_uint;
    pub fn get_sender_format() -> DWORD;
    pub fn get_sender_fps() -> c_double;
    pub fn get_sender_frame() -> c_long;
    pub fn get_sender_handle() -> HANDLE;
    pub fn get_sender_cpu() -> bool;
    pub fn get_sender_gldx() -> bool;
    pub fn select_sender();

    pub fn set_frame_count(enable: bool);
    pub fn disable_frame_count();
    pub fn is_frame_count_enabled();
    pub fn hold_fps(fps: c_int);
    pub fn get_refresh_rate() -> c_double;
    pub fn set_frame_sync(sender_name: *const c_char);
    pub fn wait_frame_sync(sender_name: *const c_char, timeout: DWORD) -> bool;
    pub fn enable_frame_sync(sync: bool);

    pub fn write_memory_buffer(sender_name: *const c_char, data: *const c_char, length: c_int) -> bool;
    pub fn read_memory_buffer(sender_name: *const c_char, data: *mut c_char, max_length: c_int) -> c_int;
    pub fn create_memory_buffer(name: *const c_char, length: c_int) -> bool;
    pub fn delete_memory_buffer() -> bool;
    pub fn get_memory_buffer_size(name: *const c_char) -> c_int;

    pub fn open_spout_console();
    pub fn close_spout_console(warning: bool);
    pub fn enable_spout_log();
    pub fn enable_spout_log_file(filename: *const c_char, append: bool);
    fn get_spout_log() -> *mut c_void; // this was supposed to be usable from C lmao
    pub fn show_spout_logs();
    pub fn disable_spout_log();
    pub fn set_spout_log_level(level: SpoutLibLogLevel);
    pub fn spout_log(format: *const c_char);
    pub fn spout_log_verbose(format: *const c_char);
    pub fn spout_log_notice(format: *const c_char);
    pub fn spout_log_warning(format: *const c_char);
    pub fn spout_log_error(format: *const c_char);
    pub fn spout_log_fatal(format: *const c_char);
    pub fn spout_message_box(message: *const c_char, milliseconds: DWORD) -> c_int;
    // fn _spout_message_box(); // nah
    fn _spout_message_box_icon();
    // fn __spout_message_box_icon() -> bool;
    fn _spout_message_box_button();
    fn _spout_message_box_modeless();
    fn _spout_message_box_window();
    fn _copy_to_clip_board() -> bool;

    fn _read_dword_from_registry() -> bool;
    fn _write_dword_to_registry() -> bool;
    fn _read_path_from_registry() -> bool;
    fn _write_path_to_registry() -> bool;
    fn _remove_path_from_registry() -> bool;
    fn _remove_sub_key() -> bool;
    fn _find_sub_key() -> bool;

    pub fn get_sdk_version() -> *mut c_void;
    pub fn is_laptop() -> bool;

    pub fn start_timing();
    pub fn end_timing() -> c_double;

    pub fn is_initialized() -> bool;
    pub fn bind_shared_texture() -> bool;
    pub fn un_bind_shared_texture() -> bool;
    pub fn get_shared_texture_id() -> GLuint;

    pub fn get_sender_count() -> c_int;
    pub fn get_sender(index: c_int, sender_name: *mut c_char, max_size: c_int) -> bool;
    pub fn find_sender_name(sender_name: *const c_char) -> bool;
    pub fn get_sender_info(sender_name: *const c_char, width: *mut c_uint, height: *mut c_uint, dx_share_handle: *mut HANDLE, format: *mut DWORD) -> bool;
    pub fn get_active_sender(sender_name: *mut c_char) -> bool;
    pub fn set_active_sender(sender_name: *const c_char) -> bool;

    pub fn get_buffer_mode() -> bool;
    pub fn set_buffer_mode(active: bool);
    pub fn get_buffers() -> c_int;
    pub fn set_buffers(n_buffers: c_int);
    pub fn get_max_senders() -> c_int;
    pub fn set_max_senders(max_senders: c_int);

    pub fn create_sender(sender_name: *const c_char, width: c_uint, height: c_uint, format: DWORD) -> bool;
    pub fn update_sender(sender_name: *const c_char, width: c_uint, height: c_uint) -> bool;
    pub fn create_receiver(sender_name: *const c_char, width: *mut c_uint, height: *mut c_uint, use_active: bool) -> bool;
    pub fn check_receiver(sender_name: *const c_char, width: *mut c_uint, height: *mut c_uint, connected: *mut bool) -> bool;
    pub fn get_dx9() -> bool;
    pub fn set_dx9(dx9: bool) -> bool;
    pub fn get_memory_share_mode() -> bool;
    pub fn set_memory_share_mode(mem: bool) -> bool;
    pub fn get_cpu_mode() -> bool;
    pub fn set_cpu_mode(cpu: bool) -> bool;
    pub fn get_share_mode() -> c_int;
    pub fn set_share_mode(mode: c_int);
    pub fn select_sender_panel();

    pub fn get_host_path(sender_name: *const c_char, host_path: *mut c_char, max_chars: c_int) -> bool;
    pub fn get_vertical_sync() -> c_int;
    pub fn set_vertical_sync(sync: bool) -> bool;
    pub fn get_spout_version() -> c_int;

    pub fn get_auto_share() -> bool;
    pub fn set_auto_share(auto: bool);
    pub fn is_gldx_ready() -> bool;

    pub fn get_num_adapters() -> c_int;
    pub fn get_adapter_name(index: c_int, adapter_name: *mut c_char, max_chars: c_int) -> bool;
    pub fn adapter_name() -> *mut c_char;
    pub fn get_adapter() -> c_int;

    // ifdef NTDDI_WIN10_RS4
    pub fn get_performance_preference(path: *const c_char) -> c_int;
    pub fn set_performance_preference(preference: c_int, path: *const c_char) -> bool;
    pub fn get_preferred_adapter_name(preference: c_int, adapter_name: *mut c_char, max_chars: c_int) -> bool;
    pub fn set_preferred_adapter(preference: c_int) -> bool;
    pub fn is_preference_available() -> bool;
    pub fn is_application_path(path: *const c_char) -> bool;
    // endif

    pub fn create_opengl() -> bool;
    pub fn close_opengl() -> bool;
    pub fn copy_texture(
        source_id: GLuint,
        source_target: GLuint,
        dest_id: GLuint,
        dest_target: GLuint,
        width: c_uint,
        height: c_uint,
        invert: bool,
        host_fbo: GLuint,
    );

    fn _get_dx11_format();
    fn _set_dx11_format();
    fn _dx11_format();
    fn _gldx_format();
    fn _glformat();
    fn _gl_format_name();

    pub fn open_directx() -> bool;
    pub fn close_directx();

    pub fn open_directx_11(device: *mut c_void) -> bool;
    pub fn close_directx_11();
    pub fn get_dx11_device() -> *mut c_void;
    pub fn get_dx11_context() -> *mut c_void;

    fn release();
}

#[repr(i32)]
pub enum SpoutLibLogLevel {
    Silent,
    Verbose,
    Notice,
    Warning,
    Error,
    Fatal,
    None,
}

// at the time of writing, Option::unwrap_or is not const
// should this be a compile-time decision? probably not ¯\_(ツ)_/¯
const DLL_PATH: &str = match option_env!("SPOUT_DLL_PATH") {
    Some(x) => x,
    None => "SpoutLibrary.dll",
};

static SPOUT_DLL: OnceLock<Library> = OnceLock::new();
static GET_SPOUT: OnceLock<Symbol<unsafe extern "C" fn() -> *const SpoutLibrary>> = OnceLock::new();

impl Spout {
    pub fn new() -> Self {
        unsafe {
            let spout_dll = SPOUT_DLL.get_or_init(|| Library::new(DLL_PATH).unwrap());
            let get_spout = GET_SPOUT.get_or_init(|| spout_dll.get(b"GetSpout").unwrap());
            let handle = get_spout();
            assert!(!handle.is_null());
            Self { handle }
        }
    }
}

impl Drop for Spout {
    fn drop(&mut self) {
        unsafe { self.release() }
    }
}
