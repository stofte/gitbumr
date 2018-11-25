/* generated by rust_qt_binding_generator */
#include "Bindings.h"

namespace {

    struct option_quintptr {
    public:
        quintptr value;
        bool some;
        operator QVariant() const {
            if (some) {
                return QVariant::fromValue(value);
            }
            return QVariant();
        }
    };
    static_assert(std::is_pod<option_quintptr>::value, "option_quintptr must be a POD type.");

    typedef void (*qstring_set)(QString* val, const char* utf8, int nbytes);
    void set_qstring(QString* val, const char* utf8, int nbytes) {
        *val = QString::fromUtf8(utf8, nbytes);
    }

    struct qmodelindex_t {
        int row;
        quintptr id;
    };
    inline QVariant cleanNullQVariant(const QVariant& v) {
        return (v.isNull()) ?QVariant() :v;
    }
    inline void repositoriesActiveRepositoryChanged(Repositories* o)
    {
        Q_EMIT o->activeRepositoryChanged();
    }
}
extern "C" {
    bool branches_data_checkedout(const Branches::Private*, int);
    void branches_data_name(const Branches::Private*, int, QString*, qstring_set);
    void branches_sort(Branches::Private*, unsigned char column, Qt::SortOrder order = Qt::AscendingOrder);

    int branches_row_count(const Branches::Private*);
    bool branches_insert_rows(Branches::Private*, int, int);
    bool branches_remove_rows(Branches::Private*, int, int);
    bool branches_can_fetch_more(const Branches::Private*);
    void branches_fetch_more(Branches::Private*);
}
int Branches::columnCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : 1;
}

bool Branches::hasChildren(const QModelIndex &parent) const
{
    return rowCount(parent) > 0;
}

int Branches::rowCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : branches_row_count(m_d);
}

bool Branches::insertRows(int row, int count, const QModelIndex &)
{
    return branches_insert_rows(m_d, row, count);
}

bool Branches::removeRows(int row, int count, const QModelIndex &)
{
    return branches_remove_rows(m_d, row, count);
}

QModelIndex Branches::index(int row, int column, const QModelIndex &parent) const
{
    if (!parent.isValid() && row >= 0 && row < rowCount(parent) && column >= 0 && column < 1) {
        return createIndex(row, column, (quintptr)row);
    }
    return QModelIndex();
}

QModelIndex Branches::parent(const QModelIndex &) const
{
    return QModelIndex();
}

bool Branches::canFetchMore(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : branches_can_fetch_more(m_d);
}

void Branches::fetchMore(const QModelIndex &parent)
{
    if (!parent.isValid()) {
        branches_fetch_more(m_d);
    }
}
void Branches::updatePersistentIndexes() {}

void Branches::sort(int column, Qt::SortOrder order)
{
    branches_sort(m_d, column, order);
}
Qt::ItemFlags Branches::flags(const QModelIndex &i) const
{
    auto flags = QAbstractItemModel::flags(i);
    return flags;
}

bool Branches::checkedout(int row) const
{
    return branches_data_checkedout(m_d, row);
}

QString Branches::name(int row) const
{
    QString s;
    branches_data_name(m_d, row, &s, set_qstring);
    return s;
}

QVariant Branches::data(const QModelIndex &index, int role) const
{
    Q_ASSERT(rowCount(index.parent()) > index.row());
    switch (index.column()) {
    case 0:
        switch (role) {
        case Qt::UserRole + 0:
            return QVariant::fromValue(checkedout(index.row()));
        case Qt::UserRole + 1:
            return QVariant::fromValue(name(index.row()));
        }
        break;
    }
    return QVariant();
}

int Branches::role(const char* name) const {
    auto names = roleNames();
    auto i = names.constBegin();
    while (i != names.constEnd()) {
        if (i.value() == name) {
            return i.key();
        }
        ++i;
    }
    return -1;
}
QHash<int, QByteArray> Branches::roleNames() const {
    QHash<int, QByteArray> names = QAbstractItemModel::roleNames();
    names.insert(Qt::UserRole + 0, "checkedout");
    names.insert(Qt::UserRole + 1, "name");
    return names;
}
QVariant Branches::headerData(int section, Qt::Orientation orientation, int role) const
{
    if (orientation != Qt::Horizontal) {
        return QVariant();
    }
    return m_headerData.value(qMakePair(section, (Qt::ItemDataRole)role), role == Qt::DisplayRole ?QString::number(section + 1) :QVariant());
}

