# Evolutions so far
## Already done
* Focus on testing (`cargo test`) in the `testing` branch
    - Unit tests added for `src/cli/show.rs`
    - Integration tests added for CLI modes UNSET, SHOW-INVENTORY, LOCAL, CHECK-LOCAL, SSH, and CHECK-SSH
    - Automated integration tests on a Docker lab : integration tests are made, through ssh, on Docker containers based on the main Linux distributions. To test this way, use the following command : `cargo testdocker`

## In progress
* Still focusing on tests : *once we have a strong set of automated tests, we can begin to add/improve functionalities and modules while making sure this doesn't break the rest.*


## Plans for the future

# Testing the app with Docker

The goal here is to test each module on the main Linux distributions in a lab based on Docker containers. For each commit/update/fix, the `cargo testdocker` needs to succeed. Otherwise, it means what we did just broke something elsewhere.

Here is an illustration of how it is done so far
![Testing with Docker](/tests/test-illustration.png)

Before running `cargo testdocker`, you need to install [docker](https://docs.docker.com/get-docker/). Also, to avoid permission issues, please add your user to the `docker` group in order to use the `docker` command without root privileges. This can be done with this command : `sudo usermod -aG docker $USER`


**Feel free to send comments and contributions if you feel like it !**

# Context

On 23th december 2023, Michael DeHaan sent a newsletter ("Discontinuing Jet") in which he announced that he has decided to not work on Jet any more. This repository has been created by me as a way to continue to explore Rust and learn from this great project, even if it doesn't lead anywhere. It was forked from [here](https://github.com/jetporch/jetporch) and renamed **my-custom-jetporch** in order to avoid confusion with the original project. Even if the basecode is 100% jetporch at the beginning, I want to make it evolve without restriction, merge branches...etc in an opinionated way, which can't be done with only a fork.

# Jetporch - the Jet Enterprise Professional Orchestrator

Jetporch (aka Jet) is a general-purpose, community-driven IT automation platform for configuration management, 
deployment, orchestration, patching, and arbitrary task execution workflows. 

Jet is a GPLv3 licensed project, created and run by [Michael DeHaan](https://home.laserllama.net). [(<michael@michaeldehaan.net>)](mailto:michael@michaeldehaan.net).

