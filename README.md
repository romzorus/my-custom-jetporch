# Context

On 23th december 2023, Michael DeHaan sent a newsletter ("Discontinuing Jet") in which he announced that he has decided to not work on Jet any more. This repository has been created by me as a way to continue to explore Rust and learn from this great project, even if it doesn't lead anywhere. It was forked from [here](https://github.com/jetporch/jetporch) and renamed **my-custom-jetporch** in order to avoid confusion with the original project. Even if the basecode is 100% jetporch at the beginning, I want to make it evolve without restriction, merge branches...etc in an opinionated way, which can't be done with only a fork.

# Evolutions so far

## Already done
* Focus on testing (`cargo test`) in the `testing` branch
    - Unit tests added for `src/cli/show.rs`
    - Integration tests added for UNSET and SHOW-INVENTORY CLI modes

## In progress
* Still focusing on tests : once we have a strong set of automated tests, we can begin to add/improve functionalities and modules while making sure this doesn't break the rest.

## Plans for the future


**Feel free to send comments and contributions if you feel like it !**


# Jetporch - the Jet Enterprise Professional Orchestrator

Jetporch (aka Jet) is a general-purpose, community-driven IT automation platform for configuration management, 
deployment, orchestration, patching, and arbitrary task execution workflows. 

Jet is a GPLv3 licensed project, created and run by [Michael DeHaan](https://home.laserllama.net). [(<michael@michaeldehaan.net>)](mailto:michael@michaeldehaan.net).

