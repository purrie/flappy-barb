export VERSION := `cargo metadata --no-deps -q --format-version=1 | grep -Eo '"version":"[0-9]+\.[0-9]+\.[0-9]+"' | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+'`
export NAME := `cargo metadata --no-deps -q --format-version=1 | grep -Eo '"name":".+","version":"' | sed -E 's/","version":"//' | sed -E 's/"name":"//'`

[linux]
play:
    cargo run --release --target x86_64-unknown-linux-gnu

[linux]
install: mk-desktop
    cargo build --release --target x86_64-unknown-linux-gnu --features local_unix_assets
    cp ./target/x86_64-unknown-linux-gnu/release/$NAME ~/.local/bin/
    mv ./target/pack/$NAME.desktop ~/.local/share/applications/
    mkdir -p ~/.local/share/$NAME
    cp -rfu ./assets/* ~/.local/share/$NAME/
    @echo Installation complete

[linux]
remove:
    rm ~/.local/bin/$NAME
    rm ~/.local/share/applications/$NAME.desktop
    rm -r ~/.local/share/$NAME/
    @echo Removal complete.

make-wasm:
    cargo build --release --target wasm32-unknown-unknown
    if [ -d target/pack/$NAME-wasm ]; then rm -rf target/pack/$NAME-wasm; fi
    mkdir -p target/pack/$NAME-wasm/
    wasm-bindgen --out-dir ./target/pack/$NAME-wasm/ --target web ./target/wasm32-unknown-unknown/release/$NAME.wasm
    cp -r ./assets ./target/pack/$NAME-wasm/
    cp -r ./wasm/* ./target/pack/$NAME-wasm/

pack-all: pack-zip pack-tar pack-deb pack-wasm
    @echo Packing everything completed

pack-wasm: make-wasm
    #!/usr/bin/env bash
    cd target/pack
    if [ -f $NAME-wasm.zip ]; then rm $NAME-wasm.zip; fi
    zip -r $NAME-wasm $NAME-wasm
    rm -r $NAME-wasm/
    echo Packing WASM zip Complete

pack-zip:
    #!/usr/bin/env bash
    cargo rustc --release --target x86_64-pc-windows-gnu -- -Clink-args="-Wl,--subsystem,windows"

    mkdir -p ./target/pack/$NAME
    cp ./target/x86_64-pc-windows-gnu/release/$NAME.exe ./target/pack/$NAME/$NAME.exe
    cp -r ./assets ./target/pack/$NAME/
    cp ./COPYRIGHT ./target/pack/$NAME/
    cd target/pack/
    if [ -f $NAME.zip ]; then rm $NAME.zip; fi
    zip -r $NAME $NAME
    rm -r $NAME/
    echo Packing Zip Complete

mk-desktop:
    #!/usr/bin/env bash
    mkdir -p ./target/pack
    cd ./target/pack
    echo "[Desktop Entry]" > $NAME.desktop
    echo "Type=Application" >> $NAME.desktop
    echo "Name=$NAME" >> $NAME.desktop
    echo "Comment=A side scrolling game" >> $NAME.desktop
    echo "Exec=$NAME" >> $NAME.desktop
    echo "Terminal=false" >> $NAME.desktop
    echo "Categories=Game" >> $NAME.desktop

pack-tar:
    #!/usr/bin/env bash
    just mk-desktop
    cargo build --release --target x86_64-unknown-linux-gnu --features local_unix_assets

    mkdir -p ./target/pack/$NAME
    cp ./target/x86_64-unknown-linux-gnu/release/$NAME ./target/pack/$NAME/
    cp -r ./assets ./target/pack/$NAME/
    mv ./target/pack/$NAME.desktop ./target/pack/$NAME/
    cp ./COPYRIGHT ./target/pack/$NAME/
    cd target/pack/$NAME
    INSTALL='#!/usr/bin/env bash
             \necho Installing to $HOME/.local/bin
             \ncp ./PROGRAM ~/.local/bin/
             \ncp ./PROGRAM.desktop ~/.local/share/applications/
             \nmkdir -p ~/.local/share/PROGRAM
             \ncp -rfu ./assets/* ~/.local/share/PROGRAM/
             \necho Installation completed'
    REMOVE='#!/usr/bin/env bash
            \nAB=$(which PROGRAM)
            \nif [ $? -eq 0 ]; then
            \n    echo Removed executable
            \n    rm $AB
            \nfi
            \nif [ -f $HOME/.local/share/applications/PROGRAM.desktop ]; then
            \n    echo Removed desktop entry
            \n    rm $HOME/.local/share/applications/PROGRAM.desktop
            \nfi
            \nif [ -d $HOME/.local/share/PROGRAM ]; then
            \n    echo Removed assets
            \n    rm -rf $HOME/.local/share/PROGRAM
            \nfi'
    echo -e $INSTALL | sed -E "s/PROGRAM/$NAME/g" > install.sh
    echo -e $REMOVE | sed -E "s/PROGRAM/$NAME/g" > remove.sh
    chmod 755 $NAME
    chmod 755 install.sh
    chmod 755 remove.sh
    cd ..
    tar -caf ./$NAME.tar.gz ./$NAME
    rm -r $NAME/
    echo Packing Tar Complete

pack-deb:
    #!/usr/bin/env bash
    cargo build --release --target x86_64-unknown-linux-gnu --features unix_assets

    VERSION_MAJOR=$(echo $VERSION | sed 's/.[0-9]$//')
    VERSION_MINOR=$(echo $VERSION | sed 's/[0-9].[0-9].//')
    DEB_VERSION=${VERSION_MAJOR}-${VERSION_MINOR}
    TARGET_FOLDER=${NAME}_${DEB_VERSION}

    just mk-desktop
    mkdir -p ./target/pack/$TARGET_FOLDER/usr/bin
    mkdir -p ./target/pack/$TARGET_FOLDER/usr/share/applications
    mkdir -p ./target/pack/$TARGET_FOLDER/usr/share/$NAME

    cp ./target/x86_64-unknown-linux-gnu/release/$NAME ./target/pack/$TARGET_FOLDER/usr/bin/
    mv ./target/pack/$NAME.desktop ./target/pack/$TARGET_FOLDER/usr/share/applications/
    cp -r ./assets/* ./target/pack/$TARGET_FOLDER/usr/share/$NAME/
    cd ./target/pack/$TARGET_FOLDER
    mkdir DEBIAN
    cd DEBIAN
    echo Package: $NAME > control
    echo Version: $DEB_VERSION >> control
    echo Section: games >> control
    echo Priority: optional >> control
    echo Architecture: amd64 >> control
    echo "Maintainer: Purrie Brightstar <purriestarshine@gmail.com>" >> control
    echo "Homepage: https://github.com/purrie/$NAME" >> control
    echo Description: A side scrolling game >> control

    cp ../../../../COPYRIGHT ./copyright
    if [ -f ../../changelog ]; then
        cp ../../changelog ./changelog
    else
        echo No changelog found in root/target/pack folder, skipping inclusion
    fi

    cd ../..
    dpkg-deb --build $TARGET_FOLDER
    rm -r $TARGET_FOLDER
    echo Packing Deb Complete

clear:
    rm -rf ./target
