The utility will send message with all open merge requests in the given project id to the given slack channel.

# Build

```
cargo build --release;
```

# Usage

```
USAGE:
    slack-mr-reminder <gitlab-project-id> <gitlab-token> <slack-hook-credentials> [gitlab-host]
```