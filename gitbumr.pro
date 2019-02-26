TEMPLATE = subdirs

SUBDIRS=lib \
        test \
        app

test.depends = lib app
app.depends = lib
