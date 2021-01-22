// This file is generated by rust-protobuf 2.20.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `message.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(PartialEq,Clone,Default)]
pub struct Envelope {
    // message oneof groups
    pub field_type: ::std::option::Option<Envelope_oneof_type>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Envelope {
    fn default() -> &'a Envelope {
        <Envelope as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum Envelope_oneof_type {
    ping(MessagePing),
    packet(MessagePacket),
}

impl Envelope {
    pub fn new() -> Envelope {
        ::std::default::Default::default()
    }

    // .MessagePing ping = 1;


    pub fn get_ping(&self) -> &MessagePing {
        match self.field_type {
            ::std::option::Option::Some(Envelope_oneof_type::ping(ref v)) => v,
            _ => <MessagePing as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_ping(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_ping(&self) -> bool {
        match self.field_type {
            ::std::option::Option::Some(Envelope_oneof_type::ping(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ping(&mut self, v: MessagePing) {
        self.field_type = ::std::option::Option::Some(Envelope_oneof_type::ping(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ping(&mut self) -> &mut MessagePing {
        if let ::std::option::Option::Some(Envelope_oneof_type::ping(_)) = self.field_type {
        } else {
            self.field_type = ::std::option::Option::Some(Envelope_oneof_type::ping(MessagePing::new()));
        }
        match self.field_type {
            ::std::option::Option::Some(Envelope_oneof_type::ping(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ping(&mut self) -> MessagePing {
        if self.has_ping() {
            match self.field_type.take() {
                ::std::option::Option::Some(Envelope_oneof_type::ping(v)) => v,
                _ => panic!(),
            }
        } else {
            MessagePing::new()
        }
    }

    // .MessagePacket packet = 2;


    pub fn get_packet(&self) -> &MessagePacket {
        match self.field_type {
            ::std::option::Option::Some(Envelope_oneof_type::packet(ref v)) => v,
            _ => <MessagePacket as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_packet(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_packet(&self) -> bool {
        match self.field_type {
            ::std::option::Option::Some(Envelope_oneof_type::packet(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_packet(&mut self, v: MessagePacket) {
        self.field_type = ::std::option::Option::Some(Envelope_oneof_type::packet(v))
    }

    // Mutable pointer to the field.
    pub fn mut_packet(&mut self) -> &mut MessagePacket {
        if let ::std::option::Option::Some(Envelope_oneof_type::packet(_)) = self.field_type {
        } else {
            self.field_type = ::std::option::Option::Some(Envelope_oneof_type::packet(MessagePacket::new()));
        }
        match self.field_type {
            ::std::option::Option::Some(Envelope_oneof_type::packet(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_packet(&mut self) -> MessagePacket {
        if self.has_packet() {
            match self.field_type.take() {
                ::std::option::Option::Some(Envelope_oneof_type::packet(v)) => v,
                _ => panic!(),
            }
        } else {
            MessagePacket::new()
        }
    }
}

impl ::protobuf::Message for Envelope {
    fn is_initialized(&self) -> bool {
        if let Some(Envelope_oneof_type::ping(ref v)) = self.field_type {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Envelope_oneof_type::packet(ref v)) = self.field_type {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.field_type = ::std::option::Option::Some(Envelope_oneof_type::ping(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.field_type = ::std::option::Option::Some(Envelope_oneof_type::packet(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.field_type {
            match v {
                &Envelope_oneof_type::ping(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Envelope_oneof_type::packet(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.field_type {
            match v {
                &Envelope_oneof_type::ping(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Envelope_oneof_type::packet(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

    fn new() -> Envelope {
        Envelope::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, MessagePing>(
                "ping",
                Envelope::has_ping,
                Envelope::get_ping,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, MessagePacket>(
                "packet",
                Envelope::has_packet,
                Envelope::get_packet,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Envelope>(
                "Envelope",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Envelope {
        static instance: ::protobuf::rt::LazyV2<Envelope> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Envelope::new)
    }
}

impl ::protobuf::Clear for Envelope {
    fn clear(&mut self) {
        self.field_type = ::std::option::Option::None;
        self.field_type = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Envelope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Envelope {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MessagePing {
    // message fields
    pub Timestamp: i64,
    pub LocalAddr: ::std::string::String,
    pub LocalPrivateAddr: ::std::string::String,
    pub IP: ::std::string::String,
    pub DC: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MessagePing {
    fn default() -> &'a MessagePing {
        <MessagePing as ::protobuf::Message>::default_instance()
    }
}

impl MessagePing {
    pub fn new() -> MessagePing {
        ::std::default::Default::default()
    }

    // int64 Timestamp = 1;


    pub fn get_Timestamp(&self) -> i64 {
        self.Timestamp
    }
    pub fn clear_Timestamp(&mut self) {
        self.Timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_Timestamp(&mut self, v: i64) {
        self.Timestamp = v;
    }

    // string LocalAddr = 2;


    pub fn get_LocalAddr(&self) -> &str {
        &self.LocalAddr
    }
    pub fn clear_LocalAddr(&mut self) {
        self.LocalAddr.clear();
    }

    // Param is passed by value, moved
    pub fn set_LocalAddr(&mut self, v: ::std::string::String) {
        self.LocalAddr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_LocalAddr(&mut self) -> &mut ::std::string::String {
        &mut self.LocalAddr
    }

    // Take field
    pub fn take_LocalAddr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.LocalAddr, ::std::string::String::new())
    }

    // string LocalPrivateAddr = 3;


    pub fn get_LocalPrivateAddr(&self) -> &str {
        &self.LocalPrivateAddr
    }
    pub fn clear_LocalPrivateAddr(&mut self) {
        self.LocalPrivateAddr.clear();
    }

    // Param is passed by value, moved
    pub fn set_LocalPrivateAddr(&mut self, v: ::std::string::String) {
        self.LocalPrivateAddr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_LocalPrivateAddr(&mut self) -> &mut ::std::string::String {
        &mut self.LocalPrivateAddr
    }

    // Take field
    pub fn take_LocalPrivateAddr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.LocalPrivateAddr, ::std::string::String::new())
    }

    // string IP = 4;


    pub fn get_IP(&self) -> &str {
        &self.IP
    }
    pub fn clear_IP(&mut self) {
        self.IP.clear();
    }

    // Param is passed by value, moved
    pub fn set_IP(&mut self, v: ::std::string::String) {
        self.IP = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_IP(&mut self) -> &mut ::std::string::String {
        &mut self.IP
    }

    // Take field
    pub fn take_IP(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.IP, ::std::string::String::new())
    }

    // string DC = 5;


    pub fn get_DC(&self) -> &str {
        &self.DC
    }
    pub fn clear_DC(&mut self) {
        self.DC.clear();
    }

    // Param is passed by value, moved
    pub fn set_DC(&mut self, v: ::std::string::String) {
        self.DC = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_DC(&mut self) -> &mut ::std::string::String {
        &mut self.DC
    }

    // Take field
    pub fn take_DC(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.DC, ::std::string::String::new())
    }
}

impl ::protobuf::Message for MessagePing {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.Timestamp = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.LocalAddr)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.LocalPrivateAddr)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.IP)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.DC)?;
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
        if self.Timestamp != 0 {
            my_size += ::protobuf::rt::value_size(1, self.Timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.LocalAddr.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.LocalAddr);
        }
        if !self.LocalPrivateAddr.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.LocalPrivateAddr);
        }
        if !self.IP.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.IP);
        }
        if !self.DC.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.DC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.Timestamp != 0 {
            os.write_int64(1, self.Timestamp)?;
        }
        if !self.LocalAddr.is_empty() {
            os.write_string(2, &self.LocalAddr)?;
        }
        if !self.LocalPrivateAddr.is_empty() {
            os.write_string(3, &self.LocalPrivateAddr)?;
        }
        if !self.IP.is_empty() {
            os.write_string(4, &self.IP)?;
        }
        if !self.DC.is_empty() {
            os.write_string(5, &self.DC)?;
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

    fn new() -> MessagePing {
        MessagePing::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "Timestamp",
                |m: &MessagePing| { &m.Timestamp },
                |m: &mut MessagePing| { &mut m.Timestamp },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "LocalAddr",
                |m: &MessagePing| { &m.LocalAddr },
                |m: &mut MessagePing| { &mut m.LocalAddr },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "LocalPrivateAddr",
                |m: &MessagePing| { &m.LocalPrivateAddr },
                |m: &mut MessagePing| { &mut m.LocalPrivateAddr },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "IP",
                |m: &MessagePing| { &m.IP },
                |m: &mut MessagePing| { &mut m.IP },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "DC",
                |m: &MessagePing| { &m.DC },
                |m: &mut MessagePing| { &mut m.DC },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MessagePing>(
                "MessagePing",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MessagePing {
        static instance: ::protobuf::rt::LazyV2<MessagePing> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MessagePing::new)
    }
}

impl ::protobuf::Clear for MessagePing {
    fn clear(&mut self) {
        self.Timestamp = 0;
        self.LocalAddr.clear();
        self.LocalPrivateAddr.clear();
        self.IP.clear();
        self.DC.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MessagePing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MessagePing {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MessagePacket {
    // message fields
    pub payload: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MessagePacket {
    fn default() -> &'a MessagePacket {
        <MessagePacket as ::protobuf::Message>::default_instance()
    }
}

impl MessagePacket {
    pub fn new() -> MessagePacket {
        ::std::default::Default::default()
    }

    // bytes payload = 1;


    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.payload
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.payload, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for MessagePacket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.payload)?;
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
        if !self.payload.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.payload);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.payload.is_empty() {
            os.write_bytes(1, &self.payload)?;
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

    fn new() -> MessagePacket {
        MessagePacket::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "payload",
                |m: &MessagePacket| { &m.payload },
                |m: &mut MessagePacket| { &mut m.payload },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MessagePacket>(
                "MessagePacket",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MessagePacket {
        static instance: ::protobuf::rt::LazyV2<MessagePacket> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MessagePacket::new)
    }
}

impl ::protobuf::Clear for MessagePacket {
    fn clear(&mut self) {
        self.payload.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MessagePacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MessagePacket {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rmessage.proto\"`\n\x08Envelope\x12\"\n\x04ping\x18\x01\x20\x01(\x0b2\
    \x0c.MessagePingH\0R\x04ping\x12(\n\x06packet\x18\x02\x20\x01(\x0b2\x0e.\
    MessagePacketH\0R\x06packetB\x06\n\x04type\"\x95\x01\n\x0bMessagePing\
    \x12\x1c\n\tTimestamp\x18\x01\x20\x01(\x03R\tTimestamp\x12\x1c\n\tLocalA\
    ddr\x18\x02\x20\x01(\tR\tLocalAddr\x12*\n\x10LocalPrivateAddr\x18\x03\
    \x20\x01(\tR\x10LocalPrivateAddr\x12\x0e\n\x02IP\x18\x04\x20\x01(\tR\x02\
    IP\x12\x0e\n\x02DC\x18\x05\x20\x01(\tR\x02DC\")\n\rMessagePacket\x12\x18\
    \n\x07payload\x18\x01\x20\x01(\x0cR\x07payloadB\x0fZ\rqtun/protocolJ\x9d\
    \x05\n\x06\x12\x04\0\0\x15\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\
    \x01\x08\x12\x03\x02\0$\n\t\n\x02\x08\x0b\x12\x03\x02\0$\n\n\n\x02\x04\0\
    \x12\x04\x04\0\t\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\x10\n\x0c\n\x04\
    \x04\0\x08\0\x12\x04\x05\x08\x08\t\n\x0c\n\x05\x04\0\x08\0\x01\x12\x03\
    \x05\x0e\x12\n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x10%\n\x0c\n\x05\x04\0\
    \x02\0\x06\x12\x03\x06\x10\x1b\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\
    \x1c\x20\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x06#$\n\x0b\n\x04\x04\0\x02\
    \x01\x12\x03\x07\x10)\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x07\x10\x1d\
    \n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x07\x1e$\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x07'(\n\n\n\x02\x04\x01\x12\x04\x0b\0\x11\x01\n\n\n\x03\
    \x04\x01\x01\x12\x03\x0b\x08\x13\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0c\
    \x08\x1d\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x0c\x08\x0b\x15\n\x0c\n\x05\
    \x04\x01\x02\0\x05\x12\x03\x0c\x08\r\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x03\x0c\x0f\x18\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0c\x1b\x1c\n\x0b\
    \n\x04\x04\x01\x02\x01\x12\x03\r\x08\x1d\n\r\n\x05\x04\x01\x02\x01\x04\
    \x12\x04\r\x08\x0c\x1d\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\r\x08\x0e\
    \n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\r\x0f\x18\n\x0c\n\x05\x04\x01\
    \x02\x01\x03\x12\x03\r\x1b\x1c\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x0e\
    \x08$\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\x0e\x08\r\x1d\n\x0c\n\x05\
    \x04\x01\x02\x02\x05\x12\x03\x0e\x08\x0e\n\x0c\n\x05\x04\x01\x02\x02\x01\
    \x12\x03\x0e\x0f\x1f\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x0e\"#\n\
    \x0b\n\x04\x04\x01\x02\x03\x12\x03\x0f\x08\x16\n\r\n\x05\x04\x01\x02\x03\
    \x04\x12\x04\x0f\x08\x0e$\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x0f\
    \x08\x0e\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x0f\x0f\x11\n\x0c\n\x05\
    \x04\x01\x02\x03\x03\x12\x03\x0f\x14\x15\n\x0b\n\x04\x04\x01\x02\x04\x12\
    \x03\x10\x08\x16\n\r\n\x05\x04\x01\x02\x04\x04\x12\x04\x10\x08\x0f\x16\n\
    \x0c\n\x05\x04\x01\x02\x04\x05\x12\x03\x10\x08\x0e\n\x0c\n\x05\x04\x01\
    \x02\x04\x01\x12\x03\x10\x0f\x11\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\
    \x10\x14\x15\n\n\n\x02\x04\x02\x12\x04\x13\0\x15\x01\n\n\n\x03\x04\x02\
    \x01\x12\x03\x13\x08\x15\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x14\x08\x1a\n\
    \r\n\x05\x04\x02\x02\0\x04\x12\x04\x14\x08\x13\x17\n\x0c\n\x05\x04\x02\
    \x02\0\x05\x12\x03\x14\x08\r\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x14\
    \x0e\x15\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x14\x18\x19b\x06proto3\
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