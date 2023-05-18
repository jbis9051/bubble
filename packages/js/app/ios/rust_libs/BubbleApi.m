#import <Foundation/Foundation.h>
#import "React/RCTBridgeModule.h"
#include "my_rust_module.h"

@interface MyRustModule : NSObject <RCTBridgeModule>

@end

@implementation MyRustModule

RCT_EXPORT_MODULE();

RCT_EXPORT_METHOD(addNumbers:(int)a b:(int)b
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject) {
    int result = add_numbers(a, b);
    resolve(@(result));
}

@end