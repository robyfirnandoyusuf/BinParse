# BinParse
Simple Recycle Bin Forensic Tool Written with ❤ Rust 


### To Do List :
- [X] Read file content
- [X] Read file content in Bytes format
- [X] Table and Banner
- [X] Display Index
- [X] Display Deletion Time
- [X] Display Filesize
- [X] Display Version
- [X] Display Ori Path
- [ ] Filter Feature
- [ ] Restore Feature
- [ ] Generate Report
- [ ] Refactor to more pretty code :pensive:

### Usage
```sh
$ cargo run -- --dir='./examples/$RECYCLE.BIN/S-1-5-21-4144826732-2003267707-115468498-1001'
```
### Preview
```
 ____  _       _____
|  _ \(_)     |  __ \
| |_) |_ _ __ | |__) |_ _ _ __ ___  ___ _ __
|  _ <| | '_ \|  ___/ _` | '__/ __|/ _ \ '__|
| |_) | | | | | |  | (_| | |  \__ \  __/ |
|____/|_|_| |_|_|   \__,_|_|  |___/\___|_|
                                            ♥ Rust
    ----- Built with ♥ by greycat -----

+---------------+---------------------+-----------+-----------+-------------------------------------------------------------------------------+
| Index         | Deletion Time       | File Size | Version   | Original Path                                                                 |
+---------------+---------------------+-----------+-----------+-------------------------------------------------------------------------------+
| $I0JGHX7      | 2007-09-21 06:47:49 | 0 B       | Win Vista | C:\Users\student\Desktop\New Folder 1                                         |
+---------------+---------------------+-----------+-----------+-------------------------------------------------------------------------------+
| $I1IS2OK.txt  | 2007-09-21 06:48:13 | 0 B       | Win Vista | C:\Users\student\Desktop\New Text Document blah.txt                           |
+---------------+---------------------+-----------+-----------+-------------------------------------------------------------------------------+
| $I95CUKU      | 2007-09-21 08:02:59 | 4 KiB     | Win Vista | C:\Users\student\Downloads\fau-1.3.0.2355(rc3)\fau\FAU.x86\sparsefile         |
+---------------+---------------------+-----------+-----------+-------------------------------------------------------------------------------+
| $IEW83YF.txt  | 2020-11-30 22:12:27 | 30 B      | Win 10    | D:\samples\test.txt                                                           |
+---------------+---------------------+-----------+-----------+-------------------------------------------------------------------------------+
| $IHMU3NR.zip  | 2007-09-21 08:17:19 | 4.793 MiB | Win Vista | C:\Users\student\Downloads\fau-1.3.0.2355(rc3).zip                            |
+---------------+---------------------+-----------+-----------+-------------------------------------------------------------------------------+
| $IUVFB0M.rtf  | 2007-09-21 06:32:46 | 155 B     | Win Vista | C:\Users\student\Desktop\New Rich Text Document.rtf                           |
+---------------+---------------------+-----------+-----------+-------------------------------------------------------------------------------+
| $IZUFRX4.vmdk | 2007-09-21 09:22:25 | 10 GiB    | Win Vista | C:\Virtual Machines\Windows XP Professional\Windows XP Professional-flat.vmdk |
+---------------+---------------------+-----------+-----------+-------------------------------------------------------------------------------+
```

### Inspired By
- [$IParser](https://df-stream.com/recycle-bin-i-parser/) 
- [TrashParse](https://github.com/hanasuru/TrashParse/)
