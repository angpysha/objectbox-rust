#![allow(dead_code)]
use std::ffi::{c_uint, CStr, CString};
use std::path::Path;

use crate::model::Model;
use crate::util::{ToCVoid};
use crate::{c::*, error};

pub struct Opt {
    pub(crate) obx_opt: *mut OBX_store_options,
    pub(crate) ptr_consumed: bool,
}

impl Drop for Opt {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr_consumed {
                obx_opt_free(self.obx_opt);
                self.obx_opt = std::ptr::null_mut();
            }
        }
    }
}

impl Opt {
    pub fn new() -> Self {
        let obx_opt = unsafe { obx_opt() };
        Opt {
            obx_opt,
            ptr_consumed: false,
        }
    }

    pub fn from_model(model: &mut Model) -> error::Result<Self> {
        let itself = Self::new();
        if let Some(err) = &model.error {
            return Err(err.clone());
        }
        match itself.model(model) {
            Err(err) => Err(err),
            Ok(_) => {
                model.ptr_consumed = true;
                Ok(itself)
            }
        }
    }

    pub fn directory(&self, dir: &Path) -> error::Result<&Self> {
        let c_str = CString::new(dir.to_str().unwrap_or(""))
            .map_err(|e| error::Error::new_local(&format!("Invalid directory path: {}", e)))?;
        call(unsafe { obx_opt_directory(self.obx_opt, c_str.as_ptr()) }).map(|_| self)
    }

    pub fn max_db_size_in_kb(&self, size_in_kb: u64) -> &Self {
        unsafe {
            obx_opt_max_db_size_in_kb(self.obx_opt, size_in_kb);
        }
        self
    }

    pub fn max_data_size_in_kb(&self, size_in_kb: u64) -> &Self {
        unsafe {
            obx_opt_max_data_size_in_kb(self.obx_opt, size_in_kb);
        }
        self
    }

    pub fn file_mode(&self, file_mode: u32) -> &Self {
        unsafe {
            obx_opt_file_mode(self.obx_opt, file_mode as c_uint);
        }
        self
    }

    pub fn max_readers(&self, max_readers: u32) -> &Self {
        unsafe {
            obx_opt_max_readers(self.obx_opt, max_readers as c_uint);
        }
        self
    }

    pub fn no_reader_thread_locals(&self, flag: bool) -> &Self {
        unsafe {
            obx_opt_no_reader_thread_locals(self.obx_opt, flag);
        }
        self
    }

    pub(crate) fn model(&self, model: &mut Model) -> error::Result<&Self> {
        call(unsafe { obx_opt_model(self.obx_opt, model.obx_model) }).map(|_| self)
    }

    pub fn model_bytes(&self, bytes: &Vec<u8>, size: usize) -> error::Result<&Self> {
        call(unsafe { obx_opt_model_bytes(self.obx_opt, bytes.to_const_c_void(), size) })
            .map(|_| self)
    }

    pub fn model_bytes_direct(&self, bytes: &Vec<u8>, size: usize) -> error::Result<&Self> {
        call(unsafe { obx_opt_model_bytes_direct(self.obx_opt, bytes.to_const_c_void(), size) })
            .map(|_| self)
    }

    pub fn validate_on_open_pages(&self, page_limit: usize, flags: u32) -> &Self {
        unsafe {
            obx_opt_validate_on_open_pages(self.obx_opt, page_limit, flags);
        }
        self
    }

    pub fn put_padding_mode(&self, mode: OBXPutPaddingMode) -> &Self {
        unsafe {
            obx_opt_put_padding_mode(self.obx_opt, mode);
        }
        self
    }

    pub fn read_schema(&self, value: bool) -> &Self {
        unsafe {
            obx_opt_read_schema(self.obx_opt, value);
        }
        self
    }

    pub fn use_previous_commit(&self, value: bool) -> &Self {
        unsafe {
            obx_opt_use_previous_commit(self.obx_opt, value);
        }
        self
    }

    pub fn read_only(&self, value: bool) -> &Self {
        unsafe {
            obx_opt_read_only(self.obx_opt, value);
        }
        self
    }

    pub fn debug_flags(&self, flags: u32) -> &Self {
        unsafe {
            obx_opt_debug_flags(self.obx_opt, flags);
        }
        self
    }

