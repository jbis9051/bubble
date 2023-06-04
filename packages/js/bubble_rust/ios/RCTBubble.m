#import "RCTBubble.h"
#import "../rust.h"
#import <React/RCTLog.h>
#import <stdlib.h>

@interface Callbacker : NSObject {
  RCTPromiseResolveBlock _resolve;
  RCTPromiseRejectBlock _reject;
}

- (instancetype)initWithResolve:(RCTPromiseResolveBlock)resolve
                         reject:(RCTPromiseRejectBlock)reject;
- (void)callback:(NSString *)result;
@end

@implementation Callbacker

- (instancetype)initWithResolve:(RCTPromiseResolveBlock)resolve
                         reject:(RCTPromiseRejectBlock)reject {
  self = [super init];
  if (self) {
    _resolve = resolve;
    _reject = reject;
  }
  return self;
}

- (void)callback:(id)result {
  _resolve(result);
}

void promise_callbacker_resolve(const void *self, const char *result) {
  Callbacker *callbacker = (Callbacker *)CFBridgingRelease(self);
  NSString *data = [NSString stringWithUTF8String:result];
  free((void *)result);
  callbacker->_resolve(data);
}

void promise_callbacker_reject(const void *self, const char *result) {
  Callbacker *callbacker = (Callbacker *)CFBridgingRelease(self);
  NSString *error = [NSString stringWithUTF8String:result];
  free((void *)result);
  callbacker->_resolve(error);
}

@end

@implementation RCTBubble

// To export a module named RCTCalendarModule
RCT_EXPORT_MODULE();

RCT_REMAP_METHOD(init, initWithDataDir
                 : (NSString *)dataDir withResolver
                 : (RCTPromiseResolveBlock)resolve withRejecter
                 : (RCTPromiseRejectBlock)reject) {

  Callbacker *callbacker = [[Callbacker alloc] initWithResolve:resolve
                                                        reject:reject];
  init((void *)CFBridgingRetain(callbacker), [dataDir UTF8String]);
}

RCT_REMAP_METHOD(call, callWithJson
                 : (NSString *)json withResolver
                 : (RCTPromiseResolveBlock)resolve withRejecter
                 : (RCTPromiseRejectBlock)reject) {

  Callbacker *callbacker = [[Callbacker alloc] initWithResolve:resolve
                                                        reject:reject];
  call((void *)CFBridgingRetain(callbacker), [json UTF8String]);
}

@end