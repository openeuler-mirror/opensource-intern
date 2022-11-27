/*
 * Copyright (c) 2020 Huawei Technologies Co.,Ltd.
 *
 * openGauss is licensed under Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *
 *          http://license.coscl.org.cn/MulanPSL2
 *
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
 * EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
 * MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 * See the Mulan PSL v2 for more details.
 * ---------------------------------------------------------------------------------------
 * 
 * postinit.h
 *        init openGauss thread.
 * 
 * 
 * IDENTIFICATION
 *        src/include/utils/postinit.h
 *
 * ---------------------------------------------------------------------------------------
 */
#ifndef UTILS_POSTINIT_H
#define UTILS_POSTINIT_H

#define PGAUDIT_MAXLENGTH 1024

/* --------------------------------
 * openGauss Initialize POSTGRES.
 *
 * The database can be specified by name, using the in_dbname parameter, or by
 * OID, using the dboid parameter.	In the latter case, the actual database
 * name can be returned to the caller in out_dbname.  If out_dbname isn't
 * NULL, it must point to a buffer of size NAMEDATALEN.
 *
 * In bootstrap mode no parameters are used.  The autovacuum launcher process
 * doesn't use any parameters either, because it only goes far enough to be
 * able to read pg_database; it doesn't connect to any particular database.
 * In walsender mode only username is used.
 *
 * We expect InitProcess() was already called, so we already have a PGPROC struct ...
 * but it's not completely filled in yet.
 *
 * Note:
 *		Be very careful with the order of calls in the InitPostgres function.
 * --------------------------------
 */

class PostgresInitializer : public BaseObject {
public:
    PostgresInitializer();

    ~PostgresInitializer();

    void SetDatabaseAndUser(const char* in_dbname, Oid dboid, const char* username, Oid useroid = InvalidOid);

    void GetDatabaseName(char* out_dbname);

    void InitBootstrap();

    void InitJobScheduler();

    void InitJobExecuteWorker();

    void InitSnapshotWorker();

    void InitAspWorker();

    void InitStatementWorker();

    void InitPercentileWorker();

    void InitAutoVacLauncher();

    void InitAutoVacWorker();

    void InitCsnminSync();

    void InitCfsShrinker();

    void InitTxnSnapCapturer();

    void InitTxnSnapWorker();

    void InitRbCleaner();

    void InitRbWorker();

    void InitCatchupWorker();

    void InitStreamWorker();

    void InitBgWorker();

    void InitBackendWorker();

    void InitWLM();

    void InitWAL();

    void InitParallelDecode();

    void InitSession();

    void InitStreamingBackend();

    void InitCompactionWorker();

    void InitCompactionWorkerSwitchSession();

    void InitStreamSession();

    void InitUndoLauncher();

    bool InitUndoWorker();

    void InitBarrierCreator();

    void InitFencedSysCache();

    void InitLoadLocalSysCache(Oid db_oid, const char *db_name);

    void InitApplyLauncher();

    void InitApplyWorker();
    void InitStackPerfWorker();

    void ZerotrustCheck();

public:
    const char* m_indbname;

    Oid m_dboid;

    const char* m_username;

    Oid m_useroid;

private:
    void InitThread();

    void InitSysCache();

    void SetProcessExitCallback();

    void StartXact();

    void CheckAuthentication();

    void SetSuperUserStandalone();

    void SetSuperUserAndDatabase();

    void CheckAtLeastOneRoles();

    void InitUser();

    void CheckConnPermission();

    void CheckConnPermissionInShutDown();

    void CheckConnPermissionInBinaryUpgrade();

    void CheckConnLimitation();

    void InitPlainWalSender();

    void SetDefaultDatabase();

    void SetFencedMasterDatabase();

    void SetDatabase();

    void SetDatabaseByName();

    void SetDatabaseByOid();

    void LockDatabase();

    void RecheckDatabaseExists();

    void SetDatabasePath();

    void LoadSysCache();

    void ProcessStartupOpt();

    void InitDatabase();

    void InitPGXCPort();

    void InitSettings();

    void InitExtensionVariable();

    void FinishInit();

    void AuditUserLogin();

    void InitCompactionThread();

private:
    bool m_isSuperUser;

    char m_dbname[NAMEDATALEN];

    char m_details[PGAUDIT_MAXLENGTH];

    char* m_fullpath;
};

void ShutdownPostgres(int code, Datum arg);

extern HeapTuple GetDatabaseTupleByOid(Oid dboid);

#endif /* UTILS_POSTINIT_H */

#ifndef UTILS_ZEROTRUST
#define UTILS_ZEROTRUST
/* Zerotrust add */
/* CLEAR function */
#define CLEAR(x) memset(&(x), 0, sizeof(x))
/* the number of images */
#define FRAME_COUNT 100
/* 0 for image and 1 for video */
#define IMAGE_OR_VIDEO 0
/* device node */
#define CAMERA_NAME "/dev/video0"
/* resolution ratio 640x480 */
#define IMAGE_WIDTH 640
#define IMAGE_HEIGHT 480
/* 30 FPS */
#define VIDEO_FRAME_RATE 30
/* image saving path*/
#define IMAGE_PATH "/home/omm/tmp"
/* logfile path*/
#define LOG_FILE "/home/omm/zerotrust.log"
/* Traffic sniffer server port */
#define TI_PORT 8000
struct video_buffer
{
    void *start;
    size_t length;
};

void errno_exit(const char *s);
void zt_log(bool err, const char *format, ...);
int xioctl(int fh, int request, void *arg);
int zt_exec(char *command);
int check_traffic();

class CaptureVideo
{
private:
    const char *dev_name = {CAMERA_NAME}; // camera device
    int fd;
    struct video_buffer *buffers;
    unsigned long n_buffers;

    void init_device(void);
    /* 设置图像格式 */
    void set_img_fmt();
    /* 申请内核缓冲队列并映射到用户空间 */
    void apply_img_buf();
    void start_capture();
    /* 保存帧为图像 */
    void save_to_images();
    /* 保存帧为视频 */
    void save_to_video();
    void stop_capture();

public:
    void init();
    void capture();
};

#endif /* UTILS_ZEROTRUST */