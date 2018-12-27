/* generated by rust_qt_binding_generator */
#ifndef BINDINGS_H
#define BINDINGS_H

#include <QtCore/QObject>
#include <QtCore/QAbstractItemModel>

class Branches;
class Commit;
class Diffs;
class Git;
class Log;
class Repositories;

class Branches : public QAbstractItemModel
{
    Q_OBJECT
    friend class Git;
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    explicit Branches(bool owned, QObject *parent);
public:
    explicit Branches(QObject *parent = nullptr);
    ~Branches();

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool checkedout(int row) const;
    Q_INVOKABLE QString name(int row) const;
    Q_INVOKABLE QString oid(int row) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
};

class Commit : public QObject
{
    Q_OBJECT
    friend class Git;
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(QString author READ author NOTIFY authorChanged FINAL)
    Q_PROPERTY(QString cid READ cid NOTIFY cidChanged FINAL)
    Q_PROPERTY(QString committer READ committer NOTIFY committerChanged FINAL)
    Q_PROPERTY(QString message READ message NOTIFY messageChanged FINAL)
    Q_PROPERTY(QString time READ time NOTIFY timeChanged FINAL)
    Q_PROPERTY(QString tree READ tree NOTIFY treeChanged FINAL)
    explicit Commit(bool owned, QObject *parent);
public:
    explicit Commit(QObject *parent = nullptr);
    ~Commit();
    QString author() const;
    QString cid() const;
    QString committer() const;
    QString message() const;
    QString time() const;
    QString tree() const;
Q_SIGNALS:
    void authorChanged();
    void cidChanged();
    void committerChanged();
    void messageChanged();
    void timeChanged();
    void treeChanged();
};

class Diffs : public QAbstractItemModel
{
    Q_OBJECT
    friend class Git;
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    explicit Diffs(bool owned, QObject *parent);
public:
    explicit Diffs(QObject *parent = nullptr);
    ~Diffs();

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE QString filename(int row) const;
    Q_INVOKABLE QString patch(int row) const;
    Q_INVOKABLE QString status(int row) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
};

class Git : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Branches* const m_branches;
    Commit* const m_commit;
    Diffs* const m_diffs;
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(Branches* branches READ branches NOTIFY branchesChanged FINAL)
    Q_PROPERTY(Commit* commit READ commit NOTIFY commitChanged FINAL)
    Q_PROPERTY(Diffs* diffs READ diffs NOTIFY diffsChanged FINAL)
    Q_PROPERTY(QString revwalkFilter READ revwalkFilter NOTIFY revwalkFilterChanged FINAL)
    explicit Git(bool owned, QObject *parent);
public:
    explicit Git(QObject *parent = nullptr);
    ~Git();
    const Branches* branches() const;
    Branches* branches();
    const Commit* commit() const;
    Commit* commit();
    const Diffs* diffs() const;
    Diffs* diffs();
    QString revwalkFilter() const;
    Q_INVOKABLE void load(const QString& path);
    Q_INVOKABLE void loadCommit(const QString& oid);
Q_SIGNALS:
    void branchesChanged();
    void commitChanged();
    void diffsChanged();
    void revwalkFilterChanged();
};

class Log : public QAbstractItemModel
{
    Q_OBJECT
    friend class Git;
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    explicit Log(bool owned, QObject *parent);
public:
    explicit Log(QObject *parent = nullptr);
    ~Log();
    Q_INVOKABLE void filter(const QString& filter);
    Q_INVOKABLE void load(const QString& path);

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE QString author(int row) const;
    Q_INVOKABLE QString cid(int row) const;
    Q_INVOKABLE QByteArray graph(int row) const;
    Q_INVOKABLE QString message(int row) const;
    Q_INVOKABLE QString time(int row) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
};

class Repositories : public QAbstractItemModel
{
    Q_OBJECT
    friend class Git;
public:
    class Private;
private:
    Private * m_d;
    bool m_ownsPrivate;
    Q_PROPERTY(QString activeRepository READ activeRepository NOTIFY activeRepositoryChanged FINAL)
    explicit Repositories(bool owned, QObject *parent);
public:
    explicit Repositories(QObject *parent = nullptr);
    ~Repositories();
    QString activeRepository() const;
    Q_INVOKABLE bool add(const QString& path);
    Q_INVOKABLE QString addLastError() const;
    Q_INVOKABLE void init(const QString& db_file_name);
    Q_INVOKABLE bool remove(quint64 index);
    Q_INVOKABLE void setCurrent(qint64 id);

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;
    void sort(int column, Qt::SortOrder order = Qt::AscendingOrder) override;
    int role(const char* name) const;
    QHash<int, QByteArray> roleNames() const override;
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;
    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;
    Q_INVOKABLE bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    Q_INVOKABLE bool current(int row) const;
    Q_INVOKABLE QString displayName(int row) const;
    Q_INVOKABLE qint64 id(int row) const;

Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();
Q_SIGNALS:
    void activeRepositoryChanged();
};
#endif // BINDINGS_H
