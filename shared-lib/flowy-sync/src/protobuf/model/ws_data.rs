// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `ws_data.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct ClientRevisionWSData {
    // message fields
    pub object_id: ::std::string::String,
    pub ty: ClientRevisionWSDataType,
    pub revisions: ::protobuf::SingularPtrField<super::revision::RepeatedRevision>,
    pub data_id: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ClientRevisionWSData {
    fn default() -> &'a ClientRevisionWSData {
        <ClientRevisionWSData as ::protobuf::Message>::default_instance()
    }
}

impl ClientRevisionWSData {
    pub fn new() -> ClientRevisionWSData {
        ::std::default::Default::default()
    }

    // string object_id = 1;


    pub fn get_object_id(&self) -> &str {
        &self.object_id
    }
    pub fn clear_object_id(&mut self) {
        self.object_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_object_id(&mut self, v: ::std::string::String) {
        self.object_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object_id(&mut self) -> &mut ::std::string::String {
        &mut self.object_id
    }

    // Take field
    pub fn take_object_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.object_id, ::std::string::String::new())
    }

    // .ClientRevisionWSDataType ty = 2;


    pub fn get_ty(&self) -> ClientRevisionWSDataType {
        self.ty
    }
    pub fn clear_ty(&mut self) {
        self.ty = ClientRevisionWSDataType::ClientPushRev;
    }

    // Param is passed by value, moved
    pub fn set_ty(&mut self, v: ClientRevisionWSDataType) {
        self.ty = v;
    }

    // .RepeatedRevision revisions = 3;


    pub fn get_revisions(&self) -> &super::revision::RepeatedRevision {
        self.revisions.as_ref().unwrap_or_else(|| <super::revision::RepeatedRevision as ::protobuf::Message>::default_instance())
    }
    pub fn clear_revisions(&mut self) {
        self.revisions.clear();
    }

