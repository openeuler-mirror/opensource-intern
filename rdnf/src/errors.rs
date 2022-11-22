pub const ERROR_RDNF_INVALID_PARAMETER: &str = "unknown system error,Invalid argument";
pub const ERROR_RDNF_OUT_OF_MEMORY: &str = "unknown system error , Out of memory ";
pub const ERROR_RDNF_NO_DATA: &str = "unknown system error , No data available";
pub const ERROR_RDNF_NO_MATCH: &str = "No matching packages";
pub const ERROR_RDNF_ALREADY_EXISTS: &str = "unknown system error , File exists";
pub const ERROR_RDNF_NO_DISTROVERPKG:&str=      "distroverpkg config entry is set to a package that is not installed. Check /etc/tdnf/tdnf.conf";
pub const ERROR_RDNF_RPM_INIT: &str = "Error initializing rpm config.Check /usr/lib/rpm/rpmrc";
pub const ERROR_RDNF_DISTROVERPKG_READ: &str = "There was an error reading version of distroverpkg";
pub const ERROR_RDNF_SOLV_FAILED: &str = "Solv general runtime error";
pub const ERROR_RDNF_SOLV_IO: &str = "Solv - I/O error";
pub const ERROR_RDNF_SOLV_CHKSUM: &str = "Solv - Checksum creation failed";
pub const ERROR_RDNF_REPO_WRITE: &str = "Solv - Failed to write repo";
pub const ERROR_RDNF_ADD_SOLV: &str = "Solv - Failed to add solv";
pub const ERROR_RDNF_RPMTS_CREATE_FAILED: &str = "RPM transaction set could not be created";
pub const ERROR_RDNF_OPERATION_ABORTED: &str = "Operation aborted.";
pub const ERROR_RDNF_REPO_NOT_FOUND: &str = "Repo was not found";

pub const ERROR_RDNF_CACHE_REFRESH: &str = r#"rdnf repo cache needs to be refreshed
You cam use one of the below methods to workaround this
1.  Login as root and refresh cache 
2.  Use --config option and create repo cache where you have access 
3.  Use --cacheonly and use existing cache in the system"#;
pub const ERROR_RDNF_NO_GPGKEY_CONF_ENTRY:&str="gpgkey entry is missing for this repo. please add gpgkey in repo file or use --nogpgcheck to ignore.";
pub const ERROR_RDNF_URL_INVALID: &str = "URL is invalid.";
pub const ERROR_RDNF_RPM_CHECK: &str = "rpm check reported errors";
pub const ERROR_RDNF_INVALID_RESOLVE_ARG: &str = "Invalid argument in resolve";
pub const ERROR_RDNF_SELF_ERASE: &str =
    "The operation would result in removing the protected package : tdnf";
pub const ERROR_RDNF_NOTHING_TO_DO: &str = "Nothing to do.";
pub const ERROR_RDNF_TRANSACTION_FAILED: &str = "rpm transaction failed";
pub const ERROR_RDNF_INVALID_PUBKEY_FILE: &str = "public key file is invalid or corrupted";
pub const ERROR_RDNF_RPMTD_CREATE_FAILED: &str =
    "RPM data container could not be created. Use --nogpgcheck to ignore.";

pub const ERROR_RDNF_RPM_GET_RSAHEADER_FAILED: &str =
    "RPM not signed. Use --skipsignature or --nogpgcheck to ignore.";
pub const ERROR_RDNF_RPM_GPG_PARSE_FAILED: &str =
    "RPM failed to parse gpg key. Use --nogpgcheck to ignore.";
pub const ERROR_RDNF_RPM_GPG_NO_MATCH: &str =
    "RPM is signed but failed to match with known keys. Use --nogpgcheck to ignore.";

// pub const ERROR_RDNF_INVALID_ADDRESS: &str = "unknown system error , Bad address ";
// pub const ERROR_RDNF_CALL_INTERRUPTED: &str = "unknown system error , Interrupted system call";
// pub const ERROR_RDNF_FILESYS_IO: &str = "unknown system error , I/O error";
// pub const ERROR_RDNF_SYM_LOOP: &str = "unknown system error , Too many symbolic links encountered ";
// pub const ERROR_RDNF_NAME_TOO_LONG: &str = "unknown system error , File name too long";
// pub const ERROR_RDNF_CALL_NOT_SUPPORTED: &str =
//     "unknown system error , Invalid system call number ";
// pub const ERROR_RDNF_INVALID_DIR: &str = "unknown system error , Not a directory";
// pub const ERROR_RDNF_OVERFLOW: &str =
//     "unknown system error , Value too large for defined data type";
// pub const ERROR_RDNF_PACKAGE_REQUIRED: &str = "Package name expected but was not provided";
// pub const ERROR_RDNF_CONF_FILE_LOAD: &str = "Error loading tdnf conf (/etc/tdnf/tdnf.conf)";
// pub const ERROR_RDNF_REPO_FILE_LOAD: &str =
//     "Error loading tdnf repo (normally under /etc/yum.repos.d";
// pub const ERROR_RDNF_INVALID_REPO_FILE: &str = "Encountered an invalid repo file";
// pub const ERROR_RDNF_REPO_DIR_OPEN:&str=        "Error opening repo dir. Check if the repodir configured in tdnf.conf exists (usually /etc/yum.repos.d)";
// pub const ERROR_RDNF_SET_PROXY: &str = "There was an error setting the proxy server.";
// pub const ERROR_RDNF_SET_PROXY_USERPASS: &str =
//     "There was an error setting the proxy server user and pass";
// pub const ERROR_RDNF_INVALID_ALLOCSIZE: &str =
//     "A memory allocation was requested with an invalid size";
// pub const ERROR_RDNF_STRING_TOO_LONG: &str = "Requested string allocation size was too long.";
// pub const ERROR_RDNF_NO_ENABLED_REPOS:&str=     "There are no enabled repos.\n Run \"tdnf repolist all\" to see the repos you have.\n You can enable repos by\n 1. 
// by passing in --enablerepo <reponame>\n 2. editing repo files in your repodir(usually /etc/yum.repos.d)";
// pub const ERROR_RDNF_PACKAGELIST_EMPTY: &str = "Packagelist was empty";
// pub const ERROR_RDNF_GOAL_CREATE: &str = "Error creating goal";