    pub fn add_debug_flags(&self, flags: u32) -> &Self {
        unsafe {
            obx_opt_add_debug_flags(self.obx_opt, flags);
        }
        self
    }

    pub fn async_max_queue_length(&self, value: usize) -> &Self {
        unsafe {
            obx_opt_async_max_queue_length(self.obx_opt, value);
        }
        self
    }

    pub fn async_throttle_at_queue_length(&self, value: usize) -> &Self {
        unsafe {
            obx_opt_async_throttle_at_queue_length(self.obx_opt, value);
        }
        self
    }

    pub fn async_throttle_micros(&self, value: u32) -> &Self {
        unsafe {
            obx_opt_async_throttle_micros(self.obx_opt, value);
        }
        self
    }

    pub fn async_max_in_tx_duration(&self, micros: u32) -> &Self {
        unsafe {
            obx_opt_async_max_in_tx_duration(self.obx_opt, micros);
        }
        self
    }

    pub fn async_max_in_tx_operations(&self, value: u32) -> &Self {
        unsafe {
            obx_opt_async_max_in_tx_operations(self.obx_opt, value);
        }
        self
    }

    pub fn async_pre_txn_delay(&self, delay_micros: u32) -> &Self {
        unsafe {
            obx_opt_async_pre_txn_delay(self.obx_opt, delay_micros);
        }
        self
    }

    pub fn async_pre_txn_delay4(
        &mut self,
        delay_micros: u32,
        delay2_micros: u32,
        min_queue_length_for_delay2: usize,
    ) -> &Self {
        unsafe {
            obx_opt_async_pre_txn_delay4(
                self.obx_opt,
                delay_micros,
                delay2_micros,
                min_queue_length_for_delay2,
            );
        }
        self
    }

    pub fn async_post_txn_delay(&self, delay_micros: u32) -> &Self {
        unsafe {
            obx_opt_async_post_txn_delay(self.obx_opt, delay_micros);
        }
        self
    }

    pub fn async_post_txn_delay5(
        &mut self,
        delay_micros: u32,
        delay2_micros: u32,
        min_queue_length_for_delay2: usize,
        subtract_processing_time: bool,
    ) -> &Self {
        unsafe {
            obx_opt_async_post_txn_delay5(
                self.obx_opt,
                delay_micros,
                delay2_micros,
                min_queue_length_for_delay2,
                subtract_processing_time,
            );
        }
        self
    }

    pub fn async_minor_refill_threshold(&self, queue_length: usize) -> &Self {
        unsafe {
            obx_opt_async_minor_refill_threshold(self.obx_opt, queue_length);
        }
        self
    }

    pub fn async_minor_refill_max_count(&self, value: u32) -> &Self {
        unsafe {
            obx_opt_async_minor_refill_max_count(self.obx_opt, value);
        }
        self
    }

    pub fn async_max_tx_pool_size(&self, value: usize) -> &Self {
        unsafe {
            obx_opt_async_max_tx_pool_size(self.obx_opt, value);
        }
        self
    }

    pub fn async_object_bytes_max_cache_size(&self, value: u64) -> &Self {
        unsafe {
            obx_opt_async_object_bytes_max_cache_size(self.obx_opt, value);
        }
        self
    }

    pub fn async_object_bytes_max_size_to_cache(&self, value: u64) -> &Self {
        unsafe {
            obx_opt_async_object_bytes_max_size_to_cache(self.obx_opt, value);
        }
        self
    }

    pub fn get_directory(&self) -> &str {
        unsafe {
            let c_str = obx_opt_get_directory(self.obx_opt);
            if let Ok(r) = CStr::from_ptr(c_str).to_str() {
                r
            } else {
                panic!("Error: can't get directory");
            }
        }
    }

    pub fn get_max_db_size_in_kb(&self) -> u64 {
        unsafe { obx_opt_get_max_db_size_in_kb(self.obx_opt) }
    }

    pub fn get_max_data_size_in_kb(&self) -> u64 {
        unsafe { obx_opt_get_max_data_size_in_kb(self.obx_opt) }
    }

    pub fn get_debug_flags(&self) -> u32 {
        unsafe { obx_opt_get_debug_flags(self.obx_opt) }
    }

    pub fn get_obx_options(&self) -> *mut OBX_store_options {
        self.obx_opt
    }
}
