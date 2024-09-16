//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDataReadingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSDataReadingOptions: NSUInteger {
        const NSDataReadingMappedIfSafe = 1<<0;
        const NSDataReadingUncached = 1<<1;
        const NSDataReadingMappedAlways = 1<<3;
#[deprecated]
        const NSDataReadingMapped = NSDataReadingOptions::NSDataReadingMappedIfSafe.0;
#[deprecated]
        const NSMappedRead = NSDataReadingOptions::NSDataReadingMapped.0;
#[deprecated]
        const NSUncachedRead = NSDataReadingOptions::NSDataReadingUncached.0;
    }
}

unsafe impl Encode for NSDataReadingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDataReadingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDataWritingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSDataWritingOptions: NSUInteger {
        const NSDataWritingAtomic = 1<<0;
        const NSDataWritingWithoutOverwriting = 1<<1;
        const NSDataWritingFileProtectionNone = 0x10000000;
        const NSDataWritingFileProtectionComplete = 0x20000000;
        const NSDataWritingFileProtectionCompleteUnlessOpen = 0x30000000;
        const NSDataWritingFileProtectionCompleteUntilFirstUserAuthentication = 0x40000000;
        const NSDataWritingFileProtectionCompleteWhenUserInactive = 0x50000000;
        const NSDataWritingFileProtectionMask = 0xf0000000;
#[deprecated]
        const NSAtomicWrite = NSDataWritingOptions::NSDataWritingAtomic.0;
    }
}

unsafe impl Encode for NSDataWritingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDataWritingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDataSearchOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSDataSearchOptions: NSUInteger {
        const NSDataSearchBackwards = 1<<0;
        const NSDataSearchAnchored = 1<<1;
    }
}

unsafe impl Encode for NSDataSearchOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDataSearchOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDataBase64EncodingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSDataBase64EncodingOptions: NSUInteger {
        const NSDataBase64Encoding64CharacterLineLength = 1<<0;
        const NSDataBase64Encoding76CharacterLineLength = 1<<1;
        const NSDataBase64EncodingEndLineWithCarriageReturn = 1<<4;
        const NSDataBase64EncodingEndLineWithLineFeed = 1<<5;
    }
}

unsafe impl Encode for NSDataBase64EncodingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDataBase64EncodingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDataBase64DecodingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSDataBase64DecodingOptions: NSUInteger {
        const NSDataBase64DecodingIgnoreUnknownCharacters = 1<<0;
    }
}

