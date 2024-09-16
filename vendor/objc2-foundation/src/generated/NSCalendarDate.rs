//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSDate")]
    #[deprecated = "Use NSCalendar and NSDateComponents and NSDateFormatter instead"]
    pub struct NSCalendarDate;

    #[cfg(feature = "NSDate")]
    unsafe impl ClassType for NSCalendarDate {
        #[inherits(NSObject)]
        type Super = NSDate;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "NSDate", feature = "NSObject"))]
unsafe impl NSCoding for NSCalendarDate {}

#[cfg(all(feature = "NSDate", feature = "NSObject"))]
unsafe impl NSCopying for NSCalendarDate {}

#[cfg(feature = "NSDate")]
unsafe impl NSObjectProtocol for NSCalendarDate {}

#[cfg(all(feature = "NSDate", feature = "NSObject"))]
unsafe impl NSSecureCoding for NSCalendarDate {}

extern_methods!(
    #[cfg(feature = "NSDate")]
    unsafe impl NSCalendarDate {
        #[deprecated = "Use NSCalendar instead"]
        #[method_id(@__retain_semantics Other calendarDate)]
        pub unsafe fn calendarDate() -> Retained<AnyObject>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Other dateWithString:calendarFormat:locale:)]
        pub unsafe fn dateWithString_calendarFormat_locale(
            description: &NSString,
            format: &NSString,
            locale: Option<&AnyObject>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Other dateWithString:calendarFormat:)]
        pub unsafe fn dateWithString_calendarFormat(
            description: &NSString,
            format: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSTimeZone")]
        #[deprecated = "Use NSCalendar and NSDateComponents instead"]
        #[method_id(@__retain_semantics Other dateWithYear:month:day:hour:minute:second:timeZone:)]
        pub unsafe fn dateWithYear_month_day_hour_minute_second_timeZone(
            year: NSInteger,
            month: NSUInteger,
            day: NSUInteger,
            hour: NSUInteger,
            minute: NSUInteger,
            second: NSUInteger,
            a_time_zone: Option<&NSTimeZone>,
        ) -> Retained<AnyObject>;

        #[deprecated = "Use NSCalendar instead"]
        #[method_id(@__retain_semantics Other dateByAddingYears:months:days:hours:minutes:seconds:)]
        pub unsafe fn dateByAddingYears_months_days_hours_minutes_seconds(
            &self,
            year: NSInteger,
            month: NSInteger,
            day: NSInteger,
            hour: NSInteger,
            minute: NSInteger,
            second: NSInteger,
        ) -> Retained<NSCalendarDate>;

        #[deprecated]
        #[method(dayOfCommonEra)]
        pub unsafe fn dayOfCommonEra(&self) -> NSInteger;

        #[deprecated]
        #[method(dayOfMonth)]
        pub unsafe fn dayOfMonth(&self) -> NSInteger;

        #[deprecated]
        #[method(dayOfWeek)]
        pub unsafe fn dayOfWeek(&self) -> NSInteger;

        #[deprecated]
        #[method(dayOfYear)]
        pub unsafe fn dayOfYear(&self) -> NSInteger;

        #[deprecated]
        #[method(hourOfDay)]
        pub unsafe fn hourOfDay(&self) -> NSInteger;

        #[deprecated]
        #[method(minuteOfHour)]
        pub unsafe fn minuteOfHour(&self) -> NSInteger;

        #[deprecated]
        #[method(monthOfYear)]
        pub unsafe fn monthOfYear(&self) -> NSInteger;

        #[deprecated]
        #[method(secondOfMinute)]
        pub unsafe fn secondOfMinute(&self) -> NSInteger;

        #[deprecated]
        #[method(yearOfCommonEra)]
        pub unsafe fn yearOfCommonEra(&self) -> NSInteger;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other calendarFormat)]
        pub unsafe fn calendarFormat(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptionWithCalendarFormat:locale:)]
        pub unsafe fn descriptionWithCalendarFormat_locale(
            &self,
            format: &NSString,
            locale: Option<&AnyObject>,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptionWithCalendarFormat:)]
        pub unsafe fn descriptionWithCalendarFormat(&self, format: &NSString)
            -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(
            &self,
            locale: Option<&AnyObject>,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSTimeZone")]
        #[deprecated]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Retained<NSTimeZone>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Init initWithString:calendarFormat:locale:)]
        pub unsafe fn initWithString_calendarFormat_locale(
            this: Allocated<Self>,
            description: &NSString,
            format: &NSString,
            locale: Option<&AnyObject>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Init initWithString:calendarFormat:)]
        pub unsafe fn initWithString_calendarFormat(
            this: Allocated<Self>,
            description: &NSString,
            format: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Allocated<Self>,
            description: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSTimeZone")]
        #[deprecated = "Use NSCalendar and NSDateComponents instead"]
        #[method_id(@__retain_semantics Init initWithYear:month:day:hour:minute:second:timeZone:)]
        pub unsafe fn initWithYear_month_day_hour_minute_second_timeZone(
            this: Allocated<Self>,
            year: NSInteger,
            month: NSUInteger,
            day: NSUInteger,
            hour: NSUInteger,
            minute: NSUInteger,
            second: NSUInteger,
            a_time_zone: Option<&NSTimeZone>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method(setCalendarFormat:)]
        pub unsafe fn setCalendarFormat(&self, format: Option<&NSString>);

        #[cfg(feature = "NSTimeZone")]
        #[deprecated]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, a_time_zone: Option<&NSTimeZone>);

        #[deprecated]
        #[method(years:months:days:hours:minutes:seconds:sinceDate:)]
        pub unsafe fn years_months_days_hours_minutes_seconds_sinceDate(
            &self,
            yp: *mut NSInteger,
            mop: *mut NSInteger,
            dp: *mut NSInteger,
            hp: *mut NSInteger,
            mip: *mut NSInteger,
            sp: *mut NSInteger,
            date: &NSCalendarDate,
        );

        #[deprecated]
        #[method_id(@__retain_semantics Other distantFuture)]
        pub unsafe fn distantFuture() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other distantPast)]
        pub unsafe fn distantPast() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDate`
    #[cfg(feature = "NSDate")]
    unsafe impl NSCalendarDate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithTimeIntervalSinceReferenceDate:)]
        pub unsafe fn initWithTimeIntervalSinceReferenceDate(
            this: Allocated<Self>,
            ti: NSTimeInterval,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSDate")]
    unsafe impl NSCalendarDate {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSCalendarDateExtras
    #[cfg(feature = "NSDate")]
    unsafe impl NSDate {
        #[cfg(feature = "NSString")]
        #[deprecated = "Create an NSDateFormatter with `init` and set the dateFormat property instead."]
        #[method_id(@__retain_semantics Other dateWithNaturalLanguageString:locale:)]
        pub unsafe fn dateWithNaturalLanguageString_locale(
            string: &NSString,
            locale: Option<&AnyObject>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Create an NSDateFormatter with `init` and set the dateFormat property instead."]
        #[method_id(@__retain_semantics Other dateWithNaturalLanguageString:)]
        pub unsafe fn dateWithNaturalLanguageString(
            string: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Other dateWithString:)]
        pub unsafe fn dateWithString(a_string: &NSString) -> Retained<AnyObject>;

        #[cfg(all(feature = "NSString", feature = "NSTimeZone"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other dateWithCalendarFormat:timeZone:)]
        pub unsafe fn dateWithCalendarFormat_timeZone(
            &self,
            format: Option<&NSString>,
            a_time_zone: Option<&NSTimeZone>,
        ) -> Retained<NSCalendarDate>;

        #[cfg(all(feature = "NSString", feature = "NSTimeZone"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other descriptionWithCalendarFormat:timeZone:locale:)]
        pub unsafe fn descriptionWithCalendarFormat_timeZone_locale(
            &self,
            format: Option<&NSString>,
            a_time_zone: Option<&NSTimeZone>,
            locale: Option<&AnyObject>,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSDateFormatter instead"]
        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Allocated<Self>,
            description: &NSString,
        ) -> Option<Retained<Self>>;
    }
);
