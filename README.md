# possum 

Possums are just little angry buddies that would rather fart and play dead than
put up a fight. They're extremely resourceful, making home where ever food,
water and shelter is available. And they're immune to rabies because they're
too cold inside. There's a lot to relate with a possum, especially when it
comes to working with github actions.

## Features

- [ ] parse and lint workflows and actions
    - [ ] support all triggers
    - [ ] detect typo'd names in places like needs
    - [ ] lint gha expressions
    - [ ] detect if too many workflows are involved in a call
    - find bad arguments for:
        - [ ] workflow_call invocations
        - [ ] action invocation
    - to the best of ability, vet the existence of:
        - [ ] secrets
        - [ ] configuration
        - [ ] runner label
        - [ ] environment
        - [ ] container image
- test scenarios like:
    - [ ] what if an error happens in this given step
    - [ ] what if a cancellation happens in this given step
    - [ ] what does my concurrency group actually cancel or queue
- analyze actions via:
    - [ ] cli usage:
        - [ ] `possum rummage -e pull_request -e push -e workflow_dispatch path/to/dir`
        - [ ] `possum rummage --tag tests path/to/dir`
    - [ ] web ui: `possum rummage serve path/to/dir`
- [ ] create an ancillary metadata file to store things like tags, etc
- works on:
    - [x] directory: `path/to/repo` -- automatically discovers `.github/workflows`
    - [ ] single file: `path/to/workflow.yml`
    - [ ] stdin
- [ ] lsp
