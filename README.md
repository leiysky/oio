# oio

## Warning: This project is still in early stage, you should use it very carefully

Simple benchmark tool for object storage powered by [Apache OpenDAL](https://opendal.apache.org/).

## Usage

Build with:
```shell
$ cargo build --release
```

Run with:
```
$ target/release/oio [config_file]
```

## Configuration

`[service]` parameters:
| Parameter            | Type             | Description                                                  |
| -------------------- | ---------------- | ------------------------------------------------------------ |
| type                 | string: required | Type of storage services, one of: "s3", "oss", "minio", "fs" |
| endpoint             | string: required | Endpoint of the storage service, e.g. "s3.aws.amazon.com"    |
| region               | string: rqeuired | Region of the storage service, e.g. "us-east-1"              |
| bucket               | string: required | Bucket name, e.g. "my-bucket"                                |
| access_key           | string: required | Access key, e.g. "AKIAIOSFODNN7EXAMPLE"                      |
| secret_key           | string: required | Secret key, e.g. "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"  |
| prefix               | string: optional | Prefix of the object, e.g. "my-prefix"                       |
| virtual_hosted_style | bool: optional   | Enable virtual-hosted-style request, false by default        |

`[job]` parameters:
| Parameter | Type             | Description                                       |
| --------- | ---------------- | ------------------------------------------------- |
| workload  | string: required | Workload type, one of: "download", "upload"       |
| num_jobs  | int: optional    | Number of jobs executed in parallel, 1 by default |
| file_size | int: required    | Size of each file in bytes                        |
| run_time  | int: required    | Time to run the jobs, e.g. "1s", "1m"             |
