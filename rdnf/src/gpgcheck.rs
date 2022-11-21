use std::{ffi::CString, fs::File, io::Read};

use crate::{
    errors::{ERROR_RDNF_NO_GPGKEY_CONF_ENTRY, ERROR_RDNF_OPERATION_ABORTED, ERROR_RDNF_INVALID_PUBKEY_FILE, ERROR_RDNF_RPMTS_CREATE_FAILED, ERROR_RDNF_RPMTD_CREATE_FAILED, ERROR_RDNF_RPM_GET_RSAHEADER_FAILED, ERROR_RDNF_RPM_GPG_PARSE_FAILED, ERROR_RDNF_RPM_GPG_NO_MATCH},
    rpm_trans::{RpmTs, RPMVSF_MASK_NOSIGNATURES},
    sub_command::{install::is_remote_url, repo::RepoData},
    Rdnf, cli::AlterOption, c_lib::{char_ptr_offset},
};
use anyhow::{bail, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};
use rpm_sys::ffi::{
    rpmRC_e_RPMRC_NOKEY, rpmRC_e_RPMRC_NOTTRUSTED, rpmReadPackageFile, Fclose, Fopen, Header, rpmts, pgpParsePkts, pgpArmor_e_PGPARMOR_PUBKEY, rpmtsImportPubkey, rpmtsGetKeyring, rpmKeyring_s, rpmPubkeyNew, rpmPubkeyDig,  rpmKeyringLookup, rpmRC_e_RPMRC_OK, rpmKeyringAddKey, rpmtsCreate, rpmtsSetVSFlags, rpmtdNew, headerConvert, headerConvOps_e_HEADERCONV_RETROFIT_V3, headerGet, rpmTag_e_RPMTAG_RSAHEADER, headerGetFlags_e_HEADERGET_MINMEM, pgpNewDig, pgpPrtPkts, pgpFreeDig, headerFree, rpmtdFree, rpmtsFree,
};

