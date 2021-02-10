# Rust Native Storage Library

## Fetch
```
cd ~
git clone https://github.com/second-state/rust_native_storage_library.git
```
## Build
```
cd ~/rust_native_storage_library
make
```
## Output
Creates an executable dynamic library at the following location
```
~/rust_native_storage_library/target/debug/librust_native_storage_library.so
```
Compiles and links the C++ code (`main.cpp`), the header file `test_lib.h` and the Rust code `lib.rs` and provides an executable called `run_test_demonstration`
## Run example C++ implementation (test demonstration)
```
./run_test_demonstration
```
This test demonstration produces the following output (stores and loads from RocksDB)
```
Starting main function ...
i64 variable as integer: 1234567111
i64 variable as string: 1234567111
i64 variable as char: 1234567111
i32 variable as integer: 1234567111
i32 variable as string: 1234567111
i32 variable as char: 1234567111
Calling store data ... 
Storing data, please wait ...
Database path: "/media/nvme/ssvm_database"
Database instance: RocksDB { path: "/media/nvme/ssvm_database" }
Item added to database
Calling load data ... 1234567111
Loading data, please wait ...
Retrieved the following string: 1234567111
```

# Building a machine to run this library

The following is a list of tasks which will result in you having a machine which is fully equiped to deploy this storage library application.

# Creating a machine on which to deploy this (and the SSVM runtime on)
The following included information about creating a new machine, additional storage, RockDB database installation and more ...

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
sudo chown -R $USER:$USER /media/nvme/
```

# Installing database (RocksDB)
Create directory for database
```bash
mkdir /media/nvme/ssvm_database
sudo chown -R $USER:$USER /media/nvme/ssvm_database
```

Install system requirements
```
sudo apt-get -y install devscripts debhelper build-essential fakeroot zlib1g-dev libbz2-dev libsnappy-dev libgflags-dev libzstd-dev make clang pkg-config libssl-dev
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

