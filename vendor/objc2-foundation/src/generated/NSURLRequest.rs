//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSURLRequestCachePolicy(pub NSUInteger);
impl NSURLRequestCachePolicy {
    pub const NSURLRequestUseProtocolCachePolicy: Self = Self(0);
    pub const NSURLRequestReloadIgnoringLocalCacheData: Self = Self(1);
    pub const NSURLRequestReloadIgnoringLocalAndRemoteCacheData: Self = Self(4);
    pub const NSURLRequestReloadIgnoringCacheData: Self =
        Self(NSURLRequestCachePolicy::NSURLRequestReloadIgnoringLocalCacheData.0);
    pub const NSURLRequestReturnCacheDataElseLoad: Self = Self(2);
    pub const NSURLRequestReturnCacheDataDontLoad: Self = Self(3);
    pub const NSURLRequestReloadRevalidatingCacheData: Self = Self(5);
}

unsafe impl Encode for NSURLRequestCachePolicy {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSURLRequestCachePolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSURLRequestNetworkServiceType(pub NSUInteger);
impl NSURLRequestNetworkServiceType {
    pub const NSURLNetworkServiceTypeDefault: Self = Self(0);
    #[deprecated = "Use PushKit for VoIP control purposes"]
    pub const NSURLNetworkServiceTypeVoIP: Self = Self(1);
    pub const NSURLNetworkServiceTypeVideo: Self = Self(2);
    pub const NSURLNetworkServiceTypeBackground: Self = Self(3);
    pub const NSURLNetworkServiceTypeVoice: Self = Self(4);
    pub const NSURLNetworkServiceTypeResponsiveData: Self = Self(6);
    pub const NSURLNetworkServiceTypeAVStreaming: Self = Self(8);
    pub const NSURLNetworkServiceTypeResponsiveAV: Self = Self(9);
    pub const NSURLNetworkServiceTypeCallSignaling: Self = Self(11);
}

unsafe impl Encode for NSURLRequestNetworkServiceType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSURLRequestNetworkServiceType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSURLRequestAttribution(pub NSUInteger);
impl NSURLRequestAttribution {
    #[doc(alias = "NSURLRequestAttributionDeveloper")]
    pub const Developer: Self = Self(0);
    #[doc(alias = "NSURLRequestAttributionUser")]
    pub const User: Self = Self(1);
}

unsafe impl Encode for NSURLRequestAttribution {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSURLRequestAttribution {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSURLRequest;

    unsafe impl ClassType for NSURLRequest {
        type Super = NSObject;
        type Mutability = ImmutableWithMutableSubclass<NSMutableURLRequest>;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSURLRequest {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSURLRequest {}

#[cfg(feature = "NSObject")]
unsafe impl NSMutableCopying for NSURLRequest {}

unsafe impl NSObjectProtocol for NSURLRequest {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSURLRequest {}

extern_methods!(
    unsafe impl NSURLRequest {
        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other requestWithURL:)]
        pub unsafe fn requestWithURL(url: &NSURL) -> Retained<Self>;

        #[method(supportsSecureCoding)]
        pub unsafe fn supportsSecureCoding() -> bool;

        #[cfg(all(feature = "NSDate", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other requestWithURL:cachePolicy:timeoutInterval:)]
        pub unsafe fn requestWithURL_cachePolicy_timeoutInterval(
            url: &NSURL,
            cache_policy: NSURLRequestCachePolicy,
            timeout_interval: NSTimeInterval,
        ) -> Retained<Self>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;

        #[cfg(all(feature = "NSDate", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:cachePolicy:timeoutInterval:)]
        pub unsafe fn initWithURL_cachePolicy_timeoutInterval(
            this: Allocated<Self>,
            url: &NSURL,
            cache_policy: NSURLRequestCachePolicy,
            timeout_interval: NSTimeInterval,
        ) -> Retained<Self>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[method(cachePolicy)]
        pub unsafe fn cachePolicy(&self) -> NSURLRequestCachePolicy;

        #[cfg(feature = "NSDate")]
        #[method(timeoutInterval)]
        pub unsafe fn timeoutInterval(&self) -> NSTimeInterval;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other mainDocumentURL)]
        pub unsafe fn mainDocumentURL(&self) -> Option<Retained<NSURL>>;