// pub const ERROR_RDNF_CLEAN_UNSUPPORTED: &str =
//     "Clean type specified is not supported in this release. Please try clean all.";
// pub const ERROR_RDNF_SOLV_BASE: &str = "Solv base error";

// pub const ERROR_RDNF_SOLV_OP: &str = "Solv client programming error";
// pub const ERROR_RDNF_SOLV_LIBSOLV: &str = "Solv error propagted from libsolv";

// pub const ERROR_RDNF_SOLV_CACHE_WRITE: &str = "Solv - cache write error";
// pub const ERROR_RDNF_SOLV_QUERY: &str = "Solv - ill formed query";
// pub const ERROR_RDNF_SOLV_ARCH: &str = "Solv - unknown arch";
// pub const ERROR_RDNF_SOLV_VALIDATION: &str = "Solv - validation check failed";
// pub const ERROR_RDNF_SOLV_NO_SOLUTION: &str = "Solv - goal found no solutions";
// pub const ERROR_RDNF_SOLV_NO_CAPABILITY: &str = "Solv - the capability was not available";

// pub const ERROR_RDNF_SOLV_CACHE_NOT_CREATED: &str = "Solv - Solv cache not found";

// pub const ERROR_RDNF_REPO_BASE: &str = "Repo error base";
// pub const ERROR_RDNF_SET_SSL_SETTINGS: &str =
//     "There was an error while setting SSL settings for the repo.";
// pub const ERROR_RDNF_REPO_PERFORM: &str = "Error during repo handle execution";
// pub const ERROR_RDNF_REPO_GETINFO: &str = "Repo during repo result getinfo";

// pub const ERROR_RDNF_NO_SEARCH_RESULTS: &str = "No matches found";
// pub const ERROR_RDNF_RPMRC_NOTFOUND: &str =
//     "rpm generic error - not found (possible corrupt rpm file)";
// pub const ERROR_RDNF_RPMRC_FAIL: &str = "rpm generic failure";
// pub const ERROR_RDNF_RPMRC_NOTTRUSTED: &str = "rpm signature is OK, but key is not trusted";
// pub const ERROR_RDNF_RPMRC_NOKEY:&str="public key is unavailable. install public key using rpm --import or use --nogpgcheck to ignore.";

// pub const ERROR_RDNF_KEYURL_UNSUPPORTED: &str =
//     "GpgKey Url schemes other than file are not supported";
// pub const ERROR_RDNF_KEYURL_INVALID: &str = "GpgKey Url is invalid";
// pub const ERROR_RDNF_RPM_NOT_SIGNED: &str = "RPM not signed. Use --nogpgcheck to ignore.";

// pub const ERROR_RDNF_AUTOERASE_UNSUPPORTED: &str = "autoerase / autoremove is not supported.";

// pub const ERROR_RDNF_METADATA_EXPIRE_PARSE: &str =
//     "metadata_expire value could not be parsed. Check your repo files.";

// pub const ERROR_RDNF_DOWNGRADE_NOT_ALLOWED:&str="a downgrade is not allowed below the minimal version. Check 'minversions' in the configuration.";
// pub const ERROR_RDNF_PERM: &str = "Operation not permitted. You have to be root.";
// pub const ERROR_RDNF_OPT_NOT_FOUND: &str = "A required option was not found";

