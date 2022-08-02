# quill replace-node-provider-id

Signs a message to replace Node Provide ID in targeted Node Operator Record.

## Basic usage

The basic syntax for running `quill replace-node-provider-id` commands is:

``` bash
quill replace-node-provider-id --node-operator-id <NODE_OPERATOR_ID> --node-provider-id <NODE_PROVIDER_ID>
```

## Flags

| Flag                 | Description                                     |
|----------------------|-------------------------------------------------|
| `-h`, `--help`       | Displays usage information.                     |

## Options

| Option | Description |
|----------|-------------|
| `--node-operator-id <NODE_OPERATOR_ID>` | The Principal id of the node operator. This principal is the entity that is able to add and remove nodes. |
| `--node-provider-id <NODE_PROVIDER_ID>` | The new Principal id of the node provider. |

