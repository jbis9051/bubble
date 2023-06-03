#import "RCTBubble.h"
#import <React/RCTLog.h>
#import "../rust.h"

@interface Callbacker : NSObject {
  RCTPromiseResolveBlock _resolve;
  RCTPromiseRejectBlock _reject;
}

- (instancetype)initWithResolve:(RCTPromiseResolveBlock)resolve
                         reject:(RCTPromiseRejectBlock)reject;
- (void)callback:(NSString *)result;

void C_callback(void *self, const char *result);
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
  callbacker->_resolve([NSString stringWithUTF8String:result]);
}

void promise_callbacker_reject(const void *self, const char *result) {
  Callbacker *callbacker = (Callbacker *)CFBridgingRelease(self);
  callbacker->_resolve([NSString stringWithUTF8String:result]);
}

@end

@implementation RCTBubble

// To export a module named RCTCalendarModule
RCT_EXPORT_MODULE();

RCT_REMAP_METHOD(rust_foo, multiplyWithA
                 : (RCTPromiseResolveBlock)resolve withRejecter
                 : (RCTPromiseRejectBlock)reject) {

  Callbacker *callbacker = [[Callbacker alloc] initWithResolve:resolve
                                                        reject:reject];
  rust_foo((void *)CFBridgingRetain(callbacker));
}

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