bool Branches::setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role)
{
    if (orientation != Qt::Horizontal) {
        return false;
    }
    m_headerData.insert(qMakePair(section, (Qt::ItemDataRole)role), value);
    return true;
}

extern "C" {
    Branches::Private* branches_new(Branches*,
        void (*)(const Branches*),
        void (*)(Branches*),
        void (*)(Branches*),
        void (*)(Branches*, quintptr, quintptr),
        void (*)(Branches*),
        void (*)(Branches*),
        void (*)(Branches*, int, int),
        void (*)(Branches*),
        void (*)(Branches*, int, int, int),
        void (*)(Branches*),
        void (*)(Branches*, int, int),
        void (*)(Branches*));
    void branches_free(Branches::Private*);
};

extern "C" {
    Git::Private* git_new(Git*, Branches*,
        void (*)(const Branches*),
        void (*)(Branches*),
        void (*)(Branches*),
        void (*)(Branches*, quintptr, quintptr),
        void (*)(Branches*),
        void (*)(Branches*),
        void (*)(Branches*, int, int),
        void (*)(Branches*),
        void (*)(Branches*, int, int, int),
        void (*)(Branches*),
        void (*)(Branches*, int, int),
        void (*)(Branches*));
    void git_free(Git::Private*);
    Branches::Private* git_branches_get(const Git::Private*);
};

extern "C" {
    void log_data_author(const Log::Private*, int, QString*, qstring_set);
    void log_data_message(const Log::Private*, int, QString*, qstring_set);
    void log_data_oid(const Log::Private*, int, QString*, qstring_set);
    void log_data_time(const Log::Private*, int, QString*, qstring_set);
    void log_sort(Log::Private*, unsigned char column, Qt::SortOrder order = Qt::AscendingOrder);

    int log_row_count(const Log::Private*);
    bool log_insert_rows(Log::Private*, int, int);
    bool log_remove_rows(Log::Private*, int, int);
    bool log_can_fetch_more(const Log::Private*);
    void log_fetch_more(Log::Private*);
}
int Log::columnCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : 1;
}

bool Log::hasChildren(const QModelIndex &parent) const
{
    return rowCount(parent) > 0;
}

int Log::rowCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : log_row_count(m_d);
}

bool Log::insertRows(int row, int count, const QModelIndex &)
{
    return log_insert_rows(m_d, row, count);
}

bool Log::removeRows(int row, int count, const QModelIndex &)
{
    return log_remove_rows(m_d, row, count);
}

QModelIndex Log::index(int row, int column, const QModelIndex &parent) const
{
    if (!parent.isValid() && row >= 0 && row < rowCount(parent) && column >= 0 && column < 1) {
        return createIndex(row, column, (quintptr)row);
    }
    return QModelIndex();
}

QModelIndex Log::parent(const QModelIndex &) const
{
    return QModelIndex();
}

bool Log::canFetchMore(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : log_can_fetch_more(m_d);
}

void Log::fetchMore(const QModelIndex &parent)
{
    if (!parent.isValid()) {
        log_fetch_more(m_d);
    }
}
void Log::updatePersistentIndexes() {}

void Log::sort(int column, Qt::SortOrder order)
{
    log_sort(m_d, column, order);
}
Qt::ItemFlags Log::flags(const QModelIndex &i) const
{
    auto flags = QAbstractItemModel::flags(i);
    return flags;
}

QString Log::author(int row) const
{
    QString s;
    log_data_author(m_d, row, &s, set_qstring);
    return s;
}

QString Log::message(int row) const
{
    QString s;
    log_data_message(m_d, row, &s, set_qstring);
    return s;
}