unsafe impl Encode for NSDataBase64DecodingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDataBase64DecodingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSData;

    unsafe impl ClassType for NSData {
        type Super = NSObject;
        type Mutability = ImmutableWithMutableSubclass<NSMutableData>;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSData {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSData {}

#[cfg(feature = "NSObject")]
unsafe impl NSMutableCopying for NSData {}

unsafe impl NSObjectProtocol for NSData {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSData {}

extern_methods!(
    unsafe impl NSData {
        #[method(length)]
        pub fn length(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSData {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for NSData {
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedData
    unsafe impl NSData {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Retained<NSString>;

        #[method(getBytes:length:)]
        pub unsafe fn getBytes_length(&self, buffer: NonNull<c_void>, length: NSUInteger);

        #[cfg(feature = "NSRange")]
        #[method(getBytes:range:)]
        pub unsafe fn getBytes_range(&self, buffer: NonNull<c_void>, range: NSRange);

        #[method(isEqualToData:)]
        pub unsafe fn isEqualToData(&self, other: &NSData) -> bool;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Other subdataWithRange:)]
        pub unsafe fn subdataWithRange(&self, range: NSRange) -> Retained<NSData>;

        #[cfg(feature = "NSString")]
        #[method(writeToFile:atomically:)]
        pub unsafe fn writeToFile_atomically(
            &self,
            path: &NSString,
            use_auxiliary_file: bool,
        ) -> bool;

        #[cfg(feature = "NSURL")]
        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method(writeToFile:options:error:_)]
        pub unsafe fn writeToFile_options_error(
            &self,
            path: &NSString,
            write_options_mask: NSDataWritingOptions,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method(writeToURL:options:error:_)]
        pub unsafe fn writeToURL_options_error(
            &self,
            url: &NSURL,
            write_options_mask: NSDataWritingOptions,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSRange")]
        #[method(rangeOfData:options:range:)]
        pub unsafe fn rangeOfData_options_range(
            &self,
            data_to_find: &NSData,
            mask: NSDataSearchOptions,
            search_range: NSRange,
        ) -> NSRange;

        #[cfg(all(feature = "NSRange", feature = "block2"))]
        #[method(enumerateByteRangesUsingBlock:)]
        pub unsafe fn enumerateByteRangesUsingBlock(
            &self,
            block: &block2::Block<dyn Fn(NonNull<c_void>, NSRange, NonNull<Bool>) + '_>,
        );
    }
);

extern_methods!(
    /// NSDataCreation
    unsafe impl NSData {
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data() -> Retained<Self>;

        #[method_id(@__retain_semantics Other dataWithBytes:length:)]
        pub unsafe fn dataWithBytes_length(
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:)]
        pub unsafe fn dataWithBytesNoCopy_length(
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn dataWithBytesNoCopy_length_freeWhenDone(
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:options:error:_)]
        pub unsafe fn dataWithContentsOfFile_options_error(
            path: &NSString,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:options:error:_)]
        pub unsafe fn dataWithContentsOfURL_options_error(
            url: &NSURL,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:)]
        pub unsafe fn dataWithContentsOfFile(path: &NSString) -> Option<Retained<Self>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:)]
        pub unsafe fn dataWithContentsOfURL(url: &NSURL) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithBytes:length:)]
        pub unsafe fn initWithBytes_length(
            this: Allocated<Self>,
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:)]
        pub unsafe fn initWithBytesNoCopy_length(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn initWithBytesNoCopy_length_freeWhenDone(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:deallocator:)]
        pub unsafe fn initWithBytesNoCopy_length_deallocator(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            deallocator: Option<&block2::Block<dyn Fn(NonNull<c_void>, NSUInteger)>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:options:error:_)]
        pub unsafe fn initWithContentsOfFile_options_error(
            this: Allocated<Self>,
            path: &NSString,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub fn initWithData(this: Allocated<Self>, data: &NSData) -> Retained<Self>;

        #[method_id(@__retain_semantics Other dataWithData:)]
        pub fn dataWithData(data: &NSData) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDataCreation
    unsafe impl NSMutableData {
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data() -> Retained<Self>;

        #[method_id(@__retain_semantics Other dataWithBytes:length:)]
        pub unsafe fn dataWithBytes_length(
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:)]
        pub unsafe fn dataWithBytesNoCopy_length(
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn dataWithBytesNoCopy_length_freeWhenDone(
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:options:error:_)]
        pub unsafe fn dataWithContentsOfFile_options_error(
            path: &NSString,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:options:error:_)]
        pub unsafe fn dataWithContentsOfURL_options_error(
            url: &NSURL,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:)]
        pub unsafe fn dataWithContentsOfFile(path: &NSString) -> Option<Retained<Self>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:)]
        pub unsafe fn dataWithContentsOfURL(url: &NSURL) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithBytes:length:)]
        pub unsafe fn initWithBytes_length(
            this: Allocated<Self>,
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:)]
        pub unsafe fn initWithBytesNoCopy_length(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn initWithBytesNoCopy_length_freeWhenDone(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:deallocator:)]
        pub unsafe fn initWithBytesNoCopy_length_deallocator(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            deallocator: Option<&block2::Block<dyn Fn(NonNull<c_void>, NSUInteger)>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:options:error:_)]
        pub unsafe fn initWithContentsOfFile_options_error(
            this: Allocated<Self>,
            path: &NSString,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(this: Allocated<Self>, data: &NSData) -> Retained<Self>;

        #[method_id(@__retain_semantics Other dataWithData:)]
        pub fn dataWithData(data: &NSData) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDataBase64Encoding
    unsafe impl NSData {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithBase64EncodedString:options:)]
        pub unsafe fn initWithBase64EncodedString_options(
            this: Allocated<Self>,
            base64_string: &NSString,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other base64EncodedStringWithOptions:)]
        pub unsafe fn base64EncodedStringWithOptions(
            &self,
            options: NSDataBase64EncodingOptions,
        ) -> Retained<NSString>;

        #[method_id(@__retain_semantics Init initWithBase64EncodedData:options:)]
        pub unsafe fn initWithBase64EncodedData_options(
            this: Allocated<Self>,
            base64_data: &NSData,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other base64EncodedDataWithOptions:)]
        pub unsafe fn base64EncodedDataWithOptions(
            &self,
            options: NSDataBase64EncodingOptions,
        ) -> Retained<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDataBase64Encoding
    unsafe impl NSMutableData {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithBase64EncodedString:options:)]
        pub unsafe fn initWithBase64EncodedString_options(
            this: Allocated<Self>,
            base64_string: &NSString,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithBase64EncodedData:options:)]
        pub unsafe fn initWithBase64EncodedData_options(
            this: Allocated<Self>,
            base64_data: &NSData,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Retained<Self>>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDataCompressionAlgorithm(pub NSInteger);
impl NSDataCompressionAlgorithm {
    #[doc(alias = "NSDataCompressionAlgorithmLZFSE")]
    pub const LZFSE: Self = Self(0);
    #[doc(alias = "NSDataCompressionAlgorithmLZ4")]
    pub const LZ4: Self = Self(1);
    #[doc(alias = "NSDataCompressionAlgorithmLZMA")]
    pub const LZMA: Self = Self(2);
    #[doc(alias = "NSDataCompressionAlgorithmZlib")]
    pub const Zlib: Self = Self(3);
}

unsafe impl Encode for NSDataCompressionAlgorithm {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSDataCompressionAlgorithm {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSDataCompression
    unsafe impl NSData {
        #[cfg(feature = "NSError")]
        #[method_id(@__retain_semantics Other decompressedDataUsingAlgorithm:error:_)]
        pub unsafe fn decompressedDataUsingAlgorithm_error(
            &self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "NSError")]
        #[method_id(@__retain_semantics Other compressedDataUsingAlgorithm:error:_)]
        pub unsafe fn compressedDataUsingAlgorithm_error(
            &self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSData {
        #[deprecated = "This method is unsafe because it could potentially cause buffer overruns. Use -getBytes:length: instead."]
        #[method(getBytes:)]
        pub unsafe fn getBytes(&self, buffer: NonNull<c_void>);

        #[cfg(feature = "NSString")]
        #[deprecated = "Use +dataWithContentsOfURL:options:error: and NSDataReadingMappedIfSafe or NSDataReadingMappedAlways instead."]
        #[method_id(@__retain_semantics Other dataWithContentsOfMappedFile:)]
        pub unsafe fn dataWithContentsOfMappedFile(path: &NSString) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use -initWithContentsOfURL:options:error: and NSDataReadingMappedIfSafe or NSDataReadingMappedAlways instead."]
        #[method_id(@__retain_semantics Init initWithContentsOfMappedFile:)]
        pub unsafe fn initWithContentsOfMappedFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use initWithBase64EncodedString:options: instead"]
        #[method_id(@__retain_semantics Init initWithBase64Encoding:)]
        pub unsafe fn initWithBase64Encoding(
            this: Allocated<Self>,
            base64_string: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use base64EncodedStringWithOptions: instead"]
        #[method_id(@__retain_semantics Other base64Encoding)]
        pub unsafe fn base64Encoding(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDeprecated
    unsafe impl NSMutableData {
        #[cfg(feature = "NSString")]
        #[deprecated = "Use -initWithContentsOfURL:options:error: and NSDataReadingMappedIfSafe or NSDataReadingMappedAlways instead."]
        #[method_id(@__retain_semantics Init initWithContentsOfMappedFile:)]
        pub unsafe fn initWithContentsOfMappedFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use initWithBase64EncodedString:options: instead"]
        #[method_id(@__retain_semantics Init initWithBase64Encoding:)]
        pub unsafe fn initWithBase64Encoding(
            this: Allocated<Self>,
            base64_string: &NSString,
        ) -> Option<Retained<Self>>;
    }
);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSMutableData;

    unsafe impl ClassType for NSMutableData {
        #[inherits(NSObject)]
        type Super = NSData;
        type Mutability = MutableWithImmutableSuperclass<NSData>;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSMutableData {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSMutableData {}

#[cfg(feature = "NSObject")]
unsafe impl NSMutableCopying for NSMutableData {}

unsafe impl NSObjectProtocol for NSMutableData {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSMutableData {}

extern_methods!(
    unsafe impl NSMutableData {
        #[method(setLength:)]
        pub fn setLength(&mut self, length: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMutableData {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for NSMutableData {
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedMutableData
    unsafe impl NSMutableData {
        #[method(appendBytes:length:)]
        pub unsafe fn appendBytes_length(&mut self, bytes: NonNull<c_void>, length: NSUInteger);

        #[method(appendData:)]
        pub unsafe fn appendData(&mut self, other: &NSData);

        #[method(increaseLengthBy:)]
        pub unsafe fn increaseLengthBy(&mut self, extra_length: NSUInteger);

        #[cfg(feature = "NSRange")]
        #[method(replaceBytesInRange:withBytes:)]
        pub unsafe fn replaceBytesInRange_withBytes(
            &mut self,
            range: NSRange,
            bytes: NonNull<c_void>,
        );

        #[cfg(feature = "NSRange")]
        #[method(resetBytesInRange:)]
        pub unsafe fn resetBytesInRange(&mut self, range: NSRange);

        #[method(setData:)]
        pub unsafe fn setData(&mut self, data: &NSData);

        #[cfg(feature = "NSRange")]
        #[method(replaceBytesInRange:withBytes:length:)]
        pub unsafe fn replaceBytesInRange_withBytes_length(
            &mut self,
            range: NSRange,
            replacement_bytes: *mut c_void,
            replacement_length: NSUInteger,
        );
    }
);

extern_methods!(
    /// NSMutableDataCreation
    unsafe impl NSMutableData {
        #[method_id(@__retain_semantics Other dataWithCapacity:)]
        pub fn dataWithCapacity(a_num_items: NSUInteger) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other dataWithLength:)]
        pub unsafe fn dataWithLength(length: NSUInteger) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub fn initWithCapacity(
            this: Allocated<Self>,
            capacity: NSUInteger,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithLength:)]
        pub unsafe fn initWithLength(
            this: Allocated<Self>,
            length: NSUInteger,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// NSMutableDataCompression
    unsafe impl NSMutableData {
        #[cfg(feature = "NSError")]
        #[method(decompressUsingAlgorithm:error:_)]
        pub unsafe fn decompressUsingAlgorithm_error(
            &mut self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSError")]
        #[method(compressUsingAlgorithm:error:_)]
        pub unsafe fn compressUsingAlgorithm_error(
            &mut self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPurgeableData;

    unsafe impl ClassType for NSPurgeableData {
        #[inherits(NSData, NSObject)]
        type Super = NSMutableData;
        type Mutability = Mutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSPurgeableData {}

#[cfg(feature = "NSObject")]
unsafe impl NSDiscardableContent for NSPurgeableData {}

unsafe impl NSObjectProtocol for NSPurgeableData {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSPurgeableData {}

extern_methods!(
    unsafe impl NSPurgeableData {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPurgeableData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