        #[method(networkServiceType)]
        pub unsafe fn networkServiceType(&self) -> NSURLRequestNetworkServiceType;

        #[method(allowsCellularAccess)]
        pub unsafe fn allowsCellularAccess(&self) -> bool;

        #[method(allowsExpensiveNetworkAccess)]
        pub unsafe fn allowsExpensiveNetworkAccess(&self) -> bool;

        #[method(allowsConstrainedNetworkAccess)]
        pub unsafe fn allowsConstrainedNetworkAccess(&self) -> bool;

        #[method(assumesHTTP3Capable)]
        pub unsafe fn assumesHTTP3Capable(&self) -> bool;

        #[method(attribution)]
        pub unsafe fn attribution(&self) -> NSURLRequestAttribution;

        #[method(requiresDNSSECValidation)]
        pub unsafe fn requiresDNSSECValidation(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSURLRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMutableURLRequest;

    unsafe impl ClassType for NSMutableURLRequest {
        #[inherits(NSObject)]
        type Super = NSURLRequest;
        type Mutability = MutableWithImmutableSuperclass<NSURLRequest>;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSMutableURLRequest {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSMutableURLRequest {}

#[cfg(feature = "NSObject")]
unsafe impl NSMutableCopying for NSMutableURLRequest {}

unsafe impl NSObjectProtocol for NSMutableURLRequest {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSMutableURLRequest {}

extern_methods!(
    unsafe impl NSMutableURLRequest {
        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSURL")]
        #[method(setURL:)]
        pub unsafe fn setURL(&mut self, url: Option<&NSURL>);

        #[method(cachePolicy)]
        pub unsafe fn cachePolicy(&self) -> NSURLRequestCachePolicy;

        #[method(setCachePolicy:)]
        pub unsafe fn setCachePolicy(&mut self, cache_policy: NSURLRequestCachePolicy);

        #[cfg(feature = "NSDate")]
        #[method(timeoutInterval)]
        pub unsafe fn timeoutInterval(&self) -> NSTimeInterval;

        #[cfg(feature = "NSDate")]
        #[method(setTimeoutInterval:)]
        pub unsafe fn setTimeoutInterval(&mut self, timeout_interval: NSTimeInterval);

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other mainDocumentURL)]
        pub unsafe fn mainDocumentURL(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSURL")]
        #[method(setMainDocumentURL:)]
        pub unsafe fn setMainDocumentURL(&mut self, main_document_url: Option<&NSURL>);

        #[method(networkServiceType)]
        pub unsafe fn networkServiceType(&self) -> NSURLRequestNetworkServiceType;

        #[method(setNetworkServiceType:)]
        pub unsafe fn setNetworkServiceType(
            &mut self,
            network_service_type: NSURLRequestNetworkServiceType,
        );

        #[method(allowsCellularAccess)]
        pub unsafe fn allowsCellularAccess(&self) -> bool;

        #[method(setAllowsCellularAccess:)]
        pub unsafe fn setAllowsCellularAccess(&mut self, allows_cellular_access: bool);

        #[method(allowsExpensiveNetworkAccess)]
        pub unsafe fn allowsExpensiveNetworkAccess(&self) -> bool;

        #[method(setAllowsExpensiveNetworkAccess:)]
        pub unsafe fn setAllowsExpensiveNetworkAccess(
            &mut self,
            allows_expensive_network_access: bool,
        );

        #[method(allowsConstrainedNetworkAccess)]
        pub unsafe fn allowsConstrainedNetworkAccess(&self) -> bool;

        #[method(setAllowsConstrainedNetworkAccess:)]
        pub unsafe fn setAllowsConstrainedNetworkAccess(
            &mut self,
            allows_constrained_network_access: bool,
        );

        #[method(assumesHTTP3Capable)]
        pub unsafe fn assumesHTTP3Capable(&self) -> bool;

        #[method(setAssumesHTTP3Capable:)]
        pub unsafe fn setAssumesHTTP3Capable(&mut self, assumes_http3_capable: bool);

        #[method(attribution)]
        pub unsafe fn attribution(&self) -> NSURLRequestAttribution;

        #[method(setAttribution:)]
        pub unsafe fn setAttribution(&mut self, attribution: NSURLRequestAttribution);

        #[method(requiresDNSSECValidation)]
        pub unsafe fn requiresDNSSECValidation(&self) -> bool;

        #[method(setRequiresDNSSECValidation:)]
        pub unsafe fn setRequiresDNSSECValidation(&mut self, requires_dnssec_validation: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSURLRequest`
    unsafe impl NSMutableURLRequest {
        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other requestWithURL:)]
        pub unsafe fn requestWithURL(url: &NSURL) -> Retained<Self>;

        #[cfg(all(feature = "NSDate", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other requestWithURL:cachePolicy:timeoutInterval:)]
        pub unsafe fn requestWithURL_cachePolicy_timeoutInterval(
            url: &NSURL,
            cache_policy: NSURLRequestCachePolicy,
            timeout_interval: NSTimeInterval,
        ) -> Retained<Self>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;

        #[cfg(all(feature = "NSDate", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:cachePolicy:timeoutInterval:)]
        pub unsafe fn initWithURL_cachePolicy_timeoutInterval(
            this: Allocated<Self>,
            url: &NSURL,
            cache_policy: NSURLRequestCachePolicy,
            timeout_interval: NSTimeInterval,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMutableURLRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSHTTPURLRequest
    unsafe impl NSURLRequest {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other HTTPMethod)]
        pub unsafe fn HTTPMethod(&self) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other allHTTPHeaderFields)]
        pub unsafe fn allHTTPHeaderFields(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSString>>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other valueForHTTPHeaderField:)]
        pub unsafe fn valueForHTTPHeaderField(
            &self,
            field: &NSString,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Other HTTPBody)]
        pub unsafe fn HTTPBody(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "NSStream")]
        #[method_id(@__retain_semantics Other HTTPBodyStream)]
        pub unsafe fn HTTPBodyStream(&self) -> Option<Retained<NSInputStream>>;

        #[method(HTTPShouldHandleCookies)]
        pub unsafe fn HTTPShouldHandleCookies(&self) -> bool;

        #[method(HTTPShouldUsePipelining)]
        pub unsafe fn HTTPShouldUsePipelining(&self) -> bool;
    }
);

extern_methods!(
    /// NSMutableHTTPURLRequest
    unsafe impl NSMutableURLRequest {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other HTTPMethod)]
        pub unsafe fn HTTPMethod(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method(setHTTPMethod:)]
        pub unsafe fn setHTTPMethod(&mut self, http_method: &NSString);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other allHTTPHeaderFields)]
        pub unsafe fn allHTTPHeaderFields(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSString>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method(setAllHTTPHeaderFields:)]
        pub unsafe fn setAllHTTPHeaderFields(
            &mut self,
            all_http_header_fields: Option<&NSDictionary<NSString, NSString>>,
        );

        #[cfg(feature = "NSString")]
        #[method(setValue:forHTTPHeaderField:)]
        pub unsafe fn setValue_forHTTPHeaderField(
            &mut self,
            value: Option<&NSString>,
            field: &NSString,
        );

        #[cfg(feature = "NSString")]
        #[method(addValue:forHTTPHeaderField:)]
        pub unsafe fn addValue_forHTTPHeaderField(&mut self, value: &NSString, field: &NSString);

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Other HTTPBody)]
        pub unsafe fn HTTPBody(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "NSData")]
        #[method(setHTTPBody:)]
        pub unsafe fn setHTTPBody(&mut self, http_body: Option<&NSData>);

        #[cfg(feature = "NSStream")]
        #[method_id(@__retain_semantics Other HTTPBodyStream)]
        pub unsafe fn HTTPBodyStream(&self) -> Option<Retained<NSInputStream>>;

        #[cfg(feature = "NSStream")]
        #[method(setHTTPBodyStream:)]
        pub unsafe fn setHTTPBodyStream(&mut self, http_body_stream: Option<&NSInputStream>);

        #[method(HTTPShouldHandleCookies)]
        pub unsafe fn HTTPShouldHandleCookies(&self) -> bool;

        #[method(setHTTPShouldHandleCookies:)]
        pub unsafe fn setHTTPShouldHandleCookies(&mut self, http_should_handle_cookies: bool);

        #[method(HTTPShouldUsePipelining)]
        pub unsafe fn HTTPShouldUsePipelining(&self) -> bool;

        #[method(setHTTPShouldUsePipelining:)]
        pub unsafe fn setHTTPShouldUsePipelining(&mut self, http_should_use_pipelining: bool);
    }
);
