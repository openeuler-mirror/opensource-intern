use crate::conf::{Credential, Remote};
use git2::{BranchType, Cred, CredentialType, Error, FetchOptions, PushOptions, RemoteCallbacks, Repository};
use std::fs::{remove_dir_all, remove_file};
use std::{env, path::Path};

/// Return `RemoteCallbacks` based on authentication type
fn remote_callbacks(remote: &Remote) -> Result<RemoteCallbacks, Error> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, allowed_types| {
        let username = if let Some(name) = &remote.username {
            name
        } else {
            username_from_url.unwrap()
        };
        if let Some(cred) = &remote.cred {
            return match cred {
                Credential::SshKey(key) => Cred::ssh_key(username, None, Path::new(key), None),
                Credential::Password(passwd) => Cred::userpass_plaintext(username, passwd),
            };
        } else if allowed_types.contains(CredentialType::SSH_KEY) {
            // ref: https://github.com/martinvonz/jj/blob/main/lib/src/git.rs
            if env::var("SSH_AUTH_SOCK").is_ok() || env::var("SSH_AGENT_PID").is_ok() {
                return Cred::ssh_key_from_agent(username);
            } else if let Ok(home_dir) = env::var("HOME") {
                let key_path = Path::new(&home_dir).join(".ssh").join("id_rsa");
                if key_path.is_file() {
                    return Cred::ssh_key(username, None, &key_path, None);
                }
            }
        }
        Cred::default()
    });
    Ok(callbacks)
}

/// Return `FetchOptions`
fn fetch_options(remote: &Remote) -> Result<FetchOptions, Error> {
    let callbacks = remote_callbacks(remote)?;
    let mut fetch_option = FetchOptions::new();
    fetch_option.remote_callbacks(callbacks);
    Ok(fetch_option)
}

/// Return `PushOptions`
fn push_options(remote: &Remote) -> Result<PushOptions, Error> {
    let callbacks = remote_callbacks(remote)?;
    let mut push_option = PushOptions::new();
    push_option.remote_callbacks(callbacks);
    Ok(push_option)
}

/// clone repository
pub fn clone(source: &Remote) -> Result<Repository, Error> {
    let fo = fetch_options(source)?;

    // prepare builder
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    let path = Path::new("/tmp").join("git-sync").join("source");

    if path.exists() {
        if path.is_dir() {
            remove_dir_all(path.as_path())
                .map_err(|err| Error::from_str(err.to_string().as_str()))?;
        } else {
            remove_file(path.as_path()).map_err(|err| Error::from_str(err.to_string().as_str()))?;
        }
    }

    // clone the project
    let repo = builder.clone(&source.url, path.as_path())?;

    Ok(repo)
}

/// push commits of source to target
pub fn push_to_target(repo: &Repository, target: &Remote) -> Result<(), Error> {
    // add target remote
    let mut remote = repo.remote("target", &target.url)?;
    // add refspecs (but wildcard not supported by libgit2)
    repo.remote_add_push("target", "+refs/remotes/origin/*:refs/heads/*")?;
    repo.remote_add_push("target", "+refs/tags/*:refs/tags/*")?;

    let mut refspecs: Vec<String> = vec![];

    // get remote origin branches
    let branches = repo.branches(Some(BranchType::Remote))?;
    // add refspecs
    for b in branches.flatten() {
        let name = b.0.name();
        if let Ok(name) = name {
            if let Some(name) = name {
                let branch = &name[name.find('/').unwrap()..];
                println!("sync branch {} to target repo", name);
                refspecs.push(format!("+refs/remotes/{}:refs/heads{}", name, branch));
            }
        }
    }

    // get all tags
    let tags = repo.tag_names(None)?;
    // add refspecs
    for tag in tags.iter().flatten() {
        println!("sync tag {} to target repo", tag);
        refspecs.push(format!("+refs/tags/{}:refs/tags/{}", tag, tag));
    }

    let mut po = push_options(target)?;
    remote.push(&refspecs, Some(&mut po))?;
    Ok(())
}