    pub fn has_revisions(&self) -> bool {
        self.revisions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revisions(&mut self, v: super::revision::RepeatedRevision) {
        self.revisions = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revisions(&mut self) -> &mut super::revision::RepeatedRevision {
        if self.revisions.is_none() {
            self.revisions.set_default();
        }
        self.revisions.as_mut().unwrap()
    }

    // Take field
    pub fn take_revisions(&mut self) -> super::revision::RepeatedRevision {
        self.revisions.take().unwrap_or_else(|| super::revision::RepeatedRevision::new())
    }

    // string data_id = 4;


    pub fn get_data_id(&self) -> &str {
        &self.data_id
    }
    pub fn clear_data_id(&mut self) {
        self.data_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_data_id(&mut self, v: ::std::string::String) {
        self.data_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data_id(&mut self) -> &mut ::std::string::String {
        &mut self.data_id
    }

    // Take field
    pub fn take_data_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.data_id, ::std::string::String::new())
    }
}

impl ::protobuf::Message for ClientRevisionWSData {
    fn is_initialized(&self) -> bool {
        for v in &self.revisions {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.object_id)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.ty, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.revisions)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.data_id)?;
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
        if !self.object_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.object_id);
        }
        if self.ty != ClientRevisionWSDataType::ClientPushRev {
            my_size += ::protobuf::rt::enum_size(2, self.ty);
        }
        if let Some(ref v) = self.revisions.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.data_id.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.data_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.object_id.is_empty() {
            os.write_string(1, &self.object_id)?;
        }
        if self.ty != ClientRevisionWSDataType::ClientPushRev {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.ty))?;
        }
        if let Some(ref v) = self.revisions.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.data_id.is_empty() {
            os.write_string(4, &self.data_id)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ClientRevisionWSData {
        ClientRevisionWSData::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "object_id",
                |m: &ClientRevisionWSData| { &m.object_id },
                |m: &mut ClientRevisionWSData| { &mut m.object_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ClientRevisionWSDataType>>(
                "ty",
                |m: &ClientRevisionWSData| { &m.ty },
                |m: &mut ClientRevisionWSData| { &mut m.ty },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::revision::RepeatedRevision>>(
                "revisions",
                |m: &ClientRevisionWSData| { &m.revisions },
                |m: &mut ClientRevisionWSData| { &mut m.revisions },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "data_id",
                |m: &ClientRevisionWSData| { &m.data_id },
                |m: &mut ClientRevisionWSData| { &mut m.data_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ClientRevisionWSData>(
                "ClientRevisionWSData",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ClientRevisionWSData {
        static instance: ::protobuf::rt::LazyV2<ClientRevisionWSData> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ClientRevisionWSData::new)
    }
}

impl ::protobuf::Clear for ClientRevisionWSData {
    fn clear(&mut self) {
        self.object_id.clear();
        self.ty = ClientRevisionWSDataType::ClientPushRev;
        self.revisions.clear();
        self.data_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientRevisionWSData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientRevisionWSData {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServerRevisionWSData {
    // message fields
    pub object_id: ::std::string::String,
    pub ty: ServerRevisionWSDataType,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ServerRevisionWSData {
    fn default() -> &'a ServerRevisionWSData {
        <ServerRevisionWSData as ::protobuf::Message>::default_instance()
    }
}

impl ServerRevisionWSData {
    pub fn new() -> ServerRevisionWSData {
        ::std::default::Default::default()
    }

    // string object_id = 1;


    pub fn get_object_id(&self) -> &str {
        &self.object_id
    }
    pub fn clear_object_id(&mut self) {
        self.object_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_object_id(&mut self, v: ::std::string::String) {
        self.object_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object_id(&mut self) -> &mut ::std::string::String {
        &mut self.object_id
    }

    // Take field
    pub fn take_object_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.object_id, ::std::string::String::new())
    }

    // .ServerRevisionWSDataType ty = 2;


    pub fn get_ty(&self) -> ServerRevisionWSDataType {
        self.ty
    }
    pub fn clear_ty(&mut self) {
        self.ty = ServerRevisionWSDataType::ServerAck;
    }

    // Param is passed by value, moved
    pub fn set_ty(&mut self, v: ServerRevisionWSDataType) {
        self.ty = v;
    }

    // bytes data = 3;


    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for ServerRevisionWSData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.object_id)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.ty, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.object_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.object_id);
        }
        if self.ty != ServerRevisionWSDataType::ServerAck {
            my_size += ::protobuf::rt::enum_size(2, self.ty);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.object_id.is_empty() {
            os.write_string(1, &self.object_id)?;
        }
        if self.ty != ServerRevisionWSDataType::ServerAck {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.ty))?;
        }
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ServerRevisionWSData {
        ServerRevisionWSData::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "object_id",
                |m: &ServerRevisionWSData| { &m.object_id },
                |m: &mut ServerRevisionWSData| { &mut m.object_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ServerRevisionWSDataType>>(
                "ty",
                |m: &ServerRevisionWSData| { &m.ty },
                |m: &mut ServerRevisionWSData| { &mut m.ty },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "data",
                |m: &ServerRevisionWSData| { &m.data },
                |m: &mut ServerRevisionWSData| { &mut m.data },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ServerRevisionWSData>(
                "ServerRevisionWSData",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ServerRevisionWSData {
        static instance: ::protobuf::rt::LazyV2<ServerRevisionWSData> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ServerRevisionWSData::new)
    }
}

impl ::protobuf::Clear for ServerRevisionWSData {
    fn clear(&mut self) {
        self.object_id.clear();
        self.ty = ServerRevisionWSDataType::ServerAck;
        self.data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServerRevisionWSData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerRevisionWSData {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NewDocumentUser {
    // message fields
    pub user_id: ::std::string::String,
    pub doc_id: ::std::string::String,
    pub revision_data: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a NewDocumentUser {
    fn default() -> &'a NewDocumentUser {
        <NewDocumentUser as ::protobuf::Message>::default_instance()
    }
}

impl NewDocumentUser {
    pub fn new() -> NewDocumentUser {
        ::std::default::Default::default()
    }

    // string user_id = 1;


    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
    pub fn clear_user_id(&mut self) {
        self.user_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: ::std::string::String) {
        self.user_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_id(&mut self) -> &mut ::std::string::String {
        &mut self.user_id
    }

    // Take field
    pub fn take_user_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.user_id, ::std::string::String::new())
    }

    // string doc_id = 2;


    pub fn get_doc_id(&self) -> &str {
        &self.doc_id
    }
    pub fn clear_doc_id(&mut self) {
        self.doc_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_doc_id(&mut self, v: ::std::string::String) {
        self.doc_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_doc_id(&mut self) -> &mut ::std::string::String {
        &mut self.doc_id
    }

    // Take field
    pub fn take_doc_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.doc_id, ::std::string::String::new())
    }

    // bytes revision_data = 3;


    pub fn get_revision_data(&self) -> &[u8] {
        &self.revision_data
    }
    pub fn clear_revision_data(&mut self) {
        self.revision_data.clear();
    }

    // Param is passed by value, moved
    pub fn set_revision_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.revision_data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revision_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.revision_data
    }

    // Take field
    pub fn take_revision_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.revision_data, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for NewDocumentUser {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.user_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.doc_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.revision_data)?;
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
        if !self.user_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.user_id);
        }
        if !self.doc_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.doc_id);
        }
        if !self.revision_data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.revision_data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.user_id.is_empty() {
            os.write_string(1, &self.user_id)?;
        }
        if !self.doc_id.is_empty() {
            os.write_string(2, &self.doc_id)?;
        }
        if !self.revision_data.is_empty() {
            os.write_bytes(3, &self.revision_data)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> NewDocumentUser {
        NewDocumentUser::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "user_id",
                |m: &NewDocumentUser| { &m.user_id },
                |m: &mut NewDocumentUser| { &mut m.user_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "doc_id",
                |m: &NewDocumentUser| { &m.doc_id },
                |m: &mut NewDocumentUser| { &mut m.doc_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "revision_data",
                |m: &NewDocumentUser| { &m.revision_data },
                |m: &mut NewDocumentUser| { &mut m.revision_data },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<NewDocumentUser>(
                "NewDocumentUser",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static NewDocumentUser {
        static instance: ::protobuf::rt::LazyV2<NewDocumentUser> = ::protobuf::rt::LazyV2::INIT;
        instance.get(NewDocumentUser::new)
    }
}

impl ::protobuf::Clear for NewDocumentUser {
    fn clear(&mut self) {
        self.user_id.clear();
        self.doc_id.clear();
        self.revision_data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NewDocumentUser {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NewDocumentUser {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ClientRevisionWSDataType {
    ClientPushRev = 0,
    ClientPing = 1,
}

impl ::protobuf::ProtobufEnum for ClientRevisionWSDataType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ClientRevisionWSDataType> {
        match value {
            0 => ::std::option::Option::Some(ClientRevisionWSDataType::ClientPushRev),
            1 => ::std::option::Option::Some(ClientRevisionWSDataType::ClientPing),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ClientRevisionWSDataType] = &[
            ClientRevisionWSDataType::ClientPushRev,
            ClientRevisionWSDataType::ClientPing,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ClientRevisionWSDataType>("ClientRevisionWSDataType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ClientRevisionWSDataType {
}

impl ::std::default::Default for ClientRevisionWSDataType {
    fn default() -> Self {
        ClientRevisionWSDataType::ClientPushRev
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientRevisionWSDataType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ServerRevisionWSDataType {
    ServerAck = 0,
    ServerPushRev = 1,
    ServerPullRev = 2,
    UserConnect = 3,
}

impl ::protobuf::ProtobufEnum for ServerRevisionWSDataType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ServerRevisionWSDataType> {
        match value {
            0 => ::std::option::Option::Some(ServerRevisionWSDataType::ServerAck),
            1 => ::std::option::Option::Some(ServerRevisionWSDataType::ServerPushRev),
            2 => ::std::option::Option::Some(ServerRevisionWSDataType::ServerPullRev),
            3 => ::std::option::Option::Some(ServerRevisionWSDataType::UserConnect),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ServerRevisionWSDataType] = &[
            ServerRevisionWSDataType::ServerAck,
            ServerRevisionWSDataType::ServerPushRev,
            ServerRevisionWSDataType::ServerPullRev,
            ServerRevisionWSDataType::UserConnect,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ServerRevisionWSDataType>("ServerRevisionWSDataType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ServerRevisionWSDataType {
}

impl ::std::default::Default for ServerRevisionWSDataType {
    fn default() -> Self {
        ServerRevisionWSDataType::ServerAck
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerRevisionWSDataType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rws_data.proto\x1a\x0erevision.proto\"\xa8\x01\n\x14ClientRevisionWSD\
    ata\x12\x1b\n\tobject_id\x18\x01\x20\x01(\tR\x08objectId\x12)\n\x02ty\
    \x18\x02\x20\x01(\x0e2\x19.ClientRevisionWSDataTypeR\x02ty\x12/\n\trevis\
    ions\x18\x03\x20\x01(\x0b2\x11.RepeatedRevisionR\trevisions\x12\x17\n\
    \x07data_id\x18\x04\x20\x01(\tR\x06dataId\"r\n\x14ServerRevisionWSData\
    \x12\x1b\n\tobject_id\x18\x01\x20\x01(\tR\x08objectId\x12)\n\x02ty\x18\
    \x02\x20\x01(\x0e2\x19.ServerRevisionWSDataTypeR\x02ty\x12\x12\n\x04data\
    \x18\x03\x20\x01(\x0cR\x04data\"f\n\x0fNewDocumentUser\x12\x17\n\x07user\
    _id\x18\x01\x20\x01(\tR\x06userId\x12\x15\n\x06doc_id\x18\x02\x20\x01(\t\
    R\x05docId\x12#\n\rrevision_data\x18\x03\x20\x01(\x0cR\x0crevisionData*=\
    \n\x18ClientRevisionWSDataType\x12\x11\n\rClientPushRev\x10\0\x12\x0e\n\
    \nClientPing\x10\x01*`\n\x18ServerRevisionWSDataType\x12\r\n\tServerAck\
    \x10\0\x12\x11\n\rServerPushRev\x10\x01\x12\x11\n\rServerPullRev\x10\x02\
    \x12\x0f\n\x0bUserConnect\x10\x03b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
