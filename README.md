# rust_native_storage_library
A library which compiles to `.so` and `.dylib` and facilitates the native storage (and retrieval) of key:value pairs on the host system

# Deployment
To use this software, please compile into libraries (for which SecondState WebAssembly runtime software can interoperate). These libraries will reside on the same machine as SSVM suite. Here is an example of how to set up a host machine (in this case, an EC2 instance with sufficient storage to facilitate SSVM and a database etc.).

# Setting up a machine and OS
This installation uses the AWS i3.xlarge instance because it has a solid state hard drive which is large enough
- i3.xlarge ($0.374 per Hour)
- 1 x 950 NVMe SSD Storage
- 4 CPU
- 30.5 GB RAM

# Software (OS)
```
sudo apt-get update
sudo apt-get -y upgrade
sudo apt-get install -y build-essential
```

# Auxillary hardware
View NVMe volume (which is not yet mounted/mapped/formatted)
```bash
lsblk
```
There is a ~300Gb drive with the name of nvme0n1
```bash
nvme0n1     259:2    0 279.4G  0 disk 
```
Create a file system
```bash
sudo mkfs -t ext4 /dev/nvme0n1 
```
Part of the output from the above mkfs command will include the Filesystem UUID. Cut and paste this UUID because it will be used in an upcoming command.
```bash
Filesystem UUID: 6f6177fe-947a-46a2-b593-c36dfaaa8407
```
Create an easily accesible mount point on the main drive (where the operating system runs) and then set the permissions of this mount point to the ubuntu user.
```bash
sudo mkdir /media/nvme
sudo chown -R ubuntu:ubuntu /media/nvme/
```
Ensure that this drive is mounted each time the system is restarted. Add this line to the */etc/fstab* file (remember the UUID from the previous step?).
```bash
UUID=553acce1-4e42-4ea3-85da-c57ed3dd82b5 /media/nvme ext4 defaults 0 0
```
Once the above commands have succeeded, reboot the instance.
```bash
sudo shutdown -r now
```
After the reboot, see the mounted ~300Gb NVMe SSD using the df command
```bash
df -h
/dev/nvme0n1    275G   65M  260G   1% /media/nvme
```
```bash
#ensure that the /media/nvme directory is owned by ubuntu by typing ls -la /media/nvme If it is not then type the following command
sudo chown -R ubuntu:ubuntu /media/nvme/
```

# Installing database (RocksDB)
Create directory for database
```bash
mkdir /media/nvme/ssvm_database
```

Install system requirements
```
sudo apt-get -y install devscripts debhelper build-essential fakeroot zlib1g-dev libbz2-dev libsnappy-dev libgflags-dev libzstd-dev
```
Install RocksDB
```
git clone https://github.com/ulikoehler/deb-buildscripts.git
cd deb-buildscripts
./deb-rocksdb.py
```
Configure RocksDB - this (setting the data dir and options) is all done in the source code, as per the example below. There is nothing to do here ... please move to the next section.
```
let path = "/media/nvme/ssvm_database";
let db = DB::open_default(path).unwrap();
let mut opts = Options::default();
opts.increase_parallelism(3);
opts.create_if_missing(true);
```

# Compiling rust_native_storage_library to system level executables

# Testing the executables