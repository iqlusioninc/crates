#!/bin/bash
#
# Build script for https://rpmlib.rs
#
# You will need to have a firebase login and configure the RPMLIB_DOC_BUILD_HOST
# environment variable in order to use this.

UUID=$(uuidgen)
BUILD_PATH=/tmp/rpmlib-docs/$UUID

if [[ -z "$RPMLIB_DOC_BUILD_HOST" ]]; then
    echo "\$RPMLIB_DOC_BUILD_HOST is unset!"
    exit 1
fi

# Build the docs
ssh $RPMLIB_DOC_BUILD_HOST <<REMOTE_CMD
    mkdir -p $BUILD_PATH &&
    cd $BUILD_PATH &&
    git clone https://github.com/iqlusion-io/crates &&
    cd $BUILD_PATH/crates/rpmlib &&
    cargo doc
REMOTE_CMD

# Extract them locally
ssh $RPMLIB_DOC_BUILD_HOST "bash -c \"cd $BUILD_PATH/crates/target/doc && /usr/bin/tar -Jc rpmlib rpmlib_sys src/rpmlib src/rpmlib_sys *.woff *.css *.js *.svg\"" | tar -C public -Jxv

# (Re-)create index.html
echo "<meta http-equiv=\"refresh\" content=\"0; url=/rpmlib\">" > public/index.html

# Make rpmlib-sys links work
rm -rf public/rpmlib-sys
cp -r public/rpmlib_sys/ public/rpmlib-sys

# Deploy to Firebase
firebase deploy
