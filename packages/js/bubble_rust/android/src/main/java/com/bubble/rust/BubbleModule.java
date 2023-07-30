package com.bubble.rust;

import androidx.annotation.NonNull;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.module.annotations.ReactModule;
import com.facebook.react.bridge.Promise;

@ReactModule(name = BubbleModule.NAME)
public class BubbleModule extends ReactContextBaseJavaModule {
  public static final String NAME = "Bubble";

  public BubbleModule(ReactApplicationContext reactContext) {
    super(reactContext);
  }

  @Override
  @NonNull
  public String getName() {
    return NAME;
  }

  static {
    System.loadLibrary("frontend");
  }

  private static native void nativeInit(String dataDir, Promise promise);
  private static native void nativeCall(String json, Promise promise);

  @ReactMethod
  public void init(String dataDir, Promise promise) {
    nativeInit(dataDir, promise);
  }

  @ReactMethod
  public void call(String json, Promise promise) {
    nativeCall(json, promise);
  }

  @ReactMethod
  public void getAppDir(Promise promise) {
    promise.resolve(getReactApplicationContext().getFilesDir().getAbsolutePath());
  }

}
