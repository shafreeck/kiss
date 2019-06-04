# KISS: Keeping In-memory Structure Storage

Kiss is a memory database inspired by [redis](https://github.com/antirez/redis) and [titan](https://github.com/distributedio/titan), this project is in super early stage. Nothing has been completed except the ideas which I think having a promising future.

## Ideas

* Memory is the storage and disk is the backup
* Incrementally backup to disk
* Memory with replicas for availability
* Disk with replicas for durability
* Parallel by using multiple cores, and shared by communication.
* Consensus with raft, nomal thread applying to memory, lazy thread applying to disk
