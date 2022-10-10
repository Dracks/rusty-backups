# Rusty-backups
Is a tool designed to make backups easy

## Current features

* Connect to a machine using ssh, and execute a command, to get the output into a local file

## Todo

* Add tools to connect to Amazon S3, to upload backups as a glacier
* Add other ways to connect via SSH
* Auto cleanup of older files

## Sample configuration
you need to create a config.yaml in the place you will execute (This is temporal)
```yaml
- hostname: server-to-backup:22
  username: username
  private_key: /Users/jsingla/.ssh/home
  output_path: /media/backups/
  commands: 
    - exec: ls /
      output: mrscrooge/list_root_<date>.out
    - exec: ls
      output: list_<date>.out
```