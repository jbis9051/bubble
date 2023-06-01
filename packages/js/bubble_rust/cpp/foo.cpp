#include "foo.h"
#include "../ios/CCallback.h"

namespace fooer {
void foo(void *callback) { C_callback(callback, "Hello from C++!"); }
} // namespace foo