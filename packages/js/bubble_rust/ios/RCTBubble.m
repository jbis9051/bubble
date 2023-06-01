#import "RCTBubble.h"
#import <React/RCTLog.h>

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

void C_callback(void *self, const char *result) {
  Callbacker *callbacker = (__bridge Callbacker *)self;
  [callbacker callback:[NSString stringWithUTF8String:result]];
}

@end

@implementation RCTBubble

// To export a module named RCTCalendarModule
RCT_EXPORT_MODULE();

RCT_REMAP_METHOD(multiply, multiplyWithA
                 : (double)a withB
                 : (double)b withResolver
                 : (RCTPromiseResolveBlock)resolve withRejecter
                 : (RCTPromiseRejectBlock)reject) {

  Callbacker *callbacker = [[Callbacker alloc] initWithResolve:resolve
                                                        reject:reject];
  fooer::foo((__bridge void *)(callbacker));
}

@end