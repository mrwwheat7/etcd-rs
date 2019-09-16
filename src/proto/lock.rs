// This file is generated by rust-protobuf 2.8.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `lock.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_1;

#[derive(PartialEq,Clone,Default)]
pub struct LockRequest {
    // message fields
    pub name: ::std::vec::Vec<u8>,
    pub lease: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LockRequest {
    fn default() -> &'a LockRequest {
        <LockRequest as ::protobuf::Message>::default_instance()
    }
}

impl LockRequest {
    pub fn new() -> LockRequest {
        ::std::default::Default::default()
    }

    // bytes name = 1;


    pub fn get_name(&self) -> &[u8] {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.name, ::std::vec::Vec::new())
    }

    // int64 lease = 2;


    pub fn get_lease(&self) -> i64 {
        self.lease
    }
    pub fn clear_lease(&mut self) {
        self.lease = 0;
    }

    // Param is passed by value, moved
    pub fn set_lease(&mut self, v: i64) {
        self.lease = v;
    }
}

impl ::protobuf::Message for LockRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.lease = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.name);
        }
        if self.lease != 0 {
            my_size += ::protobuf::rt::value_size(2, self.lease, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_bytes(1, &self.name)?;
        }
        if self.lease != 0 {
            os.write_int64(2, self.lease)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> LockRequest {
        LockRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    |m: &LockRequest| { &m.name },
                    |m: &mut LockRequest| { &mut m.name },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "lease",
                    |m: &LockRequest| { &m.lease },
                    |m: &mut LockRequest| { &mut m.lease },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LockRequest>(
                    "LockRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static LockRequest {
        static mut instance: ::protobuf::lazy::Lazy<LockRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LockRequest,
        };
        unsafe {
            instance.get(LockRequest::new)
        }
    }
}

impl ::protobuf::Clear for LockRequest {
    fn clear(&mut self) {
        self.name.clear();
        self.lease = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LockRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LockRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LockResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::rpc::ResponseHeader>,
    pub key: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LockResponse {
    fn default() -> &'a LockResponse {
        <LockResponse as ::protobuf::Message>::default_instance()
    }
}

impl LockResponse {
    pub fn new() -> LockResponse {
        ::std::default::Default::default()
    }

    // .etcdserverpb.ResponseHeader header = 1;


    pub fn get_header(&self) -> &super::rpc::ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| super::rpc::ResponseHeader::default_instance())
    }
    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::rpc::ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::rpc::ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::rpc::ResponseHeader {
        self.header.take().unwrap_or_else(|| super::rpc::ResponseHeader::new())
    }

    // bytes key = 2;


    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for LockResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> LockResponse {
        LockResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::ResponseHeader>>(
                    "header",
                    |m: &LockResponse| { &m.header },
                    |m: &mut LockResponse| { &mut m.header },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    |m: &LockResponse| { &m.key },
                    |m: &mut LockResponse| { &mut m.key },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LockResponse>(
                    "LockResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static LockResponse {
        static mut instance: ::protobuf::lazy::Lazy<LockResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LockResponse,
        };
        unsafe {
            instance.get(LockResponse::new)
        }
    }
}

impl ::protobuf::Clear for LockResponse {
    fn clear(&mut self) {
        self.header.clear();
        self.key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LockResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LockResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnlockRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UnlockRequest {
    fn default() -> &'a UnlockRequest {
        <UnlockRequest as ::protobuf::Message>::default_instance()
    }
}

impl UnlockRequest {
    pub fn new() -> UnlockRequest {
        ::std::default::Default::default()
    }

    // bytes key = 1;


    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for UnlockRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> UnlockRequest {
        UnlockRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    |m: &UnlockRequest| { &m.key },
                    |m: &mut UnlockRequest| { &mut m.key },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnlockRequest>(
                    "UnlockRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static UnlockRequest {
        static mut instance: ::protobuf::lazy::Lazy<UnlockRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnlockRequest,
        };
        unsafe {
            instance.get(UnlockRequest::new)
        }
    }
}

impl ::protobuf::Clear for UnlockRequest {
    fn clear(&mut self) {
        self.key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnlockRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnlockRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnlockResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::rpc::ResponseHeader>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UnlockResponse {
    fn default() -> &'a UnlockResponse {
        <UnlockResponse as ::protobuf::Message>::default_instance()
    }
}

impl UnlockResponse {
    pub fn new() -> UnlockResponse {
        ::std::default::Default::default()
    }

    // .etcdserverpb.ResponseHeader header = 1;


    pub fn get_header(&self) -> &super::rpc::ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| super::rpc::ResponseHeader::default_instance())
    }
    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::rpc::ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::rpc::ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::rpc::ResponseHeader {
        self.header.take().unwrap_or_else(|| super::rpc::ResponseHeader::new())
    }
}

impl ::protobuf::Message for UnlockResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> UnlockResponse {
        UnlockResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rpc::ResponseHeader>>(
                    "header",
                    |m: &UnlockResponse| { &m.header },
                    |m: &mut UnlockResponse| { &mut m.header },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnlockResponse>(
                    "UnlockResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static UnlockResponse {
        static mut instance: ::protobuf::lazy::Lazy<UnlockResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnlockResponse,
        };
        unsafe {
            instance.get(UnlockResponse::new)
        }
    }
}

impl ::protobuf::Clear for UnlockResponse {
    fn clear(&mut self) {
        self.header.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnlockResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnlockResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nlock.proto\x12\x08v3lockpb\x1a\trpc.proto\"7\n\x0bLockRequest\x12\
    \x12\n\x04name\x18\x01\x20\x01(\x0cR\x04name\x12\x14\n\x05lease\x18\x02\
    \x20\x01(\x03R\x05lease\"V\n\x0cLockResponse\x124\n\x06header\x18\x01\
    \x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header\x12\x10\n\x03k\
    ey\x18\x02\x20\x01(\x0cR\x03key\"!\n\rUnlockRequest\x12\x10\n\x03key\x18\
    \x01\x20\x01(\x0cR\x03key\"F\n\x0eUnlockResponse\x124\n\x06header\x18\
    \x01\x20\x01(\x0b2\x1c.etcdserverpb.ResponseHeaderR\x06header2~\n\x04Loc\
    k\x127\n\x04Lock\x12\x15.v3lockpb.LockRequest\x1a\x16.v3lockpb.LockRespo\
    nse\"\0\x12=\n\x06Unlock\x12\x17.v3lockpb.UnlockRequest\x1a\x18.v3lockpb\
    .UnlockResponse\"\0B.\n\x16io.etcd.jetcd.api.lockB\nJetcdProtoP\x01\xa2\
    \x02\x05JetcdJ\xa5\x14\n\x06\x12\x04\x10\0E\x01\n\xc5\x04\n\x01\x0c\x12\
    \x03\x10\0\x122\xba\x04\n\x20Copyright\x202017\x20The\x20jetcd\x20author\
    s\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\x202.\
    0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20file\
    \x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20ma\
    y\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\
    \x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20requ\
    ired\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\
    \x20software\n\x20distributed\x20under\x20the\x20License\x20is\x20distri\
    buted\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\
    \x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\
    \x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20lan\
    guage\x20governing\x20permissions\x20and\n\x20limitations\x20under\x20th\
    e\x20License.\n\n\n\t\n\x02\x03\0\x12\x03\x12\0\x13\n\x08\n\x01\x02\x12\
    \x03\x14\0\x11\n\x08\n\x01\x08\x12\x03\x16\0\"\n\t\n\x02\x08\n\x12\x03\
    \x16\0\"\n\x08\n\x01\x08\x12\x03\x17\0/\n\t\n\x02\x08\x01\x12\x03\x17\0/\
    \n\x08\n\x01\x08\x12\x03\x18\0+\n\t\n\x02\x08\x08\x12\x03\x18\0+\n\x08\n\
    \x01\x08\x12\x03\x19\0#\n\t\n\x02\x08$\x12\x03\x19\0#\nZ\n\x02\x06\0\x12\
    \x04\x1c\0)\x01\x1aN\x20The\x20lock\x20service\x20exposes\x20client-side\
    \x20locking\x20facilities\x20as\x20a\x20gRPC\x20interface.\n\n\n\n\x03\
    \x06\0\x01\x12\x03\x1c\x08\x0c\n\x94\x03\n\x04\x06\0\x02\0\x12\x03#\x043\
    \x1a\x86\x03\x20Lock\x20acquires\x20a\x20distributed\x20shared\x20lock\
    \x20on\x20a\x20given\x20named\x20lock.\n\x20On\x20success,\x20it\x20will\
    \x20return\x20a\x20unique\x20key\x20that\x20exists\x20so\x20long\x20as\
    \x20the\n\x20lock\x20is\x20held\x20by\x20the\x20caller.\x20This\x20key\
    \x20can\x20be\x20used\x20in\x20conjunction\x20with\n\x20transactions\x20\
    to\x20safely\x20ensure\x20updates\x20to\x20etcd\x20only\x20occur\x20whil\
    e\x20holding\n\x20lock\x20ownership.\x20The\x20lock\x20is\x20held\x20unt\
    il\x20Unlock\x20is\x20called\x20on\x20the\x20key\x20or\x20the\n\x20lease\
    \x20associate\x20with\x20the\x20owner\x20expires.\n\n\x0c\n\x05\x06\0\
    \x02\0\x01\x12\x03#\x08\x0c\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03#\r\x18\n\
    \x0c\n\x05\x06\0\x02\0\x03\x12\x03##/\n\xb5\x01\n\x04\x06\0\x02\x01\x12\
    \x03(\x049\x1a\xa7\x01\x20Unlock\x20takes\x20a\x20key\x20returned\x20by\
    \x20Lock\x20and\x20releases\x20the\x20hold\x20on\x20lock.\x20The\n\x20ne\
    xt\x20Lock\x20caller\x20waiting\x20for\x20the\x20lock\x20will\x20then\
    \x20be\x20woken\x20up\x20and\x20given\n\x20ownership\x20of\x20the\x20loc\
    k.\n\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03(\x08\x0e\n\x0c\n\x05\x06\0\
    \x02\x01\x02\x12\x03(\x0f\x1c\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03('5\n\
    \n\n\x02\x04\0\x12\x04+\04\x01\n\n\n\x03\x04\0\x01\x12\x03+\x08\x13\nU\n\
    \x04\x04\0\x02\0\x12\x03-\x04\x13\x1aH\x20name\x20is\x20the\x20identifie\
    r\x20for\x20the\x20distributed\x20shared\x20lock\x20to\x20be\x20acquired\
    .\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04-\x04+\x15\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03-\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03-\n\x0e\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03-\x11\x12\n\xbd\x02\n\x04\x04\0\x02\x01\x12\
    \x033\x04\x14\x1a\xaf\x02\x20lease\x20is\x20the\x20ID\x20of\x20the\x20le\
    ase\x20that\x20will\x20be\x20attached\x20to\x20ownership\x20of\x20the\n\
    \x20lock.\x20If\x20the\x20lease\x20expires\x20or\x20is\x20revoked\x20and\
    \x20currently\x20holds\x20the\x20lock,\n\x20the\x20lock\x20is\x20automat\
    ically\x20released.\x20Calls\x20to\x20Lock\x20with\x20the\x20same\x20lea\
    se\x20will\n\x20be\x20treated\x20as\x20a\x20single\x20acquistion;\x20loc\
    king\x20twice\x20with\x20the\x20same\x20lease\x20is\x20a\n\x20no-op.\n\n\
    \r\n\x05\x04\0\x02\x01\x04\x12\x043\x04-\x13\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x033\x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x033\n\x0f\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x033\x12\x13\n\n\n\x02\x04\x01\x12\x046\0<\
    \x01\n\n\n\x03\x04\x01\x01\x12\x036\x08\x14\n\x0b\n\x04\x04\x01\x02\0\
    \x12\x037\x04+\n\r\n\x05\x04\x01\x02\0\x04\x12\x047\x046\x16\n\x0c\n\x05\
    \x04\x01\x02\0\x06\x12\x037\x04\x1f\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x037\x20&\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x037)*\n\xb8\x01\n\x04\x04\
    \x01\x02\x01\x12\x03;\x04\x12\x1a\xaa\x01\x20key\x20is\x20a\x20key\x20th\
    at\x20will\x20exist\x20on\x20etcd\x20for\x20the\x20duration\x20that\x20t\
    he\x20Lock\x20caller\n\x20owns\x20the\x20lock.\x20Users\x20should\x20not\
    \x20modify\x20this\x20key\x20or\x20the\x20lock\x20may\x20exhibit\n\x20un\
    defined\x20behavior.\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04;\x047+\n\
    \x0c\n\x05\x04\x01\x02\x01\x05\x12\x03;\x04\t\n\x0c\n\x05\x04\x01\x02\
    \x01\x01\x12\x03;\n\r\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03;\x10\x11\n\
    \n\n\x02\x04\x02\x12\x04>\0A\x01\n\n\n\x03\x04\x02\x01\x12\x03>\x08\x15\
    \n=\n\x04\x04\x02\x02\0\x12\x03@\x04\x12\x1a0\x20key\x20is\x20the\x20loc\
    k\x20ownership\x20key\x20granted\x20by\x20Lock.\n\n\r\n\x05\x04\x02\x02\
    \0\x04\x12\x04@\x04>\x17\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03@\x04\t\n\
    \x0c\n\x05\x04\x02\x02\0\x01\x12\x03@\n\r\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03@\x10\x11\n\n\n\x02\x04\x03\x12\x04C\0E\x01\n\n\n\x03\x04\x03\
    \x01\x12\x03C\x08\x16\n\x0b\n\x04\x04\x03\x02\0\x12\x03D\x04+\n\r\n\x05\
    \x04\x03\x02\0\x04\x12\x04D\x04C\x18\n\x0c\n\x05\x04\x03\x02\0\x06\x12\
    \x03D\x04\x1f\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03D\x20&\n\x0c\n\x05\
    \x04\x03\x02\0\x03\x12\x03D)*b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