QString Log::oid(int row) const
{
    QString s;
    log_data_oid(m_d, row, &s, set_qstring);
    return s;
}

QString Log::time(int row) const
{
    QString s;
    log_data_time(m_d, row, &s, set_qstring);
    return s;
}

QVariant Log::data(const QModelIndex &index, int role) const
{
    Q_ASSERT(rowCount(index.parent()) > index.row());
    switch (index.column()) {
    case 0:
        switch (role) {
        case Qt::UserRole + 0:
            return QVariant::fromValue(author(index.row()));
        case Qt::UserRole + 1:
            return QVariant::fromValue(message(index.row()));
        case Qt::UserRole + 2:
            return QVariant::fromValue(oid(index.row()));
        case Qt::UserRole + 3:
            return QVariant::fromValue(time(index.row()));
        }
        break;
    }
    return QVariant();
}

int Log::role(const char* name) const {
    auto names = roleNames();
    auto i = names.constBegin();
    while (i != names.constEnd()) {
        if (i.value() == name) {
            return i.key();
        }
        ++i;
    }
    return -1;
}
QHash<int, QByteArray> Log::roleNames() const {
    QHash<int, QByteArray> names = QAbstractItemModel::roleNames();
    names.insert(Qt::UserRole + 0, "author");
    names.insert(Qt::UserRole + 1, "message");
    names.insert(Qt::UserRole + 2, "oid");
    names.insert(Qt::UserRole + 3, "time");
    return names;
}
QVariant Log::headerData(int section, Qt::Orientation orientation, int role) const
{
    if (orientation != Qt::Horizontal) {
        return QVariant();
    }
    return m_headerData.value(qMakePair(section, (Qt::ItemDataRole)role), role == Qt::DisplayRole ?QString::number(section + 1) :QVariant());
}

bool Log::setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role)
{
    if (orientation != Qt::Horizontal) {
        return false;
    }
    m_headerData.insert(qMakePair(section, (Qt::ItemDataRole)role), value);
    return true;
}

extern "C" {
    Log::Private* log_new(Log*,
        void (*)(const Log*),
        void (*)(Log*),
        void (*)(Log*),
        void (*)(Log*, quintptr, quintptr),
        void (*)(Log*),
        void (*)(Log*),
        void (*)(Log*, int, int),
        void (*)(Log*),
        void (*)(Log*, int, int, int),
        void (*)(Log*),
        void (*)(Log*, int, int),
        void (*)(Log*));
    void log_free(Log::Private*);
    void log_filter(Log::Private*, const ushort*, int);
    void log_load(Log::Private*, const ushort*, int);
};

extern "C" {
    bool repositories_data_current(const Repositories::Private*, int);
    void repositories_data_display_name(const Repositories::Private*, int, QString*, qstring_set);
    qint64 repositories_data_id(const Repositories::Private*, int);
    void repositories_sort(Repositories::Private*, unsigned char column, Qt::SortOrder order = Qt::AscendingOrder);

    int repositories_row_count(const Repositories::Private*);
    bool repositories_insert_rows(Repositories::Private*, int, int);
    bool repositories_remove_rows(Repositories::Private*, int, int);
    bool repositories_can_fetch_more(const Repositories::Private*);
    void repositories_fetch_more(Repositories::Private*);
}
int Repositories::columnCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : 1;
}

bool Repositories::hasChildren(const QModelIndex &parent) const
{
    return rowCount(parent) > 0;
}

int Repositories::rowCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : repositories_row_count(m_d);
}

bool Repositories::insertRows(int row, int count, const QModelIndex &)
{
    return repositories_insert_rows(m_d, row, count);
}

bool Repositories::removeRows(int row, int count, const QModelIndex &)
{
    return repositories_remove_rows(m_d, row, count);
}

QModelIndex Repositories::index(int row, int column, const QModelIndex &parent) const
{
    if (!parent.isValid() && row >= 0 && row < rowCount(parent) && column >= 0 && column < 1) {
        return createIndex(row, column, (quintptr)row);
    }
    return QModelIndex();
}

