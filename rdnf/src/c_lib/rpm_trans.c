#include <rpm/rpmtypes.h>
#include <rpm/rpmts.h>
#include <rpm/rpmio.h>
#include <rpm/header.h>
#include <stdio.h>
typedef struct _CALL_BACK_CONTEXT_
{
    int quiet;
    FD_t fd;
} CallbackContext;

void *rdnf_callback_fn(
    const void *pArg,
    const rpmCallbackType what,
    const rpm_loff_t amount,
    const rpm_loff_t total,
    fnpyKey key,
    rpmCallbackData data)
{
    Header pkg_header_ptr = (Header)pArg;
    void *pResult = NULL;
    char *file_path = (char *)key;
    CallbackContext *context = (CallbackContext *)data;
    int quiet=context->quiet %2;
    int term_width=context->quiet >> 1;
    char *nevra = headerGetAsString(pkg_header_ptr, RPMTAG_NEVRA);
    int len=0;
    term_width=term_width > 250?250:term_width;
    term_width=term_width < 80 ? 80:term_width;
    switch (what)
    {
    case RPMCALLBACK_INST_OPEN_FILE:
        if ((!file_path) || !(*file_path))
        {
            return NULL;
        }
        context->fd = Fopen(file_path, "r.udfio");
        return (void *)context->fd;
        break;
    case RPMCALLBACK_INST_CLOSE_FILE:
        if (context->fd)
        {
            Fclose(context->fd);
            context->fd = NULL;
        }
        break;
    case RPMCALLBACK_INST_START:
        if (!quiet)
        {
            len= term_width - strlen("Installing") - 2;
            printf("%-*s\e[32mInstalling\e[0m  \r", len, nevra);
            (void)fflush(stdout);
        }
        break;
    case RPMCALLBACK_INST_STOP:
        if(!quiet){
            len=term_width-strlen("Installed")-2;
            printf("%-*s\e[32mInstalled\e[0m  \n", len, nevra);
            (void)fflush(stdout);
        }
        break;
    case RPMCALLBACK_UNINST_START:
        if (!quiet)
        {
            len = term_width - strlen("Removing") - 2;
            printf("%-*s\e[32mRemoving\e[0m  \r", len, nevra);
            (void)fflush(stdout);
        }
        break;
    case RPMCALLBACK_UNINST_STOP:
        if (!quiet){
            len=term_width-strlen("Removed")-2;
            printf("%-*s\e[32mRemoved\e[0m  \n",len,nevra);
            (void)fflush(stdout);
        }
        break;
    case RPMCALLBACK_SCRIPT_ERROR:
    {
        /* https://bugzilla.redhat.com/show_bug.cgi?id=216221#c15 */
        const char *pszScript;
        switch (amount)
        {
        case RPMTAG_PREIN:
            pszScript = "%prein";
            break;
        case RPMTAG_POSTIN:
            pszScript = "%postin";
            break;
        case RPMTAG_PREUN:
            pszScript = "%preun";
            break;
        case RPMTAG_POSTUN:
            pszScript = "%postun";
            break;
        default:
            pszScript = "(unknown)";
            break;
        }
        /* %pre and %preun will cause errors (install/uninstall will fail),
           other scripts just warn (install/uninstall will succeed) */
        if (total == RPMRC_OK)
        {
            len = term_width - strlen("warning in ") - 12;
            printf("%-*s\e[33mwarning in \e[0m%-*s", len, nevra, 12, pszScript);
            (void)fflush(stdout);
        }
        else
        {
            len = term_width- strlen("error   in ") - 12;
            printf("%-*s\e[31error    in \e[0m%-*s", len, nevra, 12, pszScript);
            (void)fflush(stdout);
        }
    }
    break;
    default:
        break;
    }
    if (nevra!=NULL){
        free(nevra);
    }
    return pResult;
}
int set_callback_fn(rpmts ts, int quiet, uint16_t term_width)
{
    CallbackContext p = {0};
    p.quiet = term_width <<1;
    p.quiet +=quiet;
    return rpmtsSetNotifyCallback(ts, rdnf_callback_fn, (void *)&p);
}