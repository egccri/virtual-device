# virtual-device

### Quick start

1. Write your `DeviceProfile` as config file(config/profile.json).
2. Write your `DeviceList` and `VirtualResource` as config file(config/device.json), `VirtualResource` is referred to device resources which define in `DeviceProfile`
3. Install a kafka broker and create topic(default `test`), config them in the `kafka_reporter`.
4. Run `main`

### Understand how it worked

1. `DeviceProfile` define a kind of device's resource, resource has value attr.
2. `DeviceList` define device's attr and virtual resource.
3. `VirtualResource` is `{profile_name}:{resource_name}:VirtualResource`, eg: `crane:current:ResourceFloat`.
4. Every device create a `VirtualDeviceTwin` with `VirtualResource`, then spawn `VirtualDeviceTwin` to a thread to generate `Values` and reporter them.

### Features

1. Rich virtual resource: random, line etc.
2. Rich reporter with sequence: kafka and mqtt, kafka use one common topic and split one device to one partition by `device_id` key,
mqtt sequence with one device one topic which topic name is `topic_prefix/device_id`.

### Supported virtual resource

+ resource_float : random f32 between maximum and minimum
+ resource_int: random i32 between maximum and minimum
+ one_of_list: config a type list, get one of them


### Example

```
"{\"device_id\":\"001\",\"event_time\":\"1650793385418\",\"current\":\"1.5\",\"voltage\":\"9\"}"
"{\"device_id\":\"002\",\"voltage\":\"7\",\"event_time\":\"1650793385418\",\"current\":\"1.5\"}"
Successfully produced record to topic test partition [0] @ offset 436
Successfully produced record to topic test partition [0] @ offset 437
"{\"device_id\":\"001\",\"event_time\":\"1650793385418\",\"current\":\"1.5\",\"voltage\":\"3\"}"
"{\"device_id\":\"002\",\"voltage\":\"0\",\"event_time\":\"1650793385418\",\"current\":\"1.5\"}"
Successfully produced record to topic test partition [0] @ offset 438
Successfully produced record to topic test partition [0] @ offset 439
"{\"device_id\":\"001\",\"event_time\":\"1650793385418\",\"current\":\"1.5\",\"voltage\":\"0\"}"
"{\"device_id\":\"002\",\"voltage\":\"0\",\"event_time\":\"1650793385418\",\"current\":\"1.5\"}"
Successfully produced record to topic test partition [0] @ offset 440
Successfully produced record to topic test partition [0] @ offset 441
"{\"device_id\":\"001\",\"event_time\":\"1650793385418\",\"current\":\"1.5\",\"voltage\":\"4\"}"
"{\"device_id\":\"002\",\"voltage\":\"0\",\"event_time\":\"1650793385418\",\"current\":\"1.5\"}"
Successfully produced record to topic test partition [0] @ offset 442
Successfully produced record to topic test partition [0] @ offset 443
```