QModelIndex Repositories::parent(const QModelIndex &) const
{
    return QModelIndex();
}

bool Repositories::canFetchMore(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : repositories_can_fetch_more(m_d);
}

void Repositories::fetchMore(const QModelIndex &parent)
{
    if (!parent.isValid()) {
        repositories_fetch_more(m_d);
    }
}
void Repositories::updatePersistentIndexes() {}

void Repositories::sort(int column, Qt::SortOrder order)
{
    repositories_sort(m_d, column, order);
}
Qt::ItemFlags Repositories::flags(const QModelIndex &i) const
{
    auto flags = QAbstractItemModel::flags(i);
    return flags;
}

bool Repositories::current(int row) const
{
    return repositories_data_current(m_d, row);
}

QString Repositories::displayName(int row) const
{
    QString s;
    repositories_data_display_name(m_d, row, &s, set_qstring);
    return s;
}

qint64 Repositories::id(int row) const
{
    return repositories_data_id(m_d, row);
}

QVariant Repositories::data(const QModelIndex &index, int role) const
{
    Q_ASSERT(rowCount(index.parent()) > index.row());
    switch (index.column()) {
    case 0:
        switch (role) {
        case Qt::UserRole + 0:
            return QVariant::fromValue(current(index.row()));
        case Qt::UserRole + 1:
            return QVariant::fromValue(displayName(index.row()));
        case Qt::UserRole + 2:
            return QVariant::fromValue(id(index.row()));
        }
        break;
    }
    return QVariant();
}

int Repositories::role(const char* name) const {
    auto names = roleNames();
    auto i = names.constBegin();
    while (i != names.constEnd()) {
        if (i.value() == name) {
            return i.key();
        }
        ++i;
    }
    return -1;
}
QHash<int, QByteArray> Repositories::roleNames() const {
    QHash<int, QByteArray> names = QAbstractItemModel::roleNames();
    names.insert(Qt::UserRole + 0, "current");
    names.insert(Qt::UserRole + 1, "displayName");
    names.insert(Qt::UserRole + 2, "id");
    return names;
}
QVariant Repositories::headerData(int section, Qt::Orientation orientation, int role) const
{
    if (orientation != Qt::Horizontal) {
        return QVariant();
    }
    return m_headerData.value(qMakePair(section, (Qt::ItemDataRole)role), role == Qt::DisplayRole ?QString::number(section + 1) :QVariant());
}

bool Repositories::setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role)
{
    if (orientation != Qt::Horizontal) {
        return false;
    }
    m_headerData.insert(qMakePair(section, (Qt::ItemDataRole)role), value);
    return true;
}

extern "C" {
    Repositories::Private* repositories_new(Repositories*, void (*)(Repositories*),
        void (*)(const Repositories*),
        void (*)(Repositories*),
        void (*)(Repositories*),
        void (*)(Repositories*, quintptr, quintptr),
        void (*)(Repositories*),
        void (*)(Repositories*),
        void (*)(Repositories*, int, int),
        void (*)(Repositories*),
        void (*)(Repositories*, int, int, int),
        void (*)(Repositories*),
        void (*)(Repositories*, int, int),
        void (*)(Repositories*));
    void repositories_free(Repositories::Private*);
    void repositories_active_repository_get(const Repositories::Private*, QString*, qstring_set);
    bool repositories_add(Repositories::Private*, const ushort*, int);
    void repositories_add_last_error(const Repositories::Private*, QString*, qstring_set);
    void repositories_init(Repositories::Private*, const ushort*, int);
    bool repositories_remove(Repositories::Private*, quint64);
    void repositories_set_current(Repositories::Private*, qint64);
};

Branches::Branches(bool /*owned*/, QObject *parent):
    QAbstractItemModel(parent),
    m_d(nullptr),
    m_ownsPrivate(false)
{
    initHeaderData();
}

