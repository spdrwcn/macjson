## macjson

 - 从`redis`获取所有键值对并写入`json`文件内

### value格式必须为`json`格式，如下：

```
{
    "bluetooth_mac": "60:A5:E2:43:BE:48",
    "wired_mac": "04:BF:1B:65:ED:9A",
    "wireless_mac": "60:A5:E2:43:BE:44"
}
```

### 例如`key`为`BPB4BX3`，`value`为以上，则写入后的文件为以下：

```
{
  "BPB4BX3": {
    "bluetooth_mac": "60:A5:E2:43:BE:48",
    "wired_mac": "04:BF:1B:65:ED:9A",
    "wireless_mac": "60:A5:E2:43:BE:44"
  }
}
```

 - 多个如下:

```
{
  "a1": {
    "bluetooth_mac": "60:A5:E2:43:BE:48",
    "wired_mac": "04:BF:1B:65:ED:9A",
    "wireless_mac": "60:A5:E2:43:BE:44"
  },
  "a2": {
    "bluetooth_mac": "60:A5:E2:43:BE:48",
    "wired_mac": "04:BF:1B:65:ED:9A",
    "wireless_mac": "60:A5:E2:43:BE:44"
  },
  "BPB4BX3": {
    "bluetooth_mac": "60:A5:E2:43:BE:48",
    "wired_mac": "04:BF:1B:65:ED:9A",
    "wireless_mac": "60:A5:E2:43:BE:44"
  }
}
```

### Execl表格可直接导入

