 #[no_mangle]
extern "C" fn quad_url_crate_version() -> u32 {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();
    (major << 24) + (minor << 16) + patch
}

#[cfg(target_arch = "wasm32")]
mod internal{
    extern "C" {
        pub fn timestamp_utc() -> u32;
        pub fn timestamp_utc_ms() -> u32;
    }
}


pub fn timestamp_utc() -> Option<i64>{ 
    
    #[cfg(target_arch = "wasm32")]
    {
        let mut timestamp = None; 
        unsafe {
            timestamp = Some(internal::timestamp_utc() as i64);
        }
        return timestamp;
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        return Some(chrono::Utc::now().timestamp());
    }
}
pub fn timestamp_utc_ms() -> Option<i64>{ 
    #[cfg(target_arch = "wasm32")]
    {
        let mut timestamp = None; 
        unsafe {
            timestamp = Some(internal::timestamp_utc_ms() as i64);
        }
        return timestamp;
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        return Some(chrono::Utc::now().timestamp_millis());
    }
}