Branches::Branches(QObject *parent):
    QAbstractItemModel(parent),
    m_d(branches_new(this,
        [](const Branches* o) {
            Q_EMIT o->newDataReady(QModelIndex());
        },
        [](Branches* o) {
            Q_EMIT o->layoutAboutToBeChanged();
        },
        [](Branches* o) {
            o->updatePersistentIndexes();
            Q_EMIT o->layoutChanged();
        },
        [](Branches* o, quintptr first, quintptr last) {
            o->dataChanged(o->createIndex(first, 0, first),
                       o->createIndex(last, 0, last));
        },
        [](Branches* o) {
            o->beginResetModel();
        },
        [](Branches* o) {
            o->endResetModel();
        },
        [](Branches* o, int first, int last) {
            o->beginInsertRows(QModelIndex(), first, last);
        },
        [](Branches* o) {
            o->endInsertRows();
        },
        [](Branches* o, int first, int last, int destination) {
            o->beginMoveRows(QModelIndex(), first, last, QModelIndex(), destination);
        },
        [](Branches* o) {
            o->endMoveRows();
        },
        [](Branches* o, int first, int last) {
            o->beginRemoveRows(QModelIndex(), first, last);
        },
        [](Branches* o) {
            o->endRemoveRows();
        }
)),
    m_ownsPrivate(true)
{
    connect(this, &Branches::newDataReady, this, [this](const QModelIndex& i) {
        this->fetchMore(i);
    }, Qt::QueuedConnection);
    initHeaderData();
}

Branches::~Branches() {
    if (m_ownsPrivate) {
        branches_free(m_d);
    }
}
void Branches::initHeaderData() {
}
Git::Git(bool /*owned*/, QObject *parent):
    QObject(parent),
    m_branches(new Branches(false, this)),
    m_d(nullptr),
    m_ownsPrivate(false)
{
}

Git::Git(QObject *parent):
    QObject(parent),
    m_branches(new Branches(false, this)),
    m_d(git_new(this, m_branches,
        [](const Branches* o) {
            Q_EMIT o->newDataReady(QModelIndex());
        },
        [](Branches* o) {
            Q_EMIT o->layoutAboutToBeChanged();
        },
        [](Branches* o) {
            o->updatePersistentIndexes();
            Q_EMIT o->layoutChanged();
        },
        [](Branches* o, quintptr first, quintptr last) {
            o->dataChanged(o->createIndex(first, 0, first),
                       o->createIndex(last, 0, last));
        },
        [](Branches* o) {
            o->beginResetModel();
        },
        [](Branches* o) {
            o->endResetModel();
        },
        [](Branches* o, int first, int last) {
            o->beginInsertRows(QModelIndex(), first, last);
        },
        [](Branches* o) {
            o->endInsertRows();
        },
        [](Branches* o, int first, int last, int destination) {
            o->beginMoveRows(QModelIndex(), first, last, QModelIndex(), destination);
        },
        [](Branches* o) {
            o->endMoveRows();
        },
        [](Branches* o, int first, int last) {
            o->beginRemoveRows(QModelIndex(), first, last);
        },
        [](Branches* o) {
            o->endRemoveRows();
        }
)),
    m_ownsPrivate(true)
{
    m_branches->m_d = git_branches_get(m_d);
    connect(this->m_branches, &Branches::newDataReady, this->m_branches, [this](const QModelIndex& i) {
        this->m_branches->fetchMore(i);
    }, Qt::QueuedConnection);
}

Git::~Git() {
    if (m_ownsPrivate) {
        git_free(m_d);
    }
}
const Branches* Git::branches() const
{
    return m_branches;
}
Branches* Git::branches()
{
    return m_branches;
}
Log::Log(bool /*owned*/, QObject *parent):
    QAbstractItemModel(parent),
    m_d(nullptr),
    m_ownsPrivate(false)
{
    initHeaderData();
}

