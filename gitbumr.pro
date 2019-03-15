TEMPLATE = subdirs

SUBDIRS=lib \
        test \
        app

test.depends = lib
app.depends = lib
