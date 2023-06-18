FROM rust:latest

WORKDIR /home/rust

RUN apt-get update
RUN apt-get install -y openjdk-11-jdk-headless

ARG ANDROID_SDK_FILENAME=commandlinetools-linux-7583922_latest.zip
ARG ANDROID_SDK_SHA=124f2d5115eee365df6cf3228ffbca6fc3911d16f8025bebd5b1c6e2fcfa7faf
ARG ANDROID_API_LEVELS=android-30
ARG ANDROID_BUILD_TOOLS_VERSION=30.0.2
ARG NDK_VERSION=25.2.9519653
ENV ANDROID_HOME /home/dev/opt/android-sdk
ENV PATH ${PATH}:${ANDROID_HOME}/tools:${ANDROID_HOME}/platform-tools:${ANDROID_HOME}/cmdline-tools/bin

RUN mkdir -p ${ANDROID_HOME}

RUN curl -O https://dl.google.com/android/repository/${ANDROID_SDK_FILENAME} \
    && echo "${ANDROID_SDK_SHA} ${ANDROID_SDK_FILENAME}" | sha256sum -c - \
    && unzip -q ${ANDROID_SDK_FILENAME} -d ${ANDROID_HOME} \
    && rm -rf ${ANDROID_SDK_FILENAME} \

RUN yes | sdkmanager --sdk_root=${ANDROID_HOME} --licenses
RUN yes | sdkmanager --no_https --verbose --sdk_root=${ANDROID_HOME} "platforms;${ANDROID_API_LEVELS}" "build-tools;${ANDROID_BUILD_TOOLS_VERSION}" "platform-tools" "ndk;${NDK_VERSION}"

RUN rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android

RUN cargo install cargo-ndk

RUN    apt-get install -y \
               clang \
               cmake \
               git \
               gpg-agent \
               libclang-dev \
               make

ENTRYPOINT ["/bin/bash", "-c"]