Log::Log(QObject *parent):
    QAbstractItemModel(parent),
    m_d(log_new(this,
        [](const Log* o) {
            Q_EMIT o->newDataReady(QModelIndex());
        },
        [](Log* o) {
            Q_EMIT o->layoutAboutToBeChanged();
        },
        [](Log* o) {
            o->updatePersistentIndexes();
            Q_EMIT o->layoutChanged();
        },
        [](Log* o, quintptr first, quintptr last) {
            o->dataChanged(o->createIndex(first, 0, first),
                       o->createIndex(last, 0, last));
        },
        [](Log* o) {
            o->beginResetModel();
        },
        [](Log* o) {
            o->endResetModel();
        },
        [](Log* o, int first, int last) {
            o->beginInsertRows(QModelIndex(), first, last);
        },
        [](Log* o) {
            o->endInsertRows();
        },
        [](Log* o, int first, int last, int destination) {
            o->beginMoveRows(QModelIndex(), first, last, QModelIndex(), destination);
        },
        [](Log* o) {
            o->endMoveRows();
        },
        [](Log* o, int first, int last) {
            o->beginRemoveRows(QModelIndex(), first, last);
        },
        [](Log* o) {
            o->endRemoveRows();
        }
)),
    m_ownsPrivate(true)
{
    connect(this, &Log::newDataReady, this, [this](const QModelIndex& i) {
        this->fetchMore(i);
    }, Qt::QueuedConnection);
    initHeaderData();
}

Log::~Log() {
    if (m_ownsPrivate) {
        log_free(m_d);
    }
}
void Log::initHeaderData() {
}
void Log::filter(const QString& filter)
{
    return log_filter(m_d, filter.utf16(), filter.size());
}
void Log::load(const QString& path)
{
    return log_load(m_d, path.utf16(), path.size());
}
Repositories::Repositories(bool /*owned*/, QObject *parent):
    QAbstractItemModel(parent),
    m_d(nullptr),
    m_ownsPrivate(false)
{
    initHeaderData();
}

Repositories::Repositories(QObject *parent):
    QAbstractItemModel(parent),
    m_d(repositories_new(this,
        repositoriesActiveRepositoryChanged,
        [](const Repositories* o) {
            Q_EMIT o->newDataReady(QModelIndex());
        },
        [](Repositories* o) {
            Q_EMIT o->layoutAboutToBeChanged();
        },
        [](Repositories* o) {
            o->updatePersistentIndexes();
            Q_EMIT o->layoutChanged();
        },
        [](Repositories* o, quintptr first, quintptr last) {
            o->dataChanged(o->createIndex(first, 0, first),
                       o->createIndex(last, 0, last));
        },
        [](Repositories* o) {
            o->beginResetModel();
        },
        [](Repositories* o) {
            o->endResetModel();
        },
        [](Repositories* o, int first, int last) {
            o->beginInsertRows(QModelIndex(), first, last);
        },
        [](Repositories* o) {
            o->endInsertRows();
        },
        [](Repositories* o, int first, int last, int destination) {
            o->beginMoveRows(QModelIndex(), first, last, QModelIndex(), destination);
        },
        [](Repositories* o) {
            o->endMoveRows();
        },
        [](Repositories* o, int first, int last) {
            o->beginRemoveRows(QModelIndex(), first, last);
        },
        [](Repositories* o) {
            o->endRemoveRows();
        }
)),
    m_ownsPrivate(true)
{
    connect(this, &Repositories::newDataReady, this, [this](const QModelIndex& i) {
        this->fetchMore(i);
    }, Qt::QueuedConnection);
    initHeaderData();
}

Repositories::~Repositories() {
    if (m_ownsPrivate) {
        repositories_free(m_d);
    }
}
void Repositories::initHeaderData() {
}
QString Repositories::activeRepository() const
{
    QString v;
    repositories_active_repository_get(m_d, &v, set_qstring);
    return v;
}
bool Repositories::add(const QString& path)
{
    return repositories_add(m_d, path.utf16(), path.size());
}
QString Repositories::addLastError() const
{
    QString s;
    repositories_add_last_error(m_d, &s, set_qstring);
    return s;
}
void Repositories::init(const QString& db_file_name)
{
    return repositories_init(m_d, db_file_name.utf16(), db_file_name.size());
}
bool Repositories::remove(quint64 index)
{
    return repositories_remove(m_d, index);
}
void Repositories::setCurrent(qint64 id)
{
    return repositories_set_current(m_d, id);
}
