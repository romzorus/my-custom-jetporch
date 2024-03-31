***Now focusing on another project : [Dux](https://gitlab.com/dux-tool/dux)***

***But feel free to send comments/requests/contributions. I can still work on this if needed !***


# Automation tool written in Rust : jetp

This project is a fork of the [jetporch](https://github.com/jetporch/jetporch) project.

**Feel free to send comments/requests/contributions !**

# Core modules

These modules are included in the jetp binary. They can be used directly.

### access
| Module | Status | Description |
|------|-------|-------------|
| `group` | (TBC) | Manage groups on the remote host |
| `user` | (TBC) | Manage users on the remote host |

### commands
| Module | Status | Description |
|------|-------|-------------|
| `external` | (TBC) | Use an external module written in Python |
| `script` | (TBD) | Push and execute a custom script on the remote host |
| `shell` | ready | Execute a custom command on the remote host |

### control
| Module | Status | Description |
|------|-------|-------------|
| `assert` | ready | Test a condition and make the playbook fail in case of false return |
| `debug` | ready | Display everything for debug purposes |
| `echo` | ready | Display a custom message in jet output |
| `facts` | ready | Gather facts about the remote host OS |
| `fail` | ready | Make the playbook fail with a custom error message |
| `set` | (TBC) | Set variables conditionally |

### files
| Module | Status | Description |
|------|-------|-------------|
| `archive` | (TBD) | Create and extract archives on the remote host |
| `copy` | ready | Copy a file from the local machine to the remote host |
| `directory` | ready | Manage directories in remote host (create, delete) |
| `fetch` | ready | Fetch a file or a folder from the remote host to the local machine |
| `file` | ready | Manage files in remote host (create, delete) |
| `git` | ready | Handle a git repository from the remote host (clone...etc) |
| `stat` | ready | Retrieve the permissions of a remote file in 'chmod numbers' format |

### packages
| Module | Status | Description |
|------|-------|-------------|
| `apt` | ready | Manage packages in Debian-like distributions |
| `homebrew` | (TBC) | Manage packages in OS X remote hosts |
| `pacman` | ready | Manage packages in Arch Linux distributions |
| `yum_dnf` | ready | Manage packages in Fedora-like distributions |
| `zypper` | ready | Manage packages in OpenSuse distributions |

### services
| Module | Status | Description |
|------|-------|-------------|
| `sd_service` | (TBC) | Manage systemd services on remote host |

*TBD : to be done*

*TBC : to be checked/tested*


# Testing the app with Docker

The goal here is to test each module on the main Linux distributions in a Docker-based local lab. When running `cargo test`, each test has its own set of containers and can be tested in it.

*How it is done so far*
![Testing with Docker](/tests/test-illustration.png)

Before running `cargo test`, you need to install [docker](https://docs.docker.com/get-docker/). Also, to avoid permission issues, please add your user to the `docker` group in order to use the `docker` command without root privileges. This can be done with this command : `sudo usermod -aG docker $USER`.

Because `cargo` runs tests in parallel, you might encounter memory issues because lots of containers will be running at the same time. To avoid this, you can limit the number of tests running at the same time by limiting the number of threads : `cargo test -- --test-threads=4` means you will only have a maximum of 4 sets of containers running at any given moment. The total memory required by one set of containers can vary from 15MB to 150MB or even more, depending on what you are doing with it.

**If you have limited ressources, it is recommended to use this command : `cargo testdocker`, which is an alias for `cargo test --no-fail-fast -- --test-threads=1`. Otherwise, just run `cargo test`.**

# Context
Jet is a GPLv3 licensed project, created and run by Michael DeHaan. [(<michael@michaeldehaan.net>)](mailto:michael@michaeldehaan.net).

On 23th december 2023, Michael DeHaan sent a newsletter ("Discontinuing Jet") in which he announced that he has decided to not work on Jet any more. This repository has been created by me as a way to continue to explore Rust and learn from this great project, even if it doesn't lead anywhere. It was forked from [here](https://github.com/jetporch/jetporch) and renamed **my-custom-jetporch** in order to avoid confusion with the original project. Even if the basecode is 100% jetporch at the beginning, I want to make it evolve without restriction, merge branches...etc in an opinionated way, which can't be done with only a fork.
