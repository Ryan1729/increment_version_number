# increment_version_number

This is a simple program to increment a version number in a text file.

## example pre-push hook

One use of `increment_version_number` is to automate incrementing the version number on pushing to a git repo. Here is an example `pre-push` hook for that purpose. 

save the following as a text file called `pre-push` and put it in your `.git/hooks` directory.

Change `Cargo.toml` to the path of the file you want to update the version in.

```bash
#!/bin/bash

errors=0

version_file = "Cargo.toml"

if result=$(increment_version_number $version_file '(version = "\d+\.\d+\.)(\d+)' 2); then
	 echo -e "$result"

	 git add $version_file

	 #separate commit
	 #git commit -m'increment version'

	 #fold increment into previous commit
	 git commit --amend --no-edit
else
	 echo "$result"
	 errors=1
fi


if [ "$errors" != 0 ]; then
	echo "Failed"
	exit 1
else
	echo "OK"
fi

```

Another example that assumes you are using Rust and want to make sure `cargo test` passes before commiting is included in this folder as [pre-push](pre-push).
