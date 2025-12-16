push:
    #!/usr/bin/env bash
    set -e
    git switch main
    git push origin
    git push github
    git switch embassy-git
    git merge main
    git push origin
    git push github
    git switch main