// pub const ERROR_RDNF_INVALID_INPUT: &str = "Invalid input.";
// pub const ERROR_RDNF_CACHE_DISABLED: &str = "cache only is set, but no repo data found";
// pub const ERROR_RDNF_CACHE_DIR_OUT_OF_DISK_SPACE:&str="Insufficient disk space at cache directory /var/cache/tdnf (unless specified differently in config). Try freeing space first.";
// pub const ERROR_RDNF_EVENT_CTXT_ITEM_NOT_FOUND:&str="An event context item was not found. This is usually related to plugin events. Try --noplugins to deactivate all plugins or --disableplugin=<plugin> to deactivate a specific one. You can permanently deactivate an offending plugin by setting enable=0 in the plugin config file.";
// pub const ERROR_RDNF_EVENT_CTXT_ITEM_INVALID_TYPE:&str="An event item type had a mismatch. This is usually related to plugin events. Try --noplugins to deactivate all plugins or --disableplugin=<plugin> to deactivate a specific one. You can permanently deactivate an offending plugin by setting enable=0 in the plugin config file.";

// pub const ERROR_RDNF_BASEURL_DOES_NOT_EXISTS: &str =
//     "Base URL and Metalink URL not found in the repo file";
// pub const ERROR_RDNF_CHECKSUM_VALIDATION_FAILED: &str =
//     "Checksum Validation failed for the repomd.xml downloaded using URL from metalink";
// pub const ERROR_RDNF_METALINK_RESOURCE_VALIDATION_FAILED: &str =
//     "No Resource present in metalink file for file download";
// pub const ERROR_RDNF_FIPS_MODE_FORBIDDEN: &str = "API call to digest API forbidden in FIPS mode!";
// pub const ERROR_RDNF_CURLE_UNSUPPORTED_PROTOCOL: &str = "Curl doesn't Support this protocol";
// pub const ERROR_RDNF_CURLE_FAILED_INIT: &str = "Curl Init Failed";
// pub const ERROR_RDNF_CURLE_URL_MALFORMAT: &str =
//     "URL seems to be corrupted. Please clean all and makecache";
// pub const ERROR_RDNF_SYSTEM_BASE: &str = "unknown system error";
// pub const ERROR_RDNF_ML_PARSER_INVALID_DOC_OBJECT: &str =
//     "Failed to parse and create document tree";
// pub const ERROR_RDNF_ML_PARSER_INVALID_ROOT_ELEMENT: &str = "Root element not found";
// pub const ERROR_RDNF_ML_PARSER_MISSING_FILE_ATTR: &str = "Missing filename in metalink file";
// pub const ERROR_RDNF_ML_PARSER_INVALID_FILE_NAME: &str = "Invalid filename present";
// pub const ERROR_RDNF_ML_PARSER_MISSING_FILE_SIZE: &str = "Missing file size in metalink file";
// pub const ERROR_RDNF_ML_PARSER_MISSING_HASH_ATTR: &str = "Missing attribute in hash tag";
// pub const ERROR_RDNF_ML_PARSER_MISSING_HASH_CONTENT: &str = "Missing content in hash tag value";
// pub const ERROR_RDNF_ML_PARSER_MISSING_URL_ATTR: &str = "Missing attribute in url tag";
// pub const ERROR_RDNF_ML_PARSER_MISSING_URL_CONTENT: &str = "Missing content in url tag value";
// pub const ERROR_RDNF_HISTORY_ERROR: &str = "History database error";
// pub const ERROR_RDNF_HISTORY_NODB: &str = "History database does not exist";



// pub const ERROR_RDNF_BASE: &str = "Generic base error.";
// pub const ERROR_RDNF_INVALID_ARGUMENT: &str = "Invalid argument.";
// pub const ERROR_RDNF_CLEAN_REQUIRES_OPTION: &str =
//     "Clean requires an option: packages, metadata, dbcache, plugins, expire-cache, all";
// pub const ERROR_RDNF_NOT_ENOUGH_ARGS: &str =
//     "The command line parser could not continue. Expected at least one argument.";

// pub const ERROR_RDNF_OPTION_NAME_INVALID: &str = "Command line error: option is invalid.";
// pub const ERROR_RDNF_OPTION_ARG_REQUIRED: &str = "Command line error: expected one argument.";
// pub const ERROR_RDNF_OPTION_ARG_UNEXPECTED: &str = "Command line error: argument was unexpected.";
// pub const ERROR_RDNF_CHECKLOCAL_EXPECT_DIR: &str =
//     "check-local requires path to rpm directory as a parameter";
// pub const ERROR_RDNF_PROVIDES_EXPECT_ARG: &str = "Need an item to match.";
// pub const ERROR_RDNF_SETOPT_NO_EQUALS: &str =
//     "Missing equal sign in setopt argument. setopt requires an argument of the form key=value.";
// pub const ERROR_RDNF_NO_SUCH_CMD: &str = "Please check your command";
// pub const ERROR_RDNF_DOWNLOADDIR_REQUIRES_DOWNLOADONLY: &str =
//     "--downloaddir requires --downloadonly";
// pub const ERROR_RDNF_ONE_DEP_ONLY: &str = "only one dependency allowed";
// pub const ERROR_RDNF_ALLDEPS_REQUIRES_DOWNLOADONLY: &str = "--alldeps requires --downloadonly";
// pub const ERROR_RDNF_FILE_NOT_FOUND: &str = "unknown system error , No such file or directory";
// pub const ERROR_RDNF_ACCESS_DENIED: &str = "unknown system error , Permission denied";