impl Rdnf {
    pub fn gpgcheck_pkg(&self, rpm_ts: &mut RpmTs, file_path: &str, repo: &RepoData,alter_args:&AlterOption) -> Result<(Header,bool)> {
        let mut gpg_sig_check = false;
        let mut url_gpg_keys = None;
        if !(alter_args.no_gpg_check || alter_args.skip_signatures) {
            if repo.base.gpgcheck {
                gpg_sig_check = true;
                if repo.details.url_gpg_keys.is_some() {
                    url_gpg_keys = repo.details.url_gpg_keys.clone();
                }
            }
        }
        let file_path_c = CString::new(file_path).unwrap();
        let fmode = CString::new("r.ufdio").unwrap();
        let fd = unsafe { Fopen(file_path_c.as_ptr(), fmode.as_ptr()) };
        let mut rpm_header = 0 as Header;
        let rpm_rc =
            unsafe { rpmReadPackageFile(rpm_ts.ts, fd, file_path_c.as_ptr(), &mut rpm_header) };
        if (rpm_rc == rpmRC_e_RPMRC_NOTTRUSTED || rpm_rc == rpmRC_e_RPMRC_NOKEY) && gpg_sig_check {
            if url_gpg_keys.is_none() {
                bail!(ERROR_RDNF_NO_GPGKEY_CONF_ENTRY);
            }
            let mut matched=0;
            for url in url_gpg_keys.unwrap() {
                let prompt = format!("Importing key from {}, is this ok", url.as_str());
                if !Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(prompt)
                    .interact()
                    .unwrap()
                {
                    bail!(ERROR_RDNF_OPERATION_ABORTED);
                };
                let key_local_path = if is_remote_url(url.as_str()) {
                    self.download_key_to_cache(url.as_str(), repo)?
                } else {
                    match url.split_once("file://") {
                        Some((_, rest)) => "/".to_string() + rest.trim_start_matches("/"),
                        None => url,
                    }
                };
                import_gpgkey_file(rpm_ts.ts,key_local_path.as_str())?;
                let key_ring=unsafe{rpmtsGetKeyring(rpm_ts.ts, 0)};
                if gpgcheck(key_ring,key_local_path.as_str(),file_path)?{
                    matched+=1;
                }
            }
            if matched==0{
                bail!(ERROR_RDNF_RPM_GPG_NO_MATCH);
            }
            unsafe { rpmReadPackageFile(rpm_ts.ts, fd, file_path_c.as_ptr(), &mut rpm_header) };
        }
        unsafe { Fclose(fd) };
        Ok((rpm_header,gpg_sig_check))
    }
}
pub fn import_gpgkey_file(ts:rpmts,file_path:&str)->Result<()>{
    let mut file=File::open(file_path)?;
    let mut buffer=String::new();
    file.read_to_string(&mut buffer)?;
    let data_size=buffer.len();
    let mut offset=0;
    let buf = CString::new(buffer).unwrap();
    let mut pkt_ptr=0 as *mut u8;
    let mut pkt_len=0 as u64;
    let mut keys=0;
    while (offset as usize) < data_size {
        unsafe{
            let buf_ptr=char_ptr_offset(buf.as_ptr(), offset);
            let armor_res=pgpParsePkts(buf_ptr,&mut pkt_ptr as *mut *mut u8,&mut pkt_len as *mut u64);
            if armor_res==pgpArmor_e_PGPARMOR_PUBKEY{
                if rpmtsImportPubkey(ts, pkt_ptr, pkt_len)!=0{
                    bail!(ERROR_RDNF_INVALID_PUBKEY_FILE);
                }
                keys+=1;
            }
            offset+=pkt_len as i32;

        };
    }
    if keys==0{
        bail!(ERROR_RDNF_INVALID_PUBKEY_FILE);
    }
    Ok(())
}
pub fn gpgcheck(key_ring:*mut rpmKeyring_s,key_file_path:&str,pkg_file:&str)->Result<bool>{
    add_key_file_to_keyring(key_ring,key_file_path)?;
    Ok(verify_rpm_sig(key_ring,pkg_file)?)
}
pub fn add_key_file_to_keyring(key_ring:*mut rpmKeyring_s,key_file_path:&str)->Result<()>{
    let mut file=File::open(key_file_path)?;
    let mut buffer=String::new();
    file.read_to_string(&mut buffer)?;
    let data_size=buffer.len();
    let mut offset=0;
    let buf = CString::new(buffer).unwrap();
    let mut pkt_ptr=0 as *mut u8;
    let mut pkt_len=0 as u64;
    let mut keys=0;
    while (offset as usize) < data_size {
        unsafe{
            let buf_ptr=char_ptr_offset(buf.as_ptr(), offset);
            let armor_res=pgpParsePkts(buf_ptr,&mut pkt_ptr as *mut *mut u8,&mut pkt_len as *mut u64);
            if armor_res==pgpArmor_e_PGPARMOR_PUBKEY{
                let pubkey=rpmPubkeyNew(pkt_ptr, pkt_len);
                if pubkey.is_null() {
                    bail!(ERROR_RDNF_INVALID_PUBKEY_FILE)
                }
                let sig=rpmPubkeyDig(pubkey);
                if sig.is_null() {
                    bail!(ERROR_RDNF_INVALID_PUBKEY_FILE);
                }
                if rpmKeyringLookup(key_ring, sig) !=rpmRC_e_RPMRC_OK{
                    rpmKeyringAddKey(key_ring, pubkey);
                };
                keys+=1;
            }
            offset+=pkt_len as i32;

        };
    }
    if keys==0{
        bail!(ERROR_RDNF_INVALID_PUBKEY_FILE);
    }
    Ok(())
}
pub fn verify_rpm_sig(key_ring:*mut rpmKeyring_s,pkg_file:&str)->Result<bool>{
        let pkg_file=CString::new(pkg_file).unwrap();
        let mode=CString::new("r.fdio").unwrap();
        let ts=unsafe{rpmtsCreate()};
        if ts.is_null() {
            bail!(ERROR_RDNF_RPMTS_CREATE_FAILED);
        }
        unsafe{rpmtsSetVSFlags(ts, RPMVSF_MASK_NOSIGNATURES)};
        let td=unsafe{rpmtdNew()};
        if td.is_null() {
            bail!(ERROR_RDNF_RPMTD_CREATE_FAILED);
        }
        let fd=unsafe{Fopen(pkg_file.as_ptr(), mode.as_ptr())};
        let b=unsafe{
            let mut header=0 as Header;
            rpmReadPackageFile(ts, fd, pkg_file.as_ptr(), &mut header);
            if headerConvert(header, headerConvOps_e_HEADERCONV_RETROFIT_V3 as i32)==0{
                bail!(ERROR_RDNF_RPM_GET_RSAHEADER_FAILED);
            };
            if headerGet(header, rpmTag_e_RPMTAG_RSAHEADER, td, headerGetFlags_e_HEADERGET_MINMEM)==0{
                bail!(ERROR_RDNF_RPM_GET_RSAHEADER_FAILED);
            };
            let digest=pgpNewDig();
            if pgpPrtPkts((*td).data as *const u8, (*td).count as u64, digest, 0) !=0{
                bail!(ERROR_RDNF_RPM_GPG_PARSE_FAILED);
            }
            let b=rpmKeyringLookup(key_ring, digest)==rpmRC_e_RPMRC_OK;
            if !digest.is_null() {
                pgpFreeDig(digest);
            }
            if !header.is_null() {
                headerFree(header);
            }
            b
        };
        unsafe{
            if !fd.is_null() {
                Fclose(fd);
            }
            if !td.is_null() {
                rpmtdFree(td);
            }
            if !ts.is_null() {
                rpmtsFree(ts);
            }
        }
        Ok(b)
}