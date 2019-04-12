function gitbumr_build_env() {
	if [ -z "$APPVEYOR_BUILD_VERSION" ]; then APPVEYOR_BUILD_VERSION=devel; fi
	if [ -z "$APPVEYOR_BUILD_FOLDER" ]; then APPVEYOR_BUILD_FOLDER=$HOME/gitbumr; fi
	if [ -z "$QT_DEPLOY_BIN" ]
	then
		export QTDBIN=$HOME/linuxdeployqt-6-x86_64.AppImage # assume local version in home folder
	else
		export QTDBIN=$APPVEYOR_BUILD_FOLDER/$QT_DEPLOY_BIN
	fi
	export SRCBASEDIR=$APPVEYOR_BUILD_FOLDER
	export VERSION=$APPVEYOR_BUILD_VERSION # linuxdeployqt uses this for naming the file
	source $HOME/.cargo/env
	export PATH="$HOME/Qt/5.12.2/gcc_64/bin:$PATH"
	export TST_GIT_PATH=$SRCBASEDIR
	export QML2_IMPORT_PATH=$SRCBASEDIR/build/lib/release
}

function gitbumr_build_step() {
	if [ -d "$SRCBASEDIR/build" ]; then rm -rf $SRCBASEDIR/build; fi
	mkdir build
	pushd build
	qmake -config release ..
	make
	popd
}

function gitbumr_pack_step() {
	pushd build
	mkdir -p AppImage/usr/{bin,lib}
	mkdir -p AppImage/usr/share/applications
	mkdir -p AppImage/usr/share/icons/hicolor/scalable/apps
	cp ../app/res/AppImage/gitbumr.desktop AppImage/usr/share/applications
	cp ../app/res/AppImage/gitbumr.svg AppImage/usr/share/icons/hicolor/scalable/apps
	cp app/bin/gitbumr AppImage/usr/bin/gitbumr
	cp -r lib/release/GitbumrComponents AppImage/usr/bin
	$QTDBIN AppImage/usr/share/applications/gitbumr.desktop -appimage -verbose=0 -qmldir=../app
	mv Gitbumr-$VERSION-x86_64.AppImage ..
	popd
}
