# pass-git-credential-helper

## Overview

[git credential helper](https://git-scm.com/docs/gitcredentials) that gets the password from a [pass](https://www.passwordstore.org/) storage

pass uses gpg under the hood

Only the password retrieval is supported, i.e, the **get** operation.

**store**, **erase** are not supported

## Example usage

Configure git to use the helper for a specific url like https://github.com

```
[credential "https://github.com"]
       username = clidev
       helper = /usr/local/bin/pass-git-credential-helper
```

It will look for a password named **gitlab.com/clidev**

Only the hostname is used (not the path)

You can override the user by adding the -u option like in the following:

```
[credential "https://github.com"]
       username = clidev
       helper = /usr/local/bin/pass-git-credential-helper -u secdev
```

It will look for a password named **gitlab.com/secdev**

Or you can set the password name entirely by using the -p option:

```
[credential "https://github.com"]
       username = clidev
       helper = /usr/local/bin/pass-git-credential-helper -p secdev
```

It will look for a password named **secdev**

## Limitations

Since the store command is not supported currently an error message is displayed to stderr about that

```
Error: Unsupported operation [store